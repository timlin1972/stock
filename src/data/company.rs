use std::fmt;

use serde::{Deserialize, Serialize};

use crate::api::fugle;
use crate::cfg::CfgData;
use crate::storage;

#[derive(Debug, Serialize, Deserialize, Clone)]
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
pub struct StockDataWithNo {
    pub stock_no: String,
    pub stock_data: StockData,
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

    pub fn get_index_by_date_range(&self, date: &str, range: usize) -> Option<usize> {
        self.stock_data
            .iter()
            .position(|data| data.date == date)
            .and_then(|index| if index >= range { Some(index) } else { None })
    }
}
