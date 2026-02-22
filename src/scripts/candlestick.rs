use crate::analysis;
use crate::common;
use crate::data::company::StockDataWithNo;
use crate::data::stocks::Stocks;

// const MODULE_NAME: &str = "scripts::candlestick";

pub fn find_long_red_candle_date(stocks: &mut Stocks, date: &str) -> Vec<StockDataWithNo> {
    let mut results = Vec::new();

    for company in &stocks.company_map.company_list {
        let stock_company = stocks
            .companies
            .get(&company.stock_no)
            .expect("找不到股票資料");
        let curr_date_index =
            match stock_company.get_index_by_date_range(&common::get_fugle_format(date), 2) {
                Some(index) => index,
                None => continue, // 如果找不到日期，跳過這家公司
            };
        let curr_stock_data = &stock_company.stock_data[curr_date_index];

        if analysis::candlestick::candlestick_type(curr_stock_data)
            == analysis::candlestick::CandlestickType::LongRedCandle
        {
            results.push(StockDataWithNo {
                stock_no: company.stock_no.clone(),
                stock_data: curr_stock_data.clone(),
            });
        }
    }

    results
}

pub fn find_doji_date(stocks: &mut Stocks, date: &str) -> Vec<StockDataWithNo> {
    let mut results = Vec::new();

    for company in &stocks.company_map.company_list {
        let stock_company = stocks
            .companies
            .get(&company.stock_no)
            .expect("找不到股票資料");
        let curr_date_index =
            match stock_company.get_index_by_date_range(&common::get_fugle_format(date), 2) {
                Some(index) => index,
                None => continue, // 如果找不到日期，跳過這家公司
            };
        let curr_stock_data = &stock_company.stock_data[curr_date_index];

        if analysis::candlestick::candlestick_type(curr_stock_data)
            == analysis::candlestick::CandlestickType::Doji
        {
            results.push(StockDataWithNo {
                stock_no: company.stock_no.clone(),
                stock_data: curr_stock_data.clone(),
            });
        }
    }

    results
}

pub fn find_doji_date_with_condition(
    stocks: &mut Stocks,
    date: &str,
    range: usize,
    min_change_percent: f64,
) -> Vec<StockDataWithNo> {
    let mut results = Vec::new();

    for company in &stocks.company_map.company_list {
        let stock_company = stocks
            .companies
            .get(&company.stock_no)
            .expect("找不到股票資料");
        let curr_date_index = match stock_company
            .get_index_by_date_range(&common::get_fugle_format(date), range + 1)
        {
            Some(index) => index,
            None => continue, // 如果找不到日期，跳過這家公司
        };
        let curr_stock_data = &stock_company.stock_data[curr_date_index];

        #[allow(clippy::collapsible_if)]
        if analysis::candlestick::candlestick_type(curr_stock_data)
            == analysis::candlestick::CandlestickType::Doji
        {
            if let Some((max_price, min_price)) =
                analysis::price::find_max_min_date_range(stock_company, date, range)
            {
                if max_price > curr_stock_data.close * (1.0 + min_change_percent)
                    || min_price < curr_stock_data.close * (1.0 - min_change_percent)
                {
                    results.push(StockDataWithNo {
                        stock_no: company.stock_no.clone(),
                        stock_data: curr_stock_data.clone(),
                    });
                }
            }
        }
    }

    results
}
