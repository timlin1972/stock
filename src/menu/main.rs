use crate::data::stocks::Stocks;
use crate::menu::candlestick;
use crate::menu::common::{get_choice, print_line};

pub async fn main_menu(stocks: &mut Stocks) {
    loop {
        print_line();
        println!("Main Menu");
        print_line();
        println!("  1. 抓 2026 全部股票資料");
        println!("  20. K線");
        println!("  q/e. 退出 (Quit/Exit)");

        let choice = get_choice();

        match choice.as_str() {
            "1" => menu_fetch_data_all_companies(stocks).await,
            "20" => candlestick::menu(stocks),
            "q" | "e" => break,
            _ => println!("無效的選項，請重新輸入"),
        }
    }
}

async fn menu_fetch_data_all_companies(stocks: &mut Stocks) {
    print_line();
    stocks.fetch_year("2026").await;
    print_line();
}
