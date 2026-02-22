use crate::common;
use crate::data::company::Company;

// 從 date 開始往前數 range 天的資料，找出這段期間的最高價和最低價
// date 本身不算在內
pub fn find_max_min_date_range(
    stock_company: &Company,
    date: &str,
    range: usize,
) -> Option<(f64, f64)> {
    let curr_date_index =
        stock_company.get_index_by_date_range(&common::get_fugle_format(date), range)?;

    let start_index = curr_date_index - range;

    let mut max_price = f64::MIN;
    let mut min_price = f64::MAX;

    for i in start_index..curr_date_index {
        let data = &stock_company.stock_data[i];
        if data.high > max_price {
            max_price = data.high;
        }
        if data.low < min_price {
            min_price = data.low;
        }
    }

    Some((max_price, min_price))
}
