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
