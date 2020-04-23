// fetch from
// https://member.bilibili.com/x/web/data/playanalysis?tmid=67358318&copyright=0
// TODO login

use std::fs::File;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct PlayList {
    aid: u32,
    bvid: String,
    view: u32,
    rate: u32,
    duration: u32,
    avg_duration: u32,
    title: String
}

#[derive(Deserialize, Debug)]
struct BilibiliPlayAnalysis {
    arc_play_list: Vec<PlayList>,
}

#[derive(Deserialize, Debug)]
struct BilibiliData {
    data: BilibiliPlayAnalysis,
}

fn main() {
    println!("Hello, world!");
    let file_path = env::var("FILE_PATH").unwrap();
    let file = File::open(file_path).expect("could not open file");
    let data_json: BilibiliData = serde_json::from_reader(file).expect("error while reading json");

    let header = Header(
        "aid".to_string(),
        "bvid".to_string(),
        "view".to_string(),
        "rate".to_string(),
        "duration".to_string(),
        "avg_duration".to_string(),
        "title".to_string(),
    );
    let data = data_json.data.arc_play_list.iter().map(|item| 
        Data(
            item.aid,
            item.bvid.clone(),
            item.view,
            item.rate,
            item.duration,
            item.avg_duration,
            item.title.clone(),
        )
    ).collect();

    let sql = generate_sql(&(header, data));
    println!("content: {}", sql);
}

// Define your own header here
#[derive(Debug)]
struct Header(String, String, String, String, String, String, String);
// Define your own data type here
#[derive(Debug)]
struct Data(u32, String, u32, u32, u32, u32, String);

fn generate_sql(csv_data: &(Header, Vec<Data>)) -> String {
    let (header, data) = csv_data;
    let base_sql = format!(
        "INSERT INTO bilibili_videos_performance ( {}, {}, {}, {}, {}, {}, {} ) VALUES ",
        header.0,
        header.1,
        header.2,
        header.3,
        header.4,
        header.5,
        header.6,
    );

    let mut values_sql = "".to_string();
    for item in data.iter() {
        let value_part = format!(
            "( {}, '{}', {}, {}, {}, {}, '{}' ),",
            item.0,
            item.1,
            item.2,
            item.3,
            item.4,
            item.5,
            item.6,
        );
        values_sql.push_str(&value_part);
    }

    let s = format!("{}{}", base_sql, values_sql);
    let len = s.len();
    s[0..len-1].to_string()
}
