use serde::Deserialize;
use serde_repr::Deserialize_repr;

const BASE_URL: &str = "https://t1.ayresia.dev";

#[derive(Deserialize)]
pub struct EntriesResponse {
    sgv: f32,
    trend: TrendDirection,
}

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum TrendDirection {
    None = 0,
    DoubleUp = 1,
    SingleUp = 2,
    FortyFiveUp = 3,
    Flat = 4,
    FortyFiveDown = 5,
    SingleDown = 6,
    DoubleDown = 7,
    NotComputable = 8,
    OutOfRange = 9,
}

pub fn parse_trend(trend: &TrendDirection) -> &str {
    match trend {
        TrendDirection::DoubleUp => "↑↑",
        TrendDirection::SingleUp => "↑",
        TrendDirection::FortyFiveUp => "↗︎",
        TrendDirection::Flat => "→",
        TrendDirection::FortyFiveDown => "↘",
        TrendDirection::SingleDown => "↓",
        TrendDirection::DoubleDown => "↓↓",
        TrendDirection::NotComputable => "-",
        _ => "",
    }
}

#[tokio::main]
pub async fn main() {
    let result = match reqwest::get(format!("{BASE_URL}/api/v1/entries.json?count=2")).await {
        Ok(res) => res,
        Err(_) => {
            eprintln!("Unable to fetch glucose");
            return;
        }
    };

    let encoded_response = match result.json::<[EntriesResponse; 2]>().await {
        Ok(response) => response,
        Err(_) => {
            eprintln!("Unable to transform response to json");
            return;
        }
    };

    let converted_value = encoded_response[0].sgv / 18_f32;
    let delta = (encoded_response[0].sgv - encoded_response[1].sgv) / 18_f32;
    let trend = parse_trend(&encoded_response[0].trend);

    println!("{converted_value:.1} {delta:+.1} {trend}");
}
