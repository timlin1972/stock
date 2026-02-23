use crate::backtest;
use crate::consts;
use crate::data::stocks::Stocks;
use crate::menu::common::{get_choice, get_year_input, print_line, print_swing_price_list};

fn get_date_list(stocks: &mut Stocks) -> Vec<String> {
    // use 2317 as base stock to find date list
    let mut date_list = Vec::new();
    let stock_company = stocks.companies.get("2317").expect("找不到股票資料");

    for data in &stock_company.stock_data {
        date_list.push(data.date.clone());
    }

    date_list.sort();
    date_list
}

pub fn menu(stocks: &mut Stocks) {
    let date_list = get_date_list(stocks);

    loop {
        print_line();
        println!("回測選單");
        print_line();
        println!("  1. 十字線");
        println!("  q/e. 退出 (Quit/Exit)");

        let choice = get_choice();

        match choice.as_str() {
            "1" => menu_doji(stocks, &date_list),
            "q" | "e" => break,
            _ => println!("無效的選項，請重新輸入"),
        }
    }
}

fn menu_doji(stocks: &mut Stocks, date_list: &[String]) {
    let input = get_year_input();

    print_line();
    let mut results = backtest::candlestick::find_doji(stocks, date_list, &input);

    backtest::price::find_price_change(
        stocks,
        &mut results,
        consts::RANGE_K_DAYS,
        consts::PRICE_MIN_CHANGE_PERCENT,
    );

    results.retain(|result| result.price_change_result.is_some());
    results.retain(|result| result.stock_data.volume >= consts::VALID_VOLUME * 1000);

    println!("總共有 {} 支股票在 {} 是十字線", results.len(), input);
    print_swing_price_list(stocks, &results);
    print_line();
}
