use crate::common;
use crate::consts;
use crate::data::company::{StockData, StockDataWithData};
use crate::data::stocks::Stocks;

fn is_bullish_engulfing(prev: &StockData, curr: &StockData) -> bool {
    // 前一天是黑K
    let prev_black = prev.close < prev.open;

    // 當天是紅K
    let curr_red = curr.close > curr.open;

    // 當天的實體完全包覆前一天的實體和影線
    let engulf = curr.open < prev.low && curr.close > prev.high;

    prev_black && curr_red && engulf
}

// 1. 當天陽吞噬
//      ==> 第一根 K 線是 黑K（跌），實體較小。
//      ==> 第二根 K 線是 紅K（漲），實體較大，且完全包覆前一天的黑K實體 (含影線)。
// 2. 前面 range 天要有 swing，必須是 UpSwingChange
// 4. 波段下跌創新低
// 5. 要有量，越大越好
pub fn find_bullish_engulfing_date(stocks: &mut Stocks, date: &str) -> Vec<StockDataWithData> {
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
        let prev_stock_data = &stock_company.stock_data[curr_date_index - 1];

        if !is_bullish_engulfing(prev_stock_data, curr_stock_data) {
            continue;
        }

        //     if let Some(swing_result) = get_swing_result(
        //         stock_company,
        //         date,
        //         range,
        //         min_change_percent,
        //         curr_stock_data,
        //     ) {
        //         results.push(StockDataWithData {
        //             stock_no: company.stock_no.clone(),
        //             stock_data: curr_stock_data.clone(),
        //             swing_result: Some(swing_result),
        //             ..Default::default()
        //         });
        //     }
    }

    results
}
