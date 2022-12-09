use codes_common::{default_finalize_for, make_default_renderer, process, Data, SimpleData};
use tera::Value;

#[allow(dead_code)]
const TYPE_NAME: &str = "ScriptCode";

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
