use crate::data::company::Company;

const MODULE_NAME: &str = "analysis::volume";
const RANGE_5_DAYS: usize = 5;
const RANGE_10_DAYS: usize = 10;
const RANGE_20_DAYS: usize = 20;

// 找出 curr_date_index 前一日的均量 (MA5, MA10, MA20)
pub fn find_prev_date_mv(
    stock_company: &Company,
    curr_date_index: usize,
) -> Option<(f64, f64, f64)> {
    if curr_date_index >= stock_company.stock_data.len() || curr_date_index < RANGE_20_DAYS {
        println!(
            "[{MODULE_NAME}] 找不到 {} 日期索引 {} 的資料 或資料不足",
            stock_company.stock_no, curr_date_index
        );
        return None;
    }

    let mut sum_mv5 = 0;
    let mut sum_mv10 = 0;
    let mut sum_mv20 = 0;

    for i in curr_date_index - RANGE_20_DAYS..=curr_date_index - 1 {
        let data = &stock_company.stock_data.get(i).expect("找不到日期");
        if i >= curr_date_index - RANGE_5_DAYS {
            sum_mv5 += data.volume;
        }
        if i >= curr_date_index - RANGE_10_DAYS {
            sum_mv10 += data.volume;
        }
        sum_mv20 += data.volume;
    }
    Some((
        sum_mv5 as f64 / RANGE_5_DAYS as f64,
        sum_mv10 as f64 / RANGE_10_DAYS as f64,
        sum_mv20 as f64 / RANGE_20_DAYS as f64,
    ))
}
