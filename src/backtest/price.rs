use crate::data::company::PriceChangeResult;
use crate::data::company::StockDataWithData;
use crate::data::stocks::Stocks;

// 對 stock_data_list 中的每一筆資料，計算後面 range 天的漲跌幅，
// 如果漲跌幅大於 min_change_percent，result = up
// 如果漲跌幅小於 -min_change_percent，result = down
// 否則 result = none
pub fn find_price_change(
    stocks: &mut Stocks,
    stock_data_list: &mut Vec<StockDataWithData>,
    range: usize,
    min_change_percent: f64,
) {
    for stock_data in stock_data_list {
        let stock_company = stocks
            .companies
            .get(&stock_data.stock_no)
            .expect("找不到股票資料");
        let curr_date_index = match stock_company
            .get_index_by_date_range_forward(&stock_data.stock_data.date, range)
        {
            Some(index) => index,
            None => continue, // 如果找不到日期，跳過這筆資料
        };

        // let curr_stock_data = &stock_company.stock_data[curr_date_index];
        let curr_close = stock_data.stock_data.close;
        let up_price = (curr_close * (1.0 + min_change_percent)).floor();
        let down_price = (curr_close * (1.0 - min_change_percent)).ceil();

        stock_data.price_change_result = Some(PriceChangeResult::Flat); // 預設為 Flat
        for i in (curr_date_index + 1)..=curr_date_index + range {
            if i >= stock_company.stock_data.len() {
                panic!("超出資料範圍，停止檢查");
            }

            let future_stock_data = &stock_company.stock_data[i];

            if future_stock_data.close >= up_price {
                stock_data.price_change_result = Some(PriceChangeResult::Up);
                break;
            } else if future_stock_data.close <= down_price {
                stock_data.price_change_result = Some(PriceChangeResult::Down);
                break;
            }
        }
    }
}
