use crate::common;
use crate::consts;
use crate::data::company::{StockData, StockDataWithData, SwingResult};
use crate::data::stocks::Stocks;
use crate::scripts;

// 不用量，不看影線
// 多頭母子: 波段下跌後，紅K躲在黑K的實體內
// 母子線為強烈多空反轉之K線排列
pub fn find_bullish_harami_date(stocks: &mut Stocks, date: &str) -> Vec<StockDataWithData> {
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

        if !is_bullish_harami(prev_stock_data, curr_stock_data) {
            continue;
        }

        #[allow(clippy::collapsible_if)]
        if let Some(swing_result) =
            scripts::common::get_swing_result(stock_company, curr_stock_data)
        {
            if swing_result == SwingResult::UpSwingChange {
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

fn is_bullish_harami(prev: &StockData, curr: &StockData) -> bool {
    // 當天是紅K
    if curr.close <= curr.open {
        return false;
    }
    // 昨天是黑K
    if prev.close >= prev.open {
        return false;
    }

    // 紅K實體被黑K實體包覆
    if curr.open <= prev.close || curr.close >= prev.open {
        return false;
    }

    true
}
