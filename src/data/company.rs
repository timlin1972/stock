use std::fmt;

use serde::{Deserialize, Serialize};

use crate::api::fugle;
use crate::cfg::CfgData;
use crate::storage;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StockData {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
    pub turnover: u64,
    pub change: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PriceChangeResult {
    Up,
    Down,
    Flat,
}

impl fmt::Display for PriceChangeResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PriceChangeResult::Up => write!(f, "Up"),
            PriceChangeResult::Down => write!(f, "Down"),
            PriceChangeResult::Flat => write!(f, "Flat"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum SwingResult {
    UpMinChange,
    DownMinChange,
    UpSwingChange,
    DownSwingChange,
    None,
}

impl fmt::Display for SwingResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SwingResult::UpMinChange => write!(f, "Down within"),
            SwingResult::DownMinChange => write!(f, "Up within"),
            SwingResult::UpSwingChange => write!(f, "Down 30%"),
            SwingResult::DownSwingChange => write!(f, "Up 30%"),
            SwingResult::None => write!(f, "Flat"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StockDataWithData {
    pub stock_no: String,
    pub stock_data: StockData,
    pub swing_result: Option<SwingResult>,
    pub price_change_result: Option<PriceChangeResult>,
}

pub struct Company {
    pub stock_no: String,
    pub stock_data: Vec<StockData>,
}

impl fmt::Debug for Company {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Company {{ stock_no: {} }}", self.stock_no)
    }
}

impl Company {
    pub fn new(stock_no: String) -> Self {
        let mut stock_data = storage::read(&stock_no);
        stock_data.sort_by(|a, b| a.date.cmp(&b.date));

        Company {
            stock_no,
            stock_data,
        }
    }

    pub async fn fetch_year(&mut self, cfg: &CfgData, year: &str) {
        let mut stock_data_year = fugle::fetch(&cfg.fugle_api_key, &self.stock_no, year)
            .await
            .unwrap();
        stock_data_year.sort_by(|a, b| a.date.cmp(&b.date));
        storage::save(&self.stock_no, year, &stock_data_year);

        self.stock_data = storage::read(&self.stock_no);
        self.stock_data.sort_by(|a, b| a.date.cmp(&b.date));
    }

    pub fn get_index_by_date_range_backward(&self, date: &str, range: usize) -> Option<usize> {
        self.stock_data
            .iter()
            .position(|data| data.date == date)
            .and_then(|index| if index >= range { Some(index) } else { None })
    }

    pub fn get_index_by_date_range_forward(&self, date: &str, range: usize) -> Option<usize> {
        self.stock_data
            .iter()
            .position(|data| data.date == date)
            .and_then(|index| {
                if index + range < self.stock_data.len() {
                    Some(index)
                } else {
                    None
                }
            })
    }
}
