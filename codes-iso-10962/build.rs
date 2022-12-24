use codes_common::build::{default_finalize_for, make_default_renderer, process, Data, SimpleData};

#[allow(dead_code)]
const TYPE_NAME: &str = "FinancialInstrumentClassification";

#[allow(dead_code)]
#[derive(Debug)]
struct Details {
    code: char,
    name: String,
    description: String,
    label: String,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Category {
    details: Details,
    groups: Vec<Group>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Group {
    details: Details,
    version: u16,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Attribute {
    details: Details,
    version: u16,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    process(
        || Ok(SimpleData::new(TYPE_NAME)),
        process_input_data,
        default_finalize_for,
        make_default_renderer("lib._rs", "generated.rs"),
    )
}

fn process_input_data(data: SimpleData) -> Result<SimpleData, Box<dyn std::error::Error>> {
    Ok(data)
}
