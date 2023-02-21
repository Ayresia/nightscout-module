use std::{str::FromStr, fmt::Display};

use serde::Deserialize;
use serde_repr::Deserialize_repr;

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

#[derive(Deserialize)]
pub struct EntriesResponse {
    pub sgv: f32,
    pub trend: TrendDirection,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Units {
    Mmol,
    Mgl
}

impl FromStr for Units {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mmol" => Ok(Units::Mmol),
            "mg/l" => Ok(Units::Mgl),
            _ => Err(format!("Unknown units value: {}", s))
        }
    }
}

impl Display for Units {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Units::Mmol => "mmol",
            Units::Mgl => "mg/l",
        };

        s.fmt(f)
    }
}
