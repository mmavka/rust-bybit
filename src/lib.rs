pub mod error;
pub mod util;
pub mod ws;

pub use ws::WebSocketApiClient;

pub enum FutureRole {
    Linear,
    Inverse,
}

pub struct KlineIntervals {
    pub kline_intervals: Vec<KlineInterval>,
}

impl KlineIntervals {
    pub fn all() -> Vec<KlineInterval> {
        vec![
            KlineInterval::Min1,
            KlineInterval::Min3,
            KlineInterval::Min5,
            KlineInterval::Min15,
            KlineInterval::Min30,
            KlineInterval::Min60,
            KlineInterval::Min120,
            KlineInterval::Min240,
            KlineInterval::Min360,
            KlineInterval::Min720,
            KlineInterval::Day,
            KlineInterval::Week,
            KlineInterval::Month,
        ]
    }
}

pub enum KlineInterval {
    Min1,
    Min3,
    Min5,
    Min15,
    Min30,
    Min60,
    Min120,
    Min240,
    Min360,
    Min720,
    Day,
    Week,
    Month,
}

impl From<KlineInterval> for &str {
    fn from(value: KlineInterval) -> Self {
        use KlineInterval::*;
        match value {
            Min1 => "1",
            Min3 => "3",
            Min5 => "5",
            Min15 => "15",
            Min30 => "30",
            Min60 => "60",
            Min120 => "120",
            Min240 => "240",
            Min360 => "360",
            Min720 => "720",
            Day => "D",
            Week => "W",
            Month => "M",
        }
    }
}

impl KlineInterval {
    pub fn to_string(&self) -> String {
        self.into()
    }
}