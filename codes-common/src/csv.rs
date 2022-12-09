/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::{input_file_name, Data, DataRow};
use csv::StringRecord;
use std::fs::File;
use tera::{Map, Value};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

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

pub fn process_vsv_input<D, F>(
    mut data: D,
    file_name: &str,
    delimiter: u8,
    process_row: F,
) -> Result<D, Box<dyn std::error::Error>>
where
    D: Data,
    F: Fn(StringRecord, &mut DataRow) -> Result<String, Box<dyn std::error::Error>>,
{
    let file_name = input_file_name(file_name);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(delimiter)
        .comment(Some(b'#'))
        .trim(csv::Trim::All)
        .from_reader(File::open(file_name)?);

    for result in rdr.records() {
        let record = result?;

        let mut row: Map<String, Value> = Default::default();

        let id = process_row(record, &mut row)?;

        data.insert_row(&id, row);
    }

    Ok(data)
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
