use crate::analysis;
use crate::common;
use crate::consts;
use crate::data::company::Company;
use crate::data::company::{StockData, SwingResult};

// 在 date 的前面 consts::RANGE_20_DAYS 天內，如果有
// 1. 漲跌幅超過 consts::SWING_CHANGE_PERCENT (30%)
//      ==> SwingResult::UpSwingChange / DownSwingChange
// 2. 漲跌幅超過 consts::SWING_MIN_CHANGE_PERCENT (15%)
//      ==> SwingResult::UpMinChange / DownMinChange
pub fn get_swing_result(
    stock_company: &Company,
    curr_stock_data: &StockData,
) -> Option<SwingResult> {
    let date = common::get_yyyymmdd_format(&curr_stock_data.date);
    if let Some((max_price, min_price)) =
        analysis::price::find_max_min_date_range(stock_company, &date, consts::RANGE_20_DAYS)
    {
        if max_price > curr_stock_data.close * (1.0 + consts::SWING_CHANGE_PERCENT) {
            return Some(SwingResult::UpSwingChange);
        }
        if min_price < curr_stock_data.close * (1.0 - consts::SWING_CHANGE_PERCENT) {
            return Some(SwingResult::DownSwingChange);
        }

        if max_price > curr_stock_data.close * (1.0 + consts::SWING_MIN_CHANGE_PERCENT) {
            return Some(SwingResult::UpMinChange);
        }

        if min_price < curr_stock_data.close * (1.0 - consts::SWING_MIN_CHANGE_PERCENT) {
            return Some(SwingResult::DownMinChange);
        }
    }

    None
}
