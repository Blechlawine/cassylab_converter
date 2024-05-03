use std::fs;

use serde::{Deserialize, Serialize};

fn main() {
    let input = fs::read_to_string("./input.xml").unwrap();
    let parsed = quick_xml::de::from_str::<CassyLab>(&input).unwrap();
    let output = serde_json::to_string(&parsed).unwrap();
    fs::write("./output.json", output).unwrap();

    let mut files: Vec<CSVFile> = vec![];

    parsed.allchannels.channels.iter().for_each(|channels| {
        channels
            .channel
            .iter()
            .enumerate()
            .for_each(|(index, channel)| {
                if let Some(file) = files.get_mut(index) {
                    if !file.header.iter().any(|h| h == &channel.quantity) {
                        file.header.push(channel.quantity.clone());
                        file.columns.push(
                            channel
                                .values
                                .clone()
                                .value
                                .unwrap_or_default()
                                .iter()
                                .map(|v| v.value)
                                .collect(),
                        );
                    }
                } else {
                    files.push(CSVFile {
                        header: vec![channel.quantity.clone()],
                        columns: vec![channel
                            .values
                            .clone()
                            .value
                            .unwrap_or_default()
                            .iter()
                            .map(|v| v.value)
                            .collect()],
                    });
                }
            })
    });
    //dbg!(&files);
    for (index, file) in files.iter().enumerate() {
        fs::write(format!("{}.csv", index), String::from(file)).unwrap();
    }
}

#[derive(Serialize, Debug)]
struct CSVFile {
    header: Vec<String>,
    columns: Vec<Vec<f64>>,
}

impl From<&CSVFile> for String {
    fn from(value: &CSVFile) -> Self {
        let mut output = String::new();
        output.push_str(&value.header.join(","));
        output.push('\n');
        let mut y = 0;
        while value.columns.iter().any(|col| col.get(y).is_some()) {
            for x in 0..value.columns.len() {
                let column = &value.columns.get(x);
                if column.is_none() {
                    output.push(',');
                    continue;
                }
                let column = column.unwrap();
                if column.is_empty() {
                    output.push(',');
                    continue;
                }
                if let Some(value) = column.get(y) {
                    output.push_str(&format!("{},", value));
                } else {
                    output.push(',');
                }
            }
            y += 1;
            output.push('\n');
        }
        output
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CassyLab {
    allchannels: AllChannels,
}

#[derive(Serialize, Deserialize, Debug)]
struct AllChannels {
    #[serde(rename = "@count")]
    count: u32,
    channels: Vec<Channels>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Channels {
    channel: Vec<Channel>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Channel {
    quantity: String,
    symbol: String,
    unit: Option<String>,
    range: Range,
    values: Values,
}

#[derive(Serialize, Deserialize, Debug)]
struct Range {
    #[serde(rename = "@min")]
    min: f64,
    #[serde(rename = "@max")]
    max: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Values {
    #[serde(rename = "@count")]
    count: u32,
    value: Option<Vec<Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Value {
    #[serde(rename = "$value")]
    value: f64,
}
