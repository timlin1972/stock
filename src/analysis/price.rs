use crate::data::company::Company;

const MODULE_NAME: &str = "analysis::price";
const RANGE_5_DAYS: usize = 5;
const RANGE_10_DAYS: usize = 10;
const RANGE_20_DAYS: usize = 20;

// 找出 curr_date_index 前一日的均價 (MA5, MA10, MA20)
pub fn find_prev_date_ma(
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

    let ma5 = stock_company.stock_data[curr_date_index - 1 - RANGE_5_DAYS + 1..curr_date_index]
        .iter()
        .map(|data| data.close)
        .sum::<f64>()
        / RANGE_5_DAYS as f64;

    let ma10 = stock_company.stock_data[curr_date_index - 1 - RANGE_10_DAYS + 1..curr_date_index]
        .iter()
        .map(|data| data.close)
        .sum::<f64>()
        / RANGE_10_DAYS as f64;

    let ma20 = stock_company.stock_data[curr_date_index - 1 - RANGE_20_DAYS + 1..curr_date_index]
        .iter()
        .map(|data| data.close)
        .sum::<f64>()
        / RANGE_20_DAYS as f64;

    Some((ma5, ma10, ma20))
}
