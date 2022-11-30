use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::reader::Reader;
use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use tera::{Context, Map, Number, Tera, Value};

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

#[derive(Debug)]
struct XmlContext {
    codes: Map<String, Value>,
    all_ids: Vec<String>,
    active_file_date: String,
    historical_file_date: String,
}

const TYPE_NAME: &str = "CurrencyCode";

const INPUT_DIR: &str = "data";

const TEMPLATE_DIR: &str = "templates";

fn main() -> Result<(), Box<dyn Error>> {
    let render = make_renderer("lib._rs".to_string(), "generated.rs".to_string());
    init_context()
        .and_then(|ctx| process_iso_list_xml(ctx, "list-one.xml", false))
        .and_then(|ctx| process_iso_list_xml(ctx, "list-three.xml", true))
        .and_then(finalize_context)
        .and_then(render)
}

fn init_context() -> Result<XmlContext, Box<dyn Error>> {
    Ok(XmlContext {
        codes: Default::default(),
        all_ids: Default::default(),
        active_file_date: Default::default(),
        historical_file_date: Default::default(),
    })
}

fn finalize_context(mut xml_ctx: XmlContext) -> Result<Context, Box<dyn Error>> {
    let mut ctx = Context::new();

    ctx.insert(
        "type_name".to_string(),
        &Value::String(TYPE_NAME.to_string()),
    );

    ctx.insert("active_file_date", &Value::String(xml_ctx.active_file_date));
    ctx.insert(
        "historical_file_date",
        &Value::String(xml_ctx.historical_file_date),
    );

    ctx.insert("codes", &Value::Object(xml_ctx.codes));

    xml_ctx.all_ids.sort();
    xml_ctx.all_ids.dedup();
    ctx.insert(
        "all_ids",
        &Value::Array(xml_ctx.all_ids.into_iter().map(Value::String).collect()),
    );

    Ok(ctx)
}

fn process_iso_list_xml(
    mut xml_ctx: XmlContext,
    file_name: &str,
    is_historical: bool,
) -> Result<XmlContext, Box<dyn Error>> {
    let file_name = format!("{}/{}", INPUT_DIR, file_name);
    let mut reader = Reader::from_file(&file_name)?;
    reader.trim_text(true);

    println!("cargo:rerun-if-changed={}", file_name);

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
                                    xml_ctx.historical_file_date =
                                        String::from_utf8(attribute.value.to_vec())?;
                                } else {
                                    xml_ctx.active_file_date =
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
                            xml_ctx.all_ids.push(text.clone());
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
                            xml_ctx
                                .codes
                                .insert(alpha_code.clone(), Value::Object(entry.clone()));
                        }
                        break;
                    }
                }
                Ok(Event::Eof) => {
                    return Ok(xml_ctx);
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

fn make_renderer(
    template_name: String,
    generated_file_name: String,
) -> impl Fn(Context) -> Result<(), Box<dyn Error>> {
    let render_template_to_file = move |ctx: Context| -> Result<(), Box<dyn Error>> {
        let output_dir: String = env::var("OUT_DIR").unwrap();
        let file_name = Path::new(&output_dir).join(&generated_file_name);
        let file = File::create(&file_name)?;

        let tera = Tera::new(&format!("{}/*._rs", TEMPLATE_DIR))?;

        println!("cargo:rerun-if-changed={}/{}", TEMPLATE_DIR, template_name);

        tera.render_to(&template_name, &ctx, file)?;

        Ok(())
    };
    render_template_to_file
}

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
