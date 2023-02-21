use crate::models::TrendDirection;

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
        _ => "??",
    }
}
