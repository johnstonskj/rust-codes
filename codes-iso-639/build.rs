use codes_common::csv::{open_csv_file, process_tsv_input};
use codes_common::{
    default_finalize_for, default_init, insert_field, insert_optional_field, make_default_renderer,
    process, Data as DataTrait, DataRow,
};
use csv::StringRecord;
use std::collections::BTreeMap;
use tera::{Map, Value};

#[derive(Debug, Default)]
struct Data {
    rows: BTreeMap<String, Map<String, Value>>,
    macros: Map<String, Value>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    process(
        default_init,
        |data: Data| process_tsv_input(data, "iso-639-1.tsv", process_part1_row),
        default_finalize_for,
        make_default_renderer("part_1._rs", "part_1.rs"),
    )?;

    process(
        default_init,
        |data| {
            process_tsv_input(data, "iso-639-3.tsv", process_part3_row)
                .and_then(process_part3_macro_csv)
        },
        finalize_part3,
        make_default_renderer("part_3._rs", "part_3.rs"),
    )?;

    process(
        default_init,
        |data: Data| process_tsv_input(data, "iso-639-5.tsv", process_part5_row),
        default_finalize_for,
        make_default_renderer("part_5._rs", "part_5.rs"),
    )?;

    Ok(())
}

fn process_part1_row(
    record: StringRecord,
    row: &mut DataRow,
) -> Result<String, Box<dyn std::error::Error>> {
    let id = record.get(1).unwrap().to_string();

    // ID field
    insert_field!(id.clone() => row, "code");

    // Required fields
    let names = record.get(2).unwrap();
    insert_field!(
        if names.contains('|') {
            let names = names.split('|');
            names
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("; ")
        } else {
            names.to_string()
        } => row, "label"
    );

    Ok(id)
}

fn process_part3_row(
    record: StringRecord,
    row: &mut DataRow,
) -> Result<String, Box<dyn std::error::Error>> {
    let id = record.get(0).unwrap().to_string();

    // ID field
    insert_field!(id.clone() => row, "code");

    // Required fields
    let part_1_code = record.get(3).unwrap();
    if part_1_code != "sh" {
        // Serbo-Croatian - Code element for 639-1 has been deprecated
        insert_field!(part_1_code => row, "part_1_code");
    }
    insert_field!(
        match record.get(4).unwrap() {
            "C" => "Collection",
            "D" => "Dialect",
            "I" => "Individual",
            "M" => "Macro",
            "R" => "Reserved",
            "S" => "Special",
            _ => unreachable!(),
        }
        => row, "scope"
    );
    insert_field!(
        match record.get(5).unwrap() {
            "A" => "Ancient",
            "C" => "Constructed",
            "E" => "Extinct",
            "H" => "Historic",
            "L" => "Living",
            "S" => "Special",
            _ => unreachable!(),
        }
        => row, "language_type"
    );
    insert_field!(record, 6 => row, "ref_name");

    // Optional fields
    insert_optional_field!(record, 7 => row, "comment");

    Ok(id)
}

fn process_part3_macro_csv(mut data: Data) -> Result<Data, Box<dyn std::error::Error>> {
    let mut rdr = open_csv_file("iso-639-3-macro-languages.tsv", Some(b'\t'))?;

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

fn process_part5_row(
    record: StringRecord,
    row: &mut DataRow,
) -> Result<String, Box<dyn std::error::Error>> {
    let id = record.get(1).unwrap().to_string();

    // ID field
    insert_field!(id.clone() => row, "code");

    // Required fields
    insert_field!(record, 2 => row, "label");

    Ok(id)
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
