use codes_common::{default_finalize, default_init, make_default_renderer, process};
use tera::Value;

#[allow(dead_code)]
const TYPE_NAME: &str = "{{ type name }}";

#[derive(Debug, Default)]
struct Data {
    #[allow(dead_code)]
    rows: Vec<DataRow>,
}

#[derive(Debug)]
struct DataRow {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    process(
        default_init,
        process_input_data,
        default_finalize,
        make_default_renderer("lib._rs", "generated.rs"),
    )
}

impl From<Data> for tera::Context {
    fn from(_data: Data) -> Self {
        let mut ctx = tera::Context::new();

        ctx.insert(
            "type_name".to_string(),
            &Value::String("LanguageCode".to_string()),
        );

        ctx
    }
}

impl From<DataRow> for Value {
    fn from(_row: DataRow) -> Self {
        todo!()
    }
}

fn process_input_data(_data: Data) -> Result<Data, Box<dyn std::error::Error>> {
    Ok(Default::default())
}
