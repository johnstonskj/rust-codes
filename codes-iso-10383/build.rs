use codes_common::csv::process_csv_input;
use codes_common::{
    default_finalize_for, insert_field, insert_optional_field, make_default_renderer, process,
    Data, DataRow, SimpleData,
};
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

    // Required fields
    insert_field!(
        record,
        row,
        3 => "market_name",
        8 => "country_code",
        9 => "city",
        11 => "status",
        12 => "created"
    );

    // Optional fields
    insert_optional_field!(
        record,
        row,
        4 => "legal_entity_name",
        5 => "legal_entity_id",
        6 => "category_code",
        7 => "acronym",
        10 => "url",
        13 => "last_modified",
        14 => "last_validated",
        15 => "expirey_date",
        16 => "comments"
    );

    Ok(id.to_string())
}
