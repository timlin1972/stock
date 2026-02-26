use crate::data::stocks::Stocks;
use crate::menu::candlestick;
use crate::menu::common::{get_choice, print_line};
use crate::menu::midterm;
use crate::monitor;

pub async fn main_menu(stocks: &mut Stocks) {
    loop {
        print_line();
        println!("Main Menu");
        print_line();
        println!("  1. 抓 2026 全部股票資料");
        println!("  2. 追蹤個股");
        println!("  20. K線");
        println!("  30. 中期");
        println!("  0/q/e. 退出 (Quit/Exit)");

        let choice = get_choice();

        match choice.as_str() {
            "1" => menu_fetch_data_all_companies(stocks).await,
            "2" => menu_monitor(stocks),
            "20" => candlestick::menu(stocks),
            "30" => midterm::menu(stocks),
            "0" | "q" | "e" => break,
            _ => println!("無效的選項，請重新輸入"),
        }
    }
}

async fn menu_fetch_data_all_companies(stocks: &mut Stocks) {
    print_line();
    stocks.fetch_year("2026").await;
    print_line();
}

fn menu_monitor(stocks: &mut Stocks) {
    let date = crate::menu::common::get_date_input();

    let mut monitor_stock_list = monitor::MonitorStockList::new(&date);
    monitor_stock_list.run(stocks);

    print_line();
    monitor_stock_list.print(stocks);
    print_line();
}
