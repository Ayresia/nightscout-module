use serde::Deserialize;
use serde_repr::Deserialize_repr;

const BASE_URL: &str = "https://ayresiacgm.herokuapp.com";

#[derive(Deserialize)]
pub struct EntriesResponse {
    sgv: f32,
    trend: TrendDirection
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
    OutOfRange = 9
}

#[tokio::main]
pub async fn main() {
    if let Ok(res) = reqwest::get(format!("{BASE_URL}/api/v1/entries.json?count=1")).await {
        if let Ok(encoded_response) = res.json::<[EntriesResponse; 2]>().await {
            let converted_value = encoded_response[0].sgv / 18_f32;
            let trend = parse_trend(&encoded_response[0].trend);

            println!("{converted_value:.1} mmol/L {trend}");
            return;
        }
    }

    println!("Unable to fetch glucose");
}

pub fn parse_trend(trend: &TrendDirection) -> &str {
    return match trend {
        TrendDirection::DoubleUp => "↑↑",
        TrendDirection::SingleUp  => "↑",
        TrendDirection::FortyFiveUp => "↗︎",
        TrendDirection::Flat => "→",
        TrendDirection::FortyFiveDown => "↘",
        TrendDirection::SingleDown => "↓",
        TrendDirection::DoubleDown => "↓↓",
        _ => ""
    }
}
