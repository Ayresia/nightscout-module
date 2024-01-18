use crate::nightscout::Nightscout;
use clap::builder::TypedValueParser;
use clap::{arg, Parser};
use models::Units;

mod models;
mod nightscout;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, help = "Nightscout url to fetch glucose data from")]
    url: String,

    #[arg(
        long,
        help = "The displayed units in glucose data",
        default_value_t = models::Units::Mmol,
        value_parser = clap::builder::PossibleValuesParser::new(["mmol", "mg/l"])
            .map(|s| s.parse::<models::Units>().unwrap()),
    )]
    units: Units,
}

pub const MGDL_PER_MMOL: f32 = 18.0;

#[tokio::main]
pub async fn main() {
    let args = Args::parse();
    let nightscout = Nightscout::new(&args.url);

    let entries = match nightscout.get_entries().await {
        Some(result) => result,
        None => {
            eprintln!("Unable to fetch glucose data");
            return;
        }
    };

    if entries.len() < 2 {
        eprintln!("No glucose data available");
        return;
    }

    let first_entry = entries.get(0).unwrap();
    let second_entry = entries.get(1).unwrap();

    let mut delta = first_entry.sgv - second_entry.sgv;
    let mut value = first_entry.sgv;

    if args.units == Units::Mmol {
        value /= MGDL_PER_MMOL;
        delta /= MGDL_PER_MMOL;
    }

    let trend = &first_entry.trend.to_string();
    println!("{value:.1} {delta:+.1} {trend}");
}
