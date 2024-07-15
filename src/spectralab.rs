use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::cassylab::{CSVFile, Channels};

pub fn convert(input: String, input_path: PathBuf) {
    let parsed = quick_xml::de::from_str::<SpectralabXML>(&input).unwrap();
    let json_file_name = input_path
        .parent()
        .unwrap()
        .join(input_path.file_stem().unwrap())
        .with_extension("json");
    let output = serde_json::to_string(&parsed).unwrap();
    fs::write(json_file_name, output).unwrap();

    let mut csv_file = CSVFile::default();

    parsed
        .measchannels
        .channels
        .into_iter()
        .for_each(|channels| {
            channels
                .channel
                .as_ref()
                .unwrap()
                .iter()
                .enumerate()
                .for_each(|(_, channel)| {
                    csv_file.header.push(channel.quantity.clone());
                    if let Some(column) = channel
                        .values
                        .value
                        .as_ref()
                        .map(|v| v.iter().map(|v| v.value).collect())
                    {
                        csv_file.columns.push(column)
                    }
                });
        });

    let csv_file_name = input_path
        .parent()
        .unwrap()
        .join(input_path.file_stem().unwrap())
        .with_extension("csv");
    fs::write(csv_file_name, String::from(&csv_file)).unwrap();
}

#[derive(Deserialize, Debug, Serialize)]
struct SpectralabXML {
    measchannels: MeasChannels,
    meascurves: MeasCurves,
    evalchannels: EvalChannels,
    evalcurves: EvalCurves,
}

#[derive(Deserialize, Debug, Serialize)]
struct EvalCurves {
    curves: Vec<Curves>,
}

#[derive(Deserialize, Debug, Serialize)]
struct EvalChannels {
    channels: Vec<Channels>,
}

#[derive(Deserialize, Debug, Serialize)]
struct MeasChannels {
    channels: Vec<Channels>,
}

#[derive(Deserialize, Debug, Serialize)]
struct MeasCurves {
    #[serde(rename = "@count")]
    count: i32,
    curves: Vec<Curves>,
}

#[derive(Deserialize, Debug, Serialize)]
struct Curves {
    #[serde(rename = "@count")]
    count: i32,
    #[serde(rename = "@text")]
    text: Option<String>,
    curve: Option<Vec<Curve>>,
    xzoom: Zoom,
    yzoom: Zoom,
    evaluations: Evaluation,
}

#[derive(Deserialize, Debug, Serialize)]
struct Curve {
    #[serde(rename = "@x0")]
    x0: i32,
    #[serde(rename = "@x1")]
    x1: i32,
    #[serde(rename = "@y0")]
    y0: i32,
    #[serde(rename = "@y1")]
    y1: i32,
    #[serde(rename = "@style")]
    style: Option<i32>,
}

#[derive(Deserialize, Debug, Serialize)]
struct Zoom {
    #[serde(rename = "@min")]
    min: i32,
    #[serde(rename = "@max")]
    max: i32,
}

#[derive(Deserialize, Debug, Serialize)]
struct Evaluation {
    #[serde(rename = "@count")]
    count: i32,
}
