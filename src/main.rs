use crate::{nightscout::Nightscout, helpers::parse_trend};
use clap::{arg, Parser};

mod models;
mod nightscout;
mod helpers;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, help = "Nightscout url to fetch glucose data from")]
    url: String
}

#[tokio::main]
pub async fn main() {
    let args = Args::parse();
    let nightscout = Nightscout::new(&args.url);
    let entries = nightscout.get_entries().await;

    match entries {
        Some(entries) => {
            if entries.len() < 2 {
                eprintln!("No available glucose data");
            }

            let first_entry = entries.get(0).unwrap();
            let second_entry = entries.get(1).unwrap();

            let converted_value = first_entry.sgv / 18_f32;
            let delta = (first_entry.sgv - second_entry.sgv) / 18_f32;
            let trend = parse_trend(&first_entry.trend);

            println!("{converted_value:.1} {delta:+.1} {trend}");
        },
        _ => eprintln!("Unable to fetch glucose data")
    };
}
