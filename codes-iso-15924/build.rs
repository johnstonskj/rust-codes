use codes_common::csv::process_scsv_input;
use codes_common::{
    default_finalize_for, make_default_renderer, process, Data, DataRow, SimpleData,
};
use csv::StringRecord;
use std::str::FromStr;

const TYPE_NAME: &str = "ScriptCode";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    process(
        || Ok(SimpleData::new(TYPE_NAME)),
        |data| process_scsv_input(data, "iso15924.txt", process_input_row),
        default_finalize_for,
        make_default_renderer("lib._rs", "generated.rs"),
    )
}

fn process_input_row(
    record: StringRecord,
    row: &mut DataRow,
) -> Result<String, Box<dyn std::error::Error>> {
    let id = record.get(0).unwrap().to_string();

    row.insert("alpha_code".to_string(), id.clone().into());

    row.insert(
        "numeric_code".to_string(),
        u16::from_str(record.get(1).unwrap()).unwrap().into(),
    );

    row.insert(
        "name".to_string(),
        record.get(2).unwrap().to_string().into(),
    );

    row.insert(
        "alias".to_string(),
        record.get(4).unwrap().to_string().into(),
    );

    row.insert(
        "unicode_version".to_string(),
        record.get(5).unwrap().to_string().into(),
    );

    row.insert(
        "date".to_string(),
        record.get(6).unwrap().to_string().into(),
    );

    Ok(id)
}
