use codes_common::{
    default_finalize, default_init, make_default_renderer, process, DEFAULT_DATA_DIR,
};
use std::fs::File;
use tera::{Map, Value};

const TYPE_NAME: &str = "MarketIdCode";

#[derive(Debug, Default)]
struct Data {
    rows: Vec<DataRow>,
}

#[derive(Debug)]
struct DataRow {
    mic: String,
    operating_mic: String,
    market_name: String,
    legal_entity_name: String,
    legal_entity_id: String,
    category_code: String,
    acronym: String,
    country_code: String, // TODO: Make this a type!
    city: String,
    url: String,
    status: String,
    created: String,
    last_modified: String,
    last_validated: String,
    expirey_date: String,
    comments: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    process(
        default_init,
        process_mic_csv,
        default_finalize,
        make_default_renderer("lib._rs", "generated.rs"),
    )
}

impl From<Data> for tera::Context {
    fn from(data: Data) -> Self {
        let mut ctx = tera::Context::new();

        ctx.insert(
            "type_name".to_string(),
            &Value::String(TYPE_NAME.to_string()),
        );

        let mut all_ids: Vec<String> = data.rows.iter().map(|r| &r.mic).cloned().collect();
        all_ids.sort();
        all_ids.dedup();

        ctx.insert(
            "all_ids",
            &Value::Array(all_ids.into_iter().map(Value::String).collect()),
        );

        ctx.insert(
            "codes",
            &Value::Object(
                data.rows
                    .into_iter()
                    .map(|row| (row.mic.clone(), row.into()))
                    .collect(),
            ),
        );

        ctx
    }
}

impl From<DataRow> for Value {
    fn from(row: DataRow) -> Self {
        let mut map: Map<String, Value> = Default::default();
        map.insert("mic".to_string(), Value::String(row.mic));
        if !row.operating_mic.is_empty() {
            map.insert(
                "operating_mic".to_string(),
                Value::String(row.operating_mic),
            );
        }
        map.insert("market_name".to_string(), Value::String(row.market_name));
        if !row.legal_entity_name.is_empty() {
            map.insert(
                "legal_entity_name".to_string(),
                Value::String(row.legal_entity_name),
            );
        }
        if !row.legal_entity_id.is_empty() {
            map.insert(
                "legal_entity_id".to_string(),
                Value::String(row.legal_entity_id),
            );
        }
        if !row.category_code.is_empty() {
            map.insert(
                "category_code".to_string(),
                Value::String(row.category_code),
            );
        }
        if !row.acronym.is_empty() {
            map.insert("acronym".to_string(), Value::String(row.acronym));
        }
        map.insert("country_code".to_string(), Value::String(row.country_code));
        map.insert("city".to_string(), Value::String(row.city));
        if !row.url.is_empty() {
            map.insert("url".to_string(), Value::String(row.url));
        }
        map.insert("status".to_string(), Value::String(row.status));
        map.insert("created".to_string(), Value::String(row.created));
        if !row.last_modified.is_empty() {
            map.insert(
                "last_modified".to_string(),
                Value::String(row.last_modified),
            );
        }
        if !row.last_validated.is_empty() {
            map.insert(
                "last_validated".to_string(),
                Value::String(row.last_validated),
            );
        }
        if !row.expirey_date.is_empty() {
            map.insert("expirey_date".to_string(), Value::String(row.expirey_date));
        }
        if !row.comments.is_empty() {
            map.insert("comments".to_string(), Value::String(row.comments));
        }

        Value::Object(map)
    }
}

fn process_mic_csv(mut data: Data) -> Result<Data, Box<dyn std::error::Error>> {
    let file_name = format!("{}/ISO10383_MIC.csv", DEFAULT_DATA_DIR);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(File::open(file_name)?);

    for result in rdr.records() {
        let record = result?;
        let oprt_sgmt = record.get(2).unwrap() == "OPRT";
        let mut row = DataRow {
            mic: record.get(0).unwrap().to_string(),
            operating_mic: record.get(1).unwrap().to_string(),
            market_name: record.get(3).unwrap().to_string(),
            legal_entity_name: record.get(4).unwrap().to_string(),
            legal_entity_id: record.get(5).unwrap().to_string(),
            category_code: record.get(6).unwrap().to_string(),
            acronym: record.get(7).unwrap().to_string(),
            country_code: record.get(8).unwrap().to_string(),
            city: record.get(9).unwrap().to_string(),
            url: record.get(10).unwrap().to_string(),
            status: record.get(11).unwrap().to_string(),
            created: record.get(12).unwrap().to_string(),
            last_modified: record.get(13).unwrap().to_string(),
            last_validated: record.get(12).unwrap().to_string(),
            expirey_date: record.get(13).unwrap().to_string(),
            comments: record.get(14).unwrap().to_string(),
        };

        if oprt_sgmt {
            assert_eq!(row.mic, row.operating_mic);
            row.operating_mic = String::new();
        } else {
            assert_ne!(row.mic, row.operating_mic);
        }
        data.rows.push(row);
    }

    Ok(data)
}
