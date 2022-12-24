use codes_common::build::csv::process_csv_input;
use codes_common::build::{
    default_finalize_for, make_default_renderer, process, Data, DataRow, SimpleData,
};
use codes_common::{insert_field, insert_optional_field};
use csv::StringRecord;

const TYPE_NAME: &str = "MarketIdCode";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    process(
        || Ok(SimpleData::new(TYPE_NAME)),
        |data| process_csv_input(data, "ISO10383_MIC.csv", process_input_row),
        default_finalize_for,
        make_default_renderer("lib._rs", "generated.rs"),
    )
}

fn process_input_row(
    record: StringRecord,
    row: &mut DataRow,
) -> Result<String, Box<dyn std::error::Error>> {
    let id = record.get(0).unwrap();

    // ID field
    insert_field!(id.to_string() => row, "mic");

    let op_mic = record.get(1).unwrap();
    if op_mic != id {
        insert_field!(op_mic => row, "operating_mic");
    }

    let country = record.get(8).unwrap();
    if country != "ZZ" {
        insert_field!(country => row, "country_code");
    }

    // Required fields
    insert_field!(
        record,
        row,
        3 => "market_name",
        9 => "city",
        11 => "status"
    );

    let created = record.get(12).unwrap();
    insert_field!(fixup_date(created) => row, "created");

    // Optional fields
    insert_optional_field!(
        record,
        row,
        4 => "legal_entity_name",
        5 => "legal_entity_id",
        6 => "category_code", // convert
        7 => "acronym",
        10 => "url",
        16 => "comments"
    );

    let date = record.get(13).unwrap();
    if !date.is_empty() {
        insert_field!(fixup_date(date) => row, "last_modified");
    }

    let date = record.get(14).unwrap();
    if !date.is_empty() {
        insert_field!(fixup_date(date) => row, "last_validated");
    }

    let date = record.get(15).unwrap();
    if !date.is_empty() {
        insert_field!(fixup_date(date) => row, "expirey_date");
    }

    Ok(id.to_string())
}

fn fixup_date(s: &str) -> String {
    format!("{}-{}-{}", &s[0..=3], &s[4..=5], &s[6..=7])
}
