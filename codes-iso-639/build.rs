use common::{default_init, make_default_renderer, process, DEFAULT_DATA_DIR};
use std::fs::File;
use tera::{Map, Value};

#[derive(Debug)]
struct Data {
    rows: Vec<Map<String, Value>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    process(
        default_init,
        process_part1_csv,
        finalize_part1,
        make_default_renderer("part_1._rs", "part_1.rs"),
    )?;

    process(
        default_init,
        process_part3_csv,
        finalize_part3,
        make_default_renderer("part_3._rs", "part_3.rs"),
    )?;

    process(
        default_init,
        process_part5_csv,
        finalize_part5,
        make_default_renderer("part_5._rs", "part_5.rs"),
    )?;

    Ok(())
}

impl Default for Data {
    fn default() -> Self {
        Self {
            rows: Default::default(),
        }
    }
}

fn process_part1_csv(mut data: Data) -> Result<Data, Box<dyn std::error::Error>> {
    let file_name = format!("{}/iso-639-1.tsv", DEFAULT_DATA_DIR);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .trim(csv::Trim::All)
        .from_reader(File::open(file_name)?);

    for result in rdr.records() {
        let record = result?;

        let mut row: Map<String, Value> = Default::default();
        row.insert(
            "code".to_string(),
            Value::String(record.get(1).unwrap().to_string()),
        );

        let label = record.get(2).unwrap().to_string();
        let (primary, other) = if label.contains('|') {
            let mut labels = label.split('|');
            let label = labels.next().unwrap().trim();
            let other = labels
                .map(|s| Value::String(s.trim().to_string()))
                .collect::<Vec<Value>>();
            (label.to_string(), other)
        } else {
            (label, Vec::default())
        };

        row.insert("label".to_string(), Value::String(primary));

        if !other.is_empty() {
            row.insert("alt_labels".to_string(), Value::Array(other));
        }

        data.rows.push(row);
    }

    Ok(data)
}

fn finalize_part1(data: Data) -> std::result::Result<tera::Context, Box<dyn std::error::Error>> {
    let mut ctx = tera::Context::new();

    ctx.insert(
        "type_name".to_string(),
        &Value::String("LanguageCode".to_string()),
    );

    let mut all_ids: Vec<String> = data
        .rows
        .iter()
        .map(|r| r.get("code").unwrap().as_str().unwrap().to_string())
        .collect();
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
                .map(|row| {
                    (
                        row.get("code").unwrap().as_str().unwrap().to_string(),
                        Value::Object(row),
                    )
                })
                .collect(),
        ),
    );

    println!("{:#?}", ctx);

    Ok(ctx)
}

fn process_part3_csv(mut data: Data) -> Result<Data, Box<dyn std::error::Error>> {
    let file_name = format!("{}/iso-639-3.tsv", DEFAULT_DATA_DIR);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .trim(csv::Trim::All)
        .from_reader(File::open(file_name)?);

    for result in rdr.records() {
        let record = result?;

        let mut row: Map<String, Value> = Default::default();
        row.insert(
            "code".to_string(),
            Value::String(record.get(0).unwrap().to_string()),
        );

        let part_1_code = record.get(3).unwrap().to_string();
        if part_1_code != "sh" {
            // Serbo-Croatian - Code element for 639-1 has been deprecated
            row.insert("part_1_code".to_string(), Value::String(part_1_code));
        }

        row.insert(
            "scope".to_string(),
            Value::String(
                match record.get(4).unwrap() {
                    "C" => "Collection",
                    "D" => "Dialect",
                    "I" => "Individual",
                    "M" => "Macro",
                    "R" => "Reserved",
                    "S" => "Special",
                    _ => unreachable!(),
                }
                .to_string(),
            ),
        );

        row.insert(
            "language_type".to_string(),
            Value::String(
                match record.get(5).unwrap() {
                    "A" => "Ancient",
                    "C" => "Constructed",
                    "E" => "Extinct",
                    "H" => "Historic",
                    "L" => "Living",
                    "S" => "Special",
                    _ => unreachable!(),
                }
                .to_string(),
            ),
        );

        row.insert(
            "ref_name".to_string(),
            Value::String(record.get(6).unwrap().to_string()),
        );

        let comment = record.get(7).unwrap().to_string();
        if !comment.is_empty() {
            row.insert("comment".to_string(), Value::String(comment));
        }

        data.rows.push(row);
    }

    Ok(data)
}

fn finalize_part3(data: Data) -> std::result::Result<tera::Context, Box<dyn std::error::Error>> {
    let mut ctx = tera::Context::new();

    ctx.insert(
        "type_name".to_string(),
        &Value::String("LanguageCode".to_string()),
    );

    let mut all_ids: Vec<String> = data
        .rows
        .iter()
        .map(|r| r.get("code").unwrap().as_str().unwrap().to_string())
        .collect();
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
                .map(|row| {
                    (
                        row.get("code").unwrap().as_str().unwrap().to_string(),
                        Value::Object(row),
                    )
                })
                .collect(),
        ),
    );

    Ok(ctx)
}

fn process_part5_csv(mut data: Data) -> Result<Data, Box<dyn std::error::Error>> {
    let file_name = format!("{}/iso-639-5.tsv", DEFAULT_DATA_DIR);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .trim(csv::Trim::All)
        .from_reader(File::open(file_name)?);

    for result in rdr.records() {
        let record = result?;

        let mut row: Map<String, Value> = Default::default();

        row.insert(
            "code".to_string(),
            Value::String(record.get(1).unwrap().to_string()),
        );
        row.insert(
            "label".to_string(),
            Value::String(record.get(2).unwrap().to_string()),
        );

        data.rows.push(row);
    }

    Ok(data)
}

fn finalize_part5(data: Data) -> std::result::Result<tera::Context, Box<dyn std::error::Error>> {
    let mut ctx = tera::Context::new();

    ctx.insert(
        "type_name".to_string(),
        &Value::String("LanguageCode".to_string()),
    );

    let mut all_ids: Vec<String> = data
        .rows
        .iter()
        .map(|r| r.get("code").unwrap().as_str().unwrap().to_string())
        .collect();
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
                .map(|row| {
                    (
                        row.get("code").unwrap().as_str().unwrap().to_string(),
                        Value::Object(row),
                    )
                })
                .collect(),
        ),
    );

    Ok(ctx)
}
