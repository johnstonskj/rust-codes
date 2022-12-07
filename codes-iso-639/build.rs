use codes_common::{
    default_finalize_for, default_init, make_default_renderer, process, rerun_if_changed,
    Data as DataTrait, DEFAULT_DATA_DIR,
};
use std::collections::BTreeMap;
use std::fs::File;
use tera::{Map, Value};

#[derive(Debug, Default)]
struct Data {
    rows: BTreeMap<String, Map<String, Value>>,
    macros: Map<String, Value>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    process(
        default_init,
        process_part1_csv,
        default_finalize_for,
        make_default_renderer("part_1._rs", "part_1.rs"),
    )?;

    process(
        default_init,
        |data| process_part3_csv(data).and_then(process_part3_macro_csv),
        finalize_part3,
        make_default_renderer("part_3._rs", "part_3.rs"),
    )?;

    process(
        default_init,
        process_part5_csv,
        default_finalize_for,
        make_default_renderer("part_5._rs", "part_5.rs"),
    )?;

    Ok(())
}

fn process_part1_csv(mut data: Data) -> Result<Data, Box<dyn std::error::Error>> {
    let file_name = format!("{}/iso-639-1.tsv", DEFAULT_DATA_DIR);

    rerun_if_changed(&file_name);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .trim(csv::Trim::All)
        .from_reader(File::open(file_name)?);

    for result in rdr.records() {
        let record = result?;

        let mut row: Map<String, Value> = Default::default();

        let id = record.get(1).unwrap().to_string();
        row.insert("code".to_string(), Value::String(id.clone()));

        let names = record.get(2).unwrap().to_string();
        row.insert(
            "label".to_string(),
            Value::String(if names.contains('|') {
                let names = names.split('|');
                names
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join("; ")
            } else {
                names.to_string()
            }),
        );

        data.insert_row(&id, row);
    }

    Ok(data)
}

fn process_part3_csv(mut data: Data) -> Result<Data, Box<dyn std::error::Error>> {
    let file_name = format!("{}/iso-639-3.tsv", DEFAULT_DATA_DIR);

    rerun_if_changed(&file_name);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .trim(csv::Trim::All)
        .from_reader(File::open(file_name)?);

    for result in rdr.records() {
        let record = result?;

        let mut row: Map<String, Value> = Default::default();

        let id = record.get(0).unwrap().to_string();
        row.insert("code".to_string(), Value::String(id.clone()));

        let part_1_code = record.get(3).unwrap().to_string();
        if part_1_code != "sh" {
            // Serbo-Croatian - Code element for 639-1 has been deprecated
            row.insert("part_1_code".to_string(), Value::String(part_1_code));
        }

        row.insert(
            "scope".to_string(),
            Value::String(
                match record.get(4).unwrap() {
                    "C" => "Collection",
                    "D" => "Dialect",
                    "I" => "Individual",
                    "M" => "Macro",
                    "R" => "Reserved",
                    "S" => "Special",
                    _ => unreachable!(),
                }
                .to_string(),
            ),
        );

        row.insert(
            "language_type".to_string(),
            Value::String(
                match record.get(5).unwrap() {
                    "A" => "Ancient",
                    "C" => "Constructed",
                    "E" => "Extinct",
                    "H" => "Historic",
                    "L" => "Living",
                    "S" => "Special",
                    _ => unreachable!(),
                }
                .to_string(),
            ),
        );

        row.insert(
            "ref_name".to_string(),
            Value::String(record.get(6).unwrap().to_string()),
        );

        let comment = record.get(7).unwrap().to_string();
        if !comment.is_empty() {
            row.insert("comment".to_string(), Value::String(comment));
        }

        data.insert_row(&id, row);
    }

    Ok(data)
}

fn process_part3_macro_csv(mut data: Data) -> Result<Data, Box<dyn std::error::Error>> {
    let file_name = format!("{}/iso-639-3-macro-languages.tsv", DEFAULT_DATA_DIR);

    rerun_if_changed(&file_name);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .trim(csv::Trim::All)
        .from_reader(File::open(file_name)?);

    for result in rdr.records() {
        let record = result?;

        if record.get(2).unwrap() == "A" {
            let macro_code = record.get(0).unwrap();
            let individual_code = Value::String(record.get(1).unwrap().to_string());
            if let Some(Value::Array(macro_langs)) = data.macros.get_mut(macro_code) {
                macro_langs.push(individual_code)
            } else {
                data.macros
                    .insert(macro_code.to_string(), Value::Array(vec![individual_code]));
            }
        }
    }

    Ok(data)
}

fn finalize_part3(data: Data) -> std::result::Result<tera::Context, Box<dyn std::error::Error>> {
    let macros = data.macros.clone();
    let mut ctx = default_finalize_for(data)?;

    ctx.insert("macro_langs", &macros);

    Ok(ctx)
}

fn process_part5_csv(mut data: Data) -> Result<Data, Box<dyn std::error::Error>> {
    let file_name = format!("{}/iso-639-5.tsv", DEFAULT_DATA_DIR);

    rerun_if_changed(&file_name);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .trim(csv::Trim::All)
        .from_reader(File::open(file_name)?);

    for result in rdr.records() {
        let record = result?;

        let mut row: Map<String, Value> = Default::default();

        let id = record.get(1).unwrap().to_string();
        row.insert("code".to_string(), Value::String(id.clone()));

        row.insert(
            "label".to_string(),
            Value::String(record.get(2).unwrap().to_string()),
        );

        data.insert_row(&id, row);
    }

    Ok(data)
}

impl DataTrait for Data {
    fn new(_: &'static str) -> Self
    where
        Self: Sized,
    {
        Self {
            rows: Default::default(),
            macros: Default::default(),
        }
    }

    fn type_name(&self) -> &'static str {
        "LanguageCode"
    }

    fn rows(&self) -> &std::collections::BTreeMap<String, Map<String, Value>> {
        &self.rows
    }

    fn rows_mut(&mut self) -> &mut std::collections::BTreeMap<String, Map<String, Value>> {
        &mut self.rows
    }

    fn into_rows(self) -> std::collections::BTreeMap<String, Map<String, Value>> {
        self.rows
    }
}
