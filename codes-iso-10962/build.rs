use codes_common::{
    default_finalize_for, input_file_name, make_default_renderer, process, Data, SimpleData,
};
use tera::Value;

#[allow(dead_code)]
const TYPE_NAME: &str = "{{ type name }}";

struct Details {
    code: char,
    name: String,
    description: String,
    label: String,
}

struct Category {
    details: Details,
    groups: Vec<Group>,
}

struct Group {
    details: Details,
    version: u16,
}

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

fn process_input_data(_data: SimpleData) -> Result<SimpleData, Box<dyn std::error::Error>> {
    todo!()
}
