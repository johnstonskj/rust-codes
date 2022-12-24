use codes_common::build::csv::process_scsv_input;
use codes_common::build::{
    default_finalize_for, make_default_renderer, process, Data, DataRow, SimpleData,
};
use codes_common::insert_field;
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

    // ID field
    insert_field!(id.clone() => row, "alpha_code");

    // Required fields
    insert_field!(record, row, 1 => "numeric_code", u16);
    insert_field!(
        record,
        row,
        2 => "name",
        4 => "alias",
        5 => "unicode_version",
        6 => "date"
    );

    Ok(id)
}
