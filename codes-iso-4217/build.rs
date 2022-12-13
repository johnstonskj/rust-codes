use codes_common::{
    default_finalize, default_init, input_file_name, make_default_renderer, process,
};
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::reader::Reader;
use std::error::Error;
use tera::{Context, Map, Number, Value};

#[derive(Debug)]
enum XmlState {
    Top,
    Entry,
    Historical,
    CountryName,
    CurrencyName,
    CurencyCode,
    CurrencyNumber,
    Units,
    WithdrawalDate,
}

#[derive(Debug, Default)]
struct Data {
    codes: Map<String, Value>,
    active_file_date: String,
    historical_file_date: String,
}

const TYPE_NAME: &str = "CurrencyCode";

fn main() -> Result<(), Box<dyn Error>> {
    process(
        default_init,
        |ctx| {
            process_iso_list_xml(ctx, "list-one.xml", false)
                .and_then(|ctx| process_iso_list_xml(ctx, "list-three.xml", true))
                .and_then(process_symbol_data)
        },
        default_finalize,
        make_default_renderer("lib._rs", "generated.rs"),
    )
}

fn process_iso_list_xml(
    mut data: Data,
    file_name: &str,
    is_historical: bool,
) -> Result<Data, Box<dyn Error>> {
    let file_name = input_file_name(file_name);

    let mut reader = Reader::from_file(&file_name)?;
    reader.trim_text(true);

    let is_historical_default = Value::Bool(is_historical);
    let is_fund_default = Value::Bool(false);

    let mut state: XmlState = XmlState::Top;
    let mut buffer = Vec::new();

    loop {
        let mut alpha_code = String::new();
        let mut entry = Map::new();
        entry.insert("is_historical".to_string(), is_historical_default.clone());
        entry.insert("is_fund".to_string(), is_fund_default.clone());
        loop {
            match reader.read_event_into(&mut buffer) {
                Ok(Event::Start(ref e)) => {
                    if e.name() == XmlState::Top.tag_name() {
                        for attribute in e.attributes() {
                            let attribute = attribute?;
                            if attribute.key == QName(b"Pblshd") {
                                if is_historical {
                                    data.historical_file_date =
                                        String::from_utf8(attribute.value.to_vec())?;
                                } else {
                                    data.active_file_date =
                                        String::from_utf8(attribute.value.to_vec())?;
                                }
                            }
                        }
                    } else if e.name() == XmlState::CurrencyName.tag_name() {
                        for attribute in e.attributes() {
                            let attribute = attribute?;
                            if attribute.key == QName(b"IsFund") {
                                entry.insert(
                                    "is_fund".to_string(),
                                    Value::Bool(
                                        String::from_utf8(attribute.value.to_vec())? == "true",
                                    ),
                                );
                            }
                        }
                        state = XmlState::CurrencyName;
                    } else if let Some(new_state) = XmlState::from_tag_name(e.name()) {
                        state = new_state;
                    }
                }
                Ok(Event::Text(e)) => {
                    let text = String::from_utf8(e.to_vec())?;
                    match state {
                        XmlState::CountryName => {
                            let text = text.replace('"', "\\\"");
                            entry.insert("country_name".to_string(), Value::String(text));
                        }
                        XmlState::CurrencyName => {
                            let text = text.replace('"', "\\\"");
                            entry.insert("currency_name".to_string(), Value::String(text));
                        }
                        XmlState::CurencyCode => {
                            alpha_code = text.clone();
                            entry.insert("alpha_code".to_string(), Value::String(text));
                        }
                        XmlState::CurrencyNumber => {
                            let number: Number = text.parse::<u16>()?.into();
                            entry.insert("numeric_code".to_string(), Value::Number(number));
                        }
                        XmlState::Units => {
                            if !text.is_empty() && text.chars().all(|c| c.is_ascii_digit()) {
                                let number: Number = text.parse::<u16>()?.into();
                                entry.insert("monetary_units".to_string(), Value::Number(number));
                            }
                        }
                        XmlState::WithdrawalDate => {
                            entry.insert("withdrawal_date".to_string(), Value::String(text));
                        }
                        _ => (),
                    };
                }
                Ok(Event::End(e)) => {
                    if e.name() == XmlState::Entry.tag_name()
                        || e.name() == XmlState::Historical.tag_name()
                    {
                        if !alpha_code.is_empty() {
                            data.codes
                                .insert(alpha_code.clone(), Value::Object(entry.clone()));
                        }
                        break;
                    }
                }
                Ok(Event::Eof) => {
                    return Ok(data);
                }
                Err(e) => {
                    eprintln!(
                        "Error at byte offset {}; error: {:?}",
                        reader.buffer_position(),
                        e
                    );
                    return Err(Box::new(e));
                }
                _ => (),
            }
            buffer.clear();
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl XmlState {
    pub fn tag_name(&self) -> QName {
        QName(match self {
            Self::Top => b"ISO_4217",
            Self::Entry => b"CcyNtry",
            Self::Historical => b"HstrcCcyNtry",
            Self::CountryName => b"CtryNm",
            Self::CurrencyName => b"CcyNm",
            Self::CurencyCode => b"Ccy",
            Self::CurrencyNumber => b"CcyNbr",
            Self::Units => b"CcyMnrUnts",
            Self::WithdrawalDate => b"WthdrwlDt",
        })
    }

    pub fn from_tag_name(name: QName<'_>) -> Option<Self> {
        match name {
            QName(b"ISO_4217") => Some(Self::Top),
            QName(b"CcyNtry") => Some(Self::Entry),
            QName(b"HstrcCcyNtry") => Some(Self::Historical),
            QName(b"CtryNm") => Some(Self::CountryName),
            QName(b"CcyNm") => Some(Self::CurrencyName),
            QName(b"Ccy") => Some(Self::CurencyCode),
            QName(b"CcyNbr") => Some(Self::CurrencyNumber),
            QName(b"CcyMnrUnts") => Some(Self::Units),
            QName(b"WthdrwlDt") => Some(Self::WithdrawalDate),
            _ => None,
        }
    }
}

impl From<Data> for Context {
    fn from(data: Data) -> Self {
        let mut ctx = Context::new();

        ctx.insert(
            "type_name".to_string(),
            &Value::String(TYPE_NAME.to_string()),
        );

        ctx.insert("active_file_date", &Value::String(data.active_file_date));

        ctx.insert(
            "historical_file_date",
            &Value::String(data.historical_file_date),
        );

        let mut all_ids: Vec<&String> = data.codes.keys().collect();

        all_ids.sort();
        all_ids.dedup();
        ctx.insert(
            "all_ids",
            &Value::Array(all_ids.into_iter().cloned().map(Value::String).collect()),
        );

        ctx.insert("codes", &Value::Object(data.codes));

        ctx
    }
}

fn process_symbol_data(mut data: Data) -> Result<Data, Box<dyn std::error::Error>> {
    use scraper::{Html, Selector};
    use std::fs;

    fn element_text(element: &scraper::element_ref::ElementRef) -> String {
        element
            .text()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("")
            .trim()
            .to_string()
    }

    let file_name = input_file_name("xe-symbols.html");
    let source = fs::read_to_string(&file_name)?;

    let document = Html::parse_document(&source);
    let row_selector = Selector::parse("li.eOCRRO").unwrap();
    let data_selector = Selector::parse("div.hawZDG").unwrap();

    for row in document.select(&row_selector) {
        let strings = row
            .select(&data_selector)
            .enumerate()
            .filter_map(|(i, e)| {
                if let 0..=11 = &i {
                    Some(element_text(&e))
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();
        assert_eq!(strings.len(), 6);

        let alpha_code = strings.get(1).unwrap();
        if alpha_code != "Currency Code" {
            if let Some(row) = data.codes.get_mut(alpha_code) {
                let row = row.as_object_mut().unwrap();

                row.insert(
                    "currency_string".into(),
                    strings.get(2).unwrap().to_string().into(),
                );

                let unicode: Vec<Value> = strings
                    .get(5)
                    .unwrap()
                    .split(',')
                    .map(|s| u32::from_str_radix(s.trim(), 16).unwrap().into())
                    .collect();

                row.insert("currency_code_points".into(), unicode.into());
            }
        }
    }

    Ok(data)
}
