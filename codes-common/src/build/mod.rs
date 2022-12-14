/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use std::{collections::BTreeMap, env, fmt::Debug, fs::File, path::Path};
use tera::{Map, Value};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const DEFAULT_NUMERIC_CODE_TYPE: &str = "u16";

pub const DEFAULT_DATA_DIR: &str = "data";

pub const DEFAULT_TEMPLATE_DIR: &str = "templates";

pub type DataRow = Map<String, Value>;

pub type DataMap = BTreeMap<String, Map<String, Value>>;

pub trait Data {
    fn new(type_name: &'static str) -> Self
    where
        Self: Sized;
    fn new_with_inner(type_name: &'static str, inner_type_name: &'static str) -> Self
    where
        Self: Sized;
    fn type_name(&self) -> &'static str;
    fn inner_type_name(&self) -> Option<&'static str>;
    fn has_inner_type(&self) -> bool {
        self.inner_type_name().is_some()
    }
    fn all_ids(&self) -> Vec<&String> {
        self.rows().keys().collect()
    }
    fn all_ids_sorted(&self) -> Value {
        let mut all_ids = self.all_ids();
        all_ids.sort();
        all_ids.dedup();
        Value::Array(
            all_ids
                .into_iter()
                .map(|s| Value::String(s.into()))
                .collect(),
        )
    }
    fn rows(&self) -> &DataMap;
    fn rows_mut(&mut self) -> &mut DataMap;
    fn into_rows(self) -> DataMap;
    fn contains(&self, id: &str) -> bool {
        self.rows().contains_key(id)
    }
    fn get(&self, id: &str) -> Option<&DataRow> {
        self.rows().get(id)
    }
    fn get_mut(&mut self, id: &str) -> Option<&mut DataRow> {
        self.rows_mut().get_mut(id)
    }
    fn insert_row(&mut self, id: &str, row: DataRow) {
        self.rows_mut().insert(id.to_string(), row);
    }
    fn insert_row_value(&mut self, id: &str, key: &str, value: Value) {
        let row = self.get_mut(id).unwrap();
        row.insert(key.to_string(), value);
    }
}

#[derive(Debug, Default)]
pub struct SimpleData {
    type_name: &'static str,
    inner_type_name: Option<&'static str>,
    rows: DataMap,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn process<T, I, P, F, R>(
    init: I,
    process_data: P,
    finalize: F,
    render: R,
) -> Result<(), Box<dyn std::error::Error>>
where
    I: FnOnce() -> Result<T, Box<dyn std::error::Error>>,
    P: FnOnce(T) -> Result<T, Box<dyn std::error::Error>>,
    F: FnOnce(T) -> Result<tera::Context, Box<dyn std::error::Error>>,
    R: FnOnce(tera::Context) -> Result<tera::Context, Box<dyn std::error::Error>>,
{
    init()
        .and_then(process_data)
        .and_then(finalize)
        .and_then(render)?;
    Ok(())
}

pub fn default_init<T>() -> Result<T, Box<dyn std::error::Error>>
where
    T: Default,
{
    Ok(Default::default())
}

pub fn default_finalize<T>(data: T) -> Result<tera::Context, Box<dyn std::error::Error>>
where
    T: Into<tera::Context>,
{
    Ok(data.into())
}

pub fn default_finalize_for<T>(data: T) -> Result<tera::Context, Box<dyn std::error::Error>>
where
    T: Data,
{
    let mut ctx = tera::Context::new();

    ctx.insert("type_name", &Value::String(data.type_name().into()));

    if let Some(inner_type_name) = data.inner_type_name() {
        ctx.insert("inner_type_name", &Value::String(inner_type_name.into()));
    }

    ctx.insert("all_ids", &data.all_ids_sorted());

    ctx.insert(
        "codes",
        &Value::Object(
            data.into_rows()
                .into_iter()
                .map(|(key, value)| (key, Value::Object(value)))
                .collect(),
        ),
    );

    Ok(ctx)
}

#[inline]
pub fn input_file_name(name: &str) -> String {
    let file_name = format!("{}/{}", DEFAULT_DATA_DIR, name);
    rerun_if_changed(&file_name);
    file_name
}

#[inline]
pub fn rerun_if_changed(file_name: &str) {
    println!("cargo:rerun-if-changed={}", file_name);
}

#[inline]
pub fn rerun_if_template_changed(file_name: &str) {
    println!(
        "cargo:rerun-if-changed={}/{}",
        DEFAULT_TEMPLATE_DIR, file_name
    );
}

pub fn make_default_renderer<S1, S2>(
    template_name: S1,
    generated_file_name: S2,
) -> impl Fn(tera::Context) -> Result<tera::Context, Box<dyn std::error::Error>>
where
    S1: Into<String>,
    S2: Into<String>,
{
    let template_name = template_name.into();
    let generated_file_name = generated_file_name.into();
    move |ctx: tera::Context| -> Result<tera::Context, Box<dyn std::error::Error>> {
        let output_dir: String = env::var("OUT_DIR").unwrap();
        let file_name = Path::new(&output_dir).join(&generated_file_name);

        rerun_if_template_changed(&template_name);

        let tera = tera::Tera::new(&format!("{}/*._rs", DEFAULT_TEMPLATE_DIR))?;

        let file = File::create(file_name)?;
        tera.render_to(&template_name, &ctx, file)?;

        Ok(ctx)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Data for SimpleData {
    fn new(type_name: &'static str) -> Self
    where
        Self: Sized,
    {
        Self {
            type_name,
            inner_type_name: None,
            rows: Default::default(),
        }
    }
    fn new_with_inner(type_name: &'static str, inner_type_name: &'static str) -> Self
    where
        Self: Sized,
    {
        Self {
            type_name,
            inner_type_name: Some(inner_type_name),
            rows: Default::default(),
        }
    }

    fn type_name(&self) -> &'static str {
        self.type_name
    }

    fn inner_type_name(&self) -> Option<&'static str> {
        self.inner_type_name
    }

    fn rows(&self) -> &BTreeMap<String, Map<String, Value>> {
        &self.rows
    }

    fn rows_mut(&mut self) -> &mut BTreeMap<String, Map<String, Value>> {
        &mut self.rows
    }

    fn into_rows(self) -> BTreeMap<String, Map<String, Value>> {
        self.rows
    }
}

impl SimpleData {
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&String, &mut Map<String, Value>) -> bool,
    {
        self.rows.retain(f);
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "csv_tools")]
#[macro_use]
pub mod csv;
