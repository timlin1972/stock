use crate::common;
use crate::data::company::StockDataWithData;
use crate::data::stocks::Stocks;
use crate::scripts;

pub fn find_doji(stocks: &mut Stocks, date_list: &[String], year: &str) -> Vec<StockDataWithData> {
    let mut results = Vec::new();

    // for date_list starting from year
    for date in date_list.iter().filter(|d| d.starts_with(year)) {
        let yyyymmdd_date = common::get_yyyymmdd_format(date);
        let results_date =
            scripts::candlestick::find_doji_date_with_condition(stocks, &yyyymmdd_date);
        results.extend(results_date);
    }

    results
}
