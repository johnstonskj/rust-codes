use codes_common::csv::process_csv_input;
use codes_common::{
    default_finalize_for, insert_field, insert_optional_field, make_default_renderer, process,
    Data, DataRow, SimpleData,
};
use csv::StringRecord;
use std::str::FromStr;
use tera::Value;

#[allow(dead_code)]
const TYPE_NAME: &str = "CharacterSetCode";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    process(
        || Ok(SimpleData::new(TYPE_NAME)),
        |data| process_csv_input(data, "character-sets-1.csv", process_input_row),
        default_finalize_for,
        make_default_renderer("lib._rs", "generated.rs"),
    )
}

fn process_input_row(
    record: StringRecord,
    row: &mut DataRow,
) -> Result<String, Box<dyn std::error::Error>> {
    let id = record.get(2).unwrap().to_string();

    let name = record.get(1).unwrap().to_string();
    insert_field!(name.clone() => row, "name");

    // Required fields
    insert_field!(record, 2 => row, "mib_enum", u16);
    insert_field!(
        record, 3 => row, "source"
    );

    insert_optional_field!(
        record,
        row,
        0 => "preferred_alias",
        4 => "reference"
    );

    // There should be a field 6: note, but there is no
    // entry with data so it is ignored.

    let aliases: Vec<Value> = record
        .get(5)
        .unwrap()
        // "\n\n" works on macos and linux, but not windows.
        .split('\n')
        .filter_map(|s| {
            if !s.is_empty() && s != "\r" && s != name {
                Some(s.to_string().into())
            } else {
                None
            }
        })
        .collect();
    insert_field!(aliases => row, "aliases");

    Ok(id)
}
