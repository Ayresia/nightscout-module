use core::{fmt::Display, str::FromStr};
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

impl ToString for TrendDirection {
    fn to_string(&self) -> String {
        let result = match self {
            TrendDirection::DoubleUp => "↑↑",
            TrendDirection::SingleUp => "↑",
            TrendDirection::FortyFiveUp => "↗︎",
            TrendDirection::Flat => "→",
            TrendDirection::FortyFiveDown => "↘",
            TrendDirection::SingleDown => "↓",
            TrendDirection::DoubleDown => "↓↓",
            TrendDirection::NotComputable => "-",
            _ => "??",
        };

        result.to_string()
    }
}

#[derive(Deserialize)]
pub struct EntriesResponse {
    pub sgv: f32,
    pub trend: TrendDirection,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Units {
    Mmol,
    Mgl,
}

impl FromStr for Units {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mmol" => Ok(Self::Mmol),
            "mg/l" => Ok(Self::Mgl),
            _ => Err(format!("Unknown units value: {}", s)),
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
