use crate::analysis;
use crate::common;
use crate::consts;
use crate::data::company::StockDataWithData;
use crate::data::stocks::Stocks;
use crate::scripts;

// const MODULE_NAME: &str = "scripts::candlestick";

pub fn find_long_red_candle_date(stocks: &mut Stocks, date: &str) -> Vec<StockDataWithData> {
    let mut results = Vec::new();

    for company in &stocks.company_map.company_list {
        let stock_company = stocks
            .companies
            .get(&company.stock_no)
            .expect("找不到股票資料");
        let curr_date_index = match stock_company
            .get_index_by_date_range_backward(&common::get_fugle_format(date), 2)
        {
            Some(index) => index,
            None => continue, // 如果找不到日期，跳過這家公司
        };
        let curr_stock_data = &stock_company.stock_data[curr_date_index];

        if analysis::candlestick::candlestick_type(curr_stock_data)
            == analysis::candlestick::CandlestickType::LongRedCandle
        {
            results.push(StockDataWithData {
                stock_no: company.stock_no.clone(),
                stock_data: curr_stock_data.clone(),
                ..Default::default()
            });
        }
    }

    results
}

pub fn find_doji_date(stocks: &mut Stocks, date: &str) -> Vec<StockDataWithData> {
    let mut results = Vec::new();

    for company in &stocks.company_map.company_list {
        let stock_company = stocks
            .companies
            .get(&company.stock_no)
            .expect("找不到股票資料");
        let curr_date_index = match stock_company
            .get_index_by_date_range_backward(&common::get_fugle_format(date), 2)
        {
            Some(index) => index,
            None => continue, // 如果找不到日期，跳過這家公司
        };
        let curr_stock_data = &stock_company.stock_data[curr_date_index];

        if analysis::candlestick::candlestick_type(curr_stock_data)
            == analysis::candlestick::CandlestickType::Doji
        {
            results.push(StockDataWithData {
                stock_no: company.stock_no.clone(),
                stock_data: curr_stock_data.clone(),
                ..Default::default()
            });
        }
    }

    results
}

// 1. 當天十字線
// 2. 前面 consts::RANGE_20_DAYS 天要有 swing (see get_swing_result())
pub fn find_doji_date_with_condition(stocks: &mut Stocks, date: &str) -> Vec<StockDataWithData> {
    let mut results = Vec::new();

    for company in &stocks.company_map.company_list {
        let stock_company = stocks
            .companies
            .get(&company.stock_no)
            .expect("找不到股票資料");
        let curr_date_index = match stock_company.get_index_by_date_range_backward(
            &common::get_fugle_format(date),
            consts::RANGE_20_DAYS + 1,
        ) {
            Some(index) => index,
            None => continue, // 如果找不到日期，跳過這家公司
        };
        let curr_stock_data = &stock_company.stock_data[curr_date_index];

        #[allow(clippy::collapsible_if)]
        if analysis::candlestick::candlestick_type(curr_stock_data)
            == analysis::candlestick::CandlestickType::Doji
        {
            if let Some(swing_result) =
                scripts::common::get_swing_result(stock_company, curr_stock_data)
            {
                results.push(StockDataWithData {
                    stock_no: company.stock_no.clone(),
                    stock_data: curr_stock_data.clone(),
                    swing_result: Some(swing_result),
                    ..Default::default()
                });
            }
        }
    }

    results
}
