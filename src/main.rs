use clap::builder::TypedValueParser;
use crate::{nightscout::Nightscout, helpers::parse_trend};
use clap::{arg, Parser};
use models::Units;

mod models;
mod nightscout;
mod helpers;

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
    units: Units
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
                return;
            }

            let first_entry = entries.get(0).unwrap();
            let second_entry = entries.get(1).unwrap();

            let trend = parse_trend(&first_entry.trend);
            let mut delta = first_entry.sgv - second_entry.sgv;

            match args.units {
                Units::Mmol => {
                    let converted_value = first_entry.sgv / 18_f32;
                    delta /= 18_f32;

                    println!("{converted_value:.1} {delta:+.1} {trend}");
                },
                Units::Mgl => {
                    println!("{0:.1} {delta:+.1} {trend}", first_entry.sgv);
                }
            }
        },
        _ => eprintln!("Unable to fetch glucose data")
    };
}
