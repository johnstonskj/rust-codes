use std::str::FromStr;
use tera::{Map, Value};

use codes_common::{
    default_finalize_for, input_file_name, make_default_renderer, process, Data, SimpleData,
};
//use tera::Value;

#[allow(dead_code)]
const TYPE_NAME: &str = "RegionClassificationCode";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    process(
        || Ok(SimpleData::new(TYPE_NAME)),
        process_input_data,
        default_finalize_for,
        make_default_renderer("lib._rs", "generated.rs"),
    )
}

fn process_input_data(mut data: SimpleData) -> Result<SimpleData, Box<dyn std::error::Error>> {
    use scraper::{Html, Selector};
    use std::fs;

    fn element_text(element: &scraper::element_ref::ElementRef) -> String {
        element
            .text()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("")
            .trim()
            .to_string()
    }

    let file_name = input_file_name("m49-overview.html");
    let source = fs::read_to_string(&file_name)?;

    let document = Html::parse_document(&source);
    let row_selector = Selector::parse("#downloadTableEN tbody tr").unwrap();
    let data_selector = Selector::parse("td").unwrap();

    for row in document.select(&row_selector) {
        let mut strings = row
            .select(&data_selector)
            .enumerate()
            .filter_map(|(i, e)| {
                if let 0..=11 = &i {
                    Some(element_text(&e))
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();
        assert_eq!(strings.len(), 12);
        let country_name = strings.remove(8);
        strings.insert(9, country_name);

        let code = strings.get(0).unwrap();
        if !code.is_empty() && !data.contains(code) {
            let mut row: Map<String, Value> = Default::default();

            row.insert(
                "code_as_int".to_string(),
                u16::from_str(code).unwrap().into(),
            );

            row.insert(
                "name".to_string(),
                strings.get(1).unwrap().to_string().into(),
            );

            row.insert("kind".to_string(), String::from("Global").into());

            data.insert_row(code, row);
        }

        let code = strings.get(2).unwrap();
        if !code.is_empty() && !data.contains(code) {
            let mut row: Map<String, Value> = Default::default();

            row.insert(
                "code_as_int".to_string(),
                u16::from_str(code).unwrap().into(),
            );

            row.insert(
                "name".to_string(),
                strings.get(3).unwrap().to_string().into(),
            );

            row.insert("kind".to_string(), String::from("Region").into());

            row.insert(
                "parent_code".to_string(),
                strings.get(0).unwrap().to_string().into(),
            );

            data.insert_row(code, row);
        }

        let code = strings.get(4).unwrap();
        if !code.is_empty() && !data.contains(code) {
            let mut row: Map<String, Value> = Default::default();

            row.insert(
                "code_as_int".to_string(),
                u16::from_str(code).unwrap().into(),
            );

            row.insert(
                "name".to_string(),
                strings.get(5).unwrap().to_string().into(),
            );

            row.insert("kind".to_string(), String::from("SubRegion").into());

            row.insert(
                "parent_code".to_string(),
                strings.get(2).unwrap().to_string().into(),
            );

            data.insert_row(code, row);
        }

        let code = strings.get(6).unwrap();
        if !code.is_empty() && !data.contains(code) {
            let mut row: Map<String, Value> = Default::default();

            row.insert(
                "code_as_int".to_string(),
                u16::from_str(code).unwrap().into(),
            );

            row.insert(
                "name".to_string(),
                strings.get(7).unwrap().to_string().into(),
            );

            row.insert(
                "kind".to_string(),
                String::from("IntermediateRegion").into(),
            );

            row.insert(
                "parent_code".to_string(),
                strings.get(4).unwrap().to_string().into(),
            );

            data.insert_row(code, row);
        }

        let code = strings.get(8).unwrap();
        if !code.is_empty() && !data.contains(code) {
            let mut row: Map<String, Value> = Default::default();

            row.insert(
                "code_as_int".to_string(),
                u16::from_str(code).unwrap().into(),
            );

            row.insert(
                "name".to_string(),
                strings.get(9).unwrap().to_string().into(),
            );

            let country_alpha_2_code = strings.get(10).unwrap();
            if country_alpha_2_code.is_empty() {
                row.insert("kind".to_string(), String::from("Area").into());
            } else {
                row.insert("kind".to_string(), String::from("Country").into());

                row.insert(
                    "country_alpha_2_code".to_string(),
                    country_alpha_2_code.to_string().into(),
                );

                row.insert(
                    "country_alpha_3_code".to_string(),
                    strings.get(11).unwrap().to_string().into(),
                );
            }

            for parent in [6, 4, 2] {
                let parent_code = strings.get(parent as usize).unwrap();
                if !parent_code.is_empty() {
                    row.insert("parent_code".to_string(), parent_code.to_string().into());
                    break;
                }
            }

            data.insert_row(code, row);
        }
    }

    Ok(data)
}
