use codes_common::{
    default_finalize_for, input_file_name, make_default_renderer, process, Data, SimpleData,
};
use std::{fs::File, str::FromStr};
use tera::{Map, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    process(
        || Ok(SimpleData::new("CountryCode")),
        |data| process_part_1_data(data).and_then(process_part_1_language_data),
        default_finalize_for,
        make_default_renderer("part_1._rs", "part_1.rs"),
    )?;

    process(
        || Ok(SimpleData::new("SubdivisionCode")),
        |data| process_part_2_data(data).and_then(process_part_2_name_data),
        default_finalize_for,
        make_default_renderer("part_2._rs", "part_2.rs"),
    )?;

    process(
        || Ok(SimpleData::new("SubdivisionCategoryCode")),
        process_part_2_category_data,
        default_finalize_for,
        make_default_renderer("categories._rs", "categories.rs"),
    )?;

    process(
        || Ok(SimpleData::new("TerritoryCode")),
        process_part_2_territory_data,
        default_finalize_for,
        make_default_renderer("territories._rs", "territories.rs"),
    )
}

fn process_part_1_data(mut data: SimpleData) -> Result<SimpleData, Box<dyn std::error::Error>> {
    let file_name = input_file_name("country-codes.csv");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(File::open(file_name)?);

    for result in rdr.records() {
        let record = result?;

        let mut row: Map<String, Value> = Default::default();

        let id = record.get(0).unwrap().to_string();
        row.insert("alpha_2_code".to_string(), Value::String(id.clone()));

        let alpha_3_code = record.get(1).unwrap().to_string();
        if !alpha_3_code.is_empty() {
            row.insert("alpha_3_code".to_string(), Value::String(alpha_3_code));
        }

        let numeric_code = record.get(2).unwrap();
        if !numeric_code.is_empty() {
            let numeric_code = u32::from_str(numeric_code).unwrap();
            row.insert(
                "numeric_code".to_string(),
                Value::Number(numeric_code.into()),
            );
        }

        let independent = record.get(3).unwrap().to_string();
        if independent == "YES" {
            row.insert("independent".to_string(), Value::Bool(true));
        } else if independent == "NO" {
            row.insert("independent".to_string(), Value::Bool(false));
        }

        row.insert(
            "status".to_string(),
            Value::String(status_from_standard_string(record.get(4).unwrap())),
        );

        let mut short_name = record.get(5).unwrap().to_string();
        if short_name.is_empty() {
            short_name = record.get(6).unwrap().to_string();
            if short_name.is_empty() {
                short_name = record.get(4).unwrap().replace('-', " ");
            }
        }
        row.insert("short_name".to_string(), Value::String(short_name));

        let full_name = record.get(7).unwrap().to_string();
        if !full_name.is_empty() {
            row.insert("full_name".to_string(), Value::String(full_name));
        }

        data.insert_row(&id, row);
    }

    Ok(data)
}

fn process_part_1_language_data(
    mut data: SimpleData,
) -> Result<SimpleData, Box<dyn std::error::Error>> {
    let file_name = input_file_name("languages.csv");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(File::open(file_name)?);

    for result in rdr.records() {
        let record = result?;

        let code = record.get(0).unwrap();
        let row = data.get_mut(code).unwrap();

        let language_alpha_3_code = record.get(4).unwrap();
        if language_alpha_3_code != "002" {
            // See https://www.iso.org/obp/ui/#iso:code:3166:KM
            let is_administrative = record.get(5).unwrap();

            if is_administrative == "YES" {
                row.insert(
                    "administrative_language".to_string(),
                    language_alpha_3_code.into(),
                );
            }
            if !row.contains_key("languages") {
                row.insert("languages".to_string(), Value::Array(Default::default()));
            }
            let languages = row.get_mut("languages").unwrap();
            languages
                .as_array_mut()
                .unwrap()
                .push(language_alpha_3_code.into());
        }
    }

    Ok(data)
}

fn process_part_2_data(mut data: SimpleData) -> Result<SimpleData, Box<dyn std::error::Error>> {
    let file_name = input_file_name("subdivisions.csv");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(File::open(file_name)?);

    for result in rdr.records() {
        let record = result?;

        let mut row: Map<String, Value> = Default::default();

        let id = record.get(4).unwrap().to_string();
        row.insert("code".to_string(), Value::String(id.clone()));

        row.insert(
            "country_alpha_2_code".to_string(),
            Value::String(record.get(0).unwrap().to_string()),
        );

        let category_code = record.get(3).unwrap();
        if !category_code.is_empty() {
            let category_code = u32::from_str(category_code).unwrap();
            row.insert(
                "category_code".to_string(),
                Value::Number(category_code.into()),
            );
        }

        let parent_code = record.get(6).unwrap().to_string();
        if !parent_code.is_empty() {
            row.insert(
                "parent_subdivision_code".to_string(),
                Value::String(parent_code.replace('-', "_")),
            );
        }

        data.insert_row(&id.replace('-', "_"), row);
    }

    Ok(data)
}

fn process_part_2_name_data(
    mut data: SimpleData,
) -> Result<SimpleData, Box<dyn std::error::Error>> {
    let file_name = input_file_name("subdivision-names.csv");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(File::open(file_name)?);

    for result in rdr.records() {
        let record = result?;

        let id = record.get(4).unwrap().replace('-', "_");
        let row = data.get_mut(&id).unwrap();

        row.insert(
            "name_language".to_string(),
            Value::String(record.get(6).unwrap().to_string()),
        );

        row.insert(
            "name".to_string(),
            Value::String(record.get(7).unwrap().to_string()),
        );

        let name_local_variation = record.get(8).unwrap().to_string();
        if !name_local_variation.is_empty() {
            row.insert(
                "name_local_variation".to_string(),
                Value::String(name_local_variation),
            );
        }
    }

    Ok(data)
}

fn process_part_2_category_data(
    mut data: SimpleData,
) -> Result<SimpleData, Box<dyn std::error::Error>> {
    let file_name = input_file_name("subdivision-categories.csv");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(File::open(file_name)?);

    for result in rdr.records() {
        let record = result?;

        let language_alpha_3_code = record.get(5).unwrap();
        if language_alpha_3_code == "eng" {
            let mut row: Map<String, Value> = Default::default();

            let id = record.get(3).unwrap().to_string();
            row.insert("code".to_string(), Value::String(id.clone()));

            row.insert(
                "country_alpha_2_code".to_string(),
                Value::String(record.get(0).unwrap().to_string()),
            );

            row.insert(
                "language_alpha_3_code".to_string(),
                Value::String(language_alpha_3_code.into()),
            );

            row.insert(
                "name".to_string(),
                Value::String(record.get(6).unwrap().to_string()),
            );

            let name_plural = record.get(7).unwrap();
            if !name_plural.is_empty() {
                row.insert("name_plural".to_string(), Value::String(name_plural.into()));
            }

            data.insert_row(&id, row);
        }
    }

    Ok(data)
}

fn process_part_2_territory_data(
    mut data: SimpleData,
) -> Result<SimpleData, Box<dyn std::error::Error>> {
    let file_name = input_file_name("subdivision-categories.csv");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(File::open(file_name)?);

    for result in rdr.records() {
        let record = result?;

        let language_alpha_3_code = record.get(5).unwrap();
        if language_alpha_3_code == "eng" {
            let mut row: Map<String, Value> = Default::default();

            let id = record.get(3).unwrap().to_string();
            row.insert("code".to_string(), Value::String(id.clone()));

            row.insert(
                "country_alpha_2_code".to_string(),
                Value::String(record.get(0).unwrap().to_string()),
            );

            row.insert(
                "language_alpha_3_code".to_string(),
                Value::String(language_alpha_3_code.into()),
            );

            row.insert(
                "name".to_string(),
                Value::String(record.get(6).unwrap().to_string()),
            );

            data.insert_row(&id, row);
        }
    }

    Ok(data)
}

fn status_from_standard_string(s: &str) -> String {
    match s {
        "officially-assigned" => "Status::OfficiallyAssigned".to_string(),
        "exceptionally-reserved" => "Status::ExceptionallyReserved".to_string(),
        "indeterminately-reserved" => "Status::IndeterminatelyReserved".to_string(),
        "transitionally-reserved" => "Status::TransitionallyReserved".to_string(),
        "formerly-used" => "Status::FormerlyUsed".to_string(),
        _ => unreachable!("not a valid status: {}", s),
    }
}
