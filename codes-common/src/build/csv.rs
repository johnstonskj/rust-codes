/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::build::{input_file_name, Data, DataRow};
use csv::Reader;
use csv::StringRecord;
use std::fs::File;
use tera::{Map, Value};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! insert_field {
    ($value:expr => $row:ident, $name:expr) => {
        $row.insert($name.to_string(), $value.into());
    };
    ($record:ident, $index:expr => $row:ident, $name:expr) => {
        $row.insert($name.to_string(), $record.get($index).unwrap().into());
    };
    ($record:ident, $index:expr => $row:ident, $name:expr) => {
        $row.insert($name.to_string(), $record.get($index).unwrap().into());
    };
    ($record:ident, $row:ident, $($index:expr => $name:expr),+) => {
        $(
            insert_field!($record, $index => $row, $name);
        )+
    };
    ($record:ident, $index:expr => $row:ident, $name:expr, $field_type:ty) => {{
        let temp = $record.get($index).unwrap();
        let temp = <$field_type>::from_str(temp).unwrap();
        $row.insert($name.to_string(), temp.into());
    }};
    ($record:ident, $row:ident, $($index:expr => $name:expr, $field_type:ty),+) => {
        $(
            insert_field!($record, $index => $row, $name, $field_type);
        )+
    };
}

#[macro_export]
macro_rules! insert_optional_field {
    ($record:ident, $index:expr => $row:ident, $name:expr) => {{
        let temp = $record.get($index).unwrap();
        if !temp.is_empty() {
            $row.insert($name.to_string(), temp.into());
        }
    }};
    ($record:ident, $row:ident, $($index:expr => $name:expr),+) => {
        $(
            insert_optional_field!($record, $index => $row, $name);
        )+
    };
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn process_scsv_input<D, F>(
    data: D,
    file_name: &str,
    process_row: F,
) -> Result<D, Box<dyn std::error::Error>>
where
    D: Data,
    F: Fn(StringRecord, &mut DataRow) -> Result<String, Box<dyn std::error::Error>>,
{
    process_vsv_input(data, file_name, b';', process_row)
}

pub fn process_csv_input<D, F>(
    data: D,
    file_name: &str,
    process_row: F,
) -> Result<D, Box<dyn std::error::Error>>
where
    D: Data,
    F: Fn(StringRecord, &mut DataRow) -> Result<String, Box<dyn std::error::Error>>,
{
    process_vsv_input(data, file_name, b',', process_row)
}

pub fn process_tsv_input<D, F>(
    data: D,
    file_name: &str,
    process_row: F,
) -> Result<D, Box<dyn std::error::Error>>
where
    D: Data,
    F: Fn(StringRecord, &mut DataRow) -> Result<String, Box<dyn std::error::Error>>,
{
    process_vsv_input(data, file_name, b'\t', process_row)
}

pub fn open_csv_file(
    file_name: &str,
    delimiter: Option<u8>,
) -> Result<Reader<File>, Box<dyn std::error::Error>> {
    let file_name = input_file_name(file_name);

    Ok(csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(delimiter.unwrap_or(b','))
        .comment(Some(b'#'))
        .trim(csv::Trim::All)
        .from_reader(File::open(file_name)?))
}

fn process_vsv_input<D, F>(
    mut data: D,
    file_name: &str,
    delimiter: u8,
    process_row: F,
) -> Result<D, Box<dyn std::error::Error>>
where
    D: Data,
    F: Fn(StringRecord, &mut DataRow) -> Result<String, Box<dyn std::error::Error>>,
{
    let mut rdr = open_csv_file(file_name, Some(delimiter))?;

    for result in rdr.records() {
        let record = result?;

        let mut row: Map<String, Value> = Default::default();

        let id = process_row(record, &mut row)?;

        data.insert_row(&id, row);
    }

    Ok(data)
}
