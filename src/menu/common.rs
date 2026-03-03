use std::io;
use std::io::Write;

use chrono::Local;

use crate::common;
use crate::consts;
use crate::data::company::StockDataWithData;
use crate::data::stocks::Stocks;

pub fn print_line() {
    println!("--------------------------------------------------------------------------------");
}

pub fn get_choice() -> String {
    print!("請輸入選項: ");
    io::stdout().flush().expect("刷新失敗");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("讀取失敗");
    input.trim().to_string()
}

pub fn get_date_input() -> String {
    let today = get_today_date();
    print!("請輸入日期 ({today}): ");
    io::stdout().flush().expect("刷新失敗");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("讀取失敗");
    let mut input = input.trim().to_string();
    if input.is_empty() {
        input = today;
    }
    input
}

// get YYYYMMDD format of today's date
fn get_today_date() -> String {
    let today = Local::now().date_naive();
    today.format("%Y%m%d").to_string()
}

pub fn print_detail_list(stock: &Stocks, results: &[StockDataWithData]) {
    println!(
        "{:<9}{:<5}{:>6}{:>6}{:>6}{:>6}{:>6}{:>6}  公司名稱",
        "日期", "台股", "成交張數", "開盤價", "收盤價", "最高價", "最低價", "漲跌",
    );
    for result in results {
        println!(
            "{:<11}{:<6}{:>10}{:>9.2}{:>9.2}{:>9.2}{:>9.2}{:>9.2}  {:<20}",
            result.stock_data.date,
            result.stock_no,
            common::str_volume(result.stock_data.volume),
            result.stock_data.open,
            result.stock_data.close,
            result.stock_data.high,
            result.stock_data.low,
            result.stock_data.change,
            stock.company_map.get_name(&result.stock_no),
        );
    }
}

pub fn print_lower_upper_30_percent_list(stock: &Stocks, results: &[StockDataWithData]) {
    println!(
        "{:<9}{:<5}{:>6}{:>6}{:>8}{:>8}  公司名稱",
        "日期", "台股", "成交張數", "收盤價", "+30%", "-30%",
    );
    for result in results {
        println!(
            "{:<11}{:<6}{:>10}{:>9.2}{:>9.2}{:>9.2}  {:<20}",
            result.stock_data.date,
            result.stock_no,
            common::str_volume(result.stock_data.volume),
            result.stock_data.close,
            result.stock_data.close * 1.3,
            result.stock_data.close * 0.7,
            stock.company_map.get_name(&result.stock_no),
        );
    }
}

pub fn print_upper_30_percent_volume_list(stock: &Stocks, results: &[StockDataWithData]) {
    println!("日期       台股    成交張數   收盤價   +30%     VolChg 公司名稱",);
    for result in results {
        println!(
            "{:<11}{:<6}{:>10}{:>9.2}{:>9.2}{:>9.2}  {:<20}",
            result.stock_data.date,
            result.stock_no,
            common::str_volume(result.stock_data.volume),
            result.stock_data.close,
            result.stock_data.close * consts::SWING_UP_RATIO,
            result.volume_change_result.unwrap(),
            stock.company_map.get_name(&result.stock_no),
        );
    }
}

pub fn print_lower_30_percent_volume_list(stock: &Stocks, results: &[StockDataWithData]) {
    println!("日期       台股    成交張數   收盤價   -30%     VolChg 公司名稱",);
    for result in results {
        println!(
            "{:<11}{:<6}{:>10}{:>9.2}{:>9.2}{:>9.2}  {:<20}",
            result.stock_data.date,
            result.stock_no,
            common::str_volume(result.stock_data.volume),
            result.stock_data.close,
            result.stock_data.close * consts::SWING_DOWN_RATIO,
            result.volume_change_result.unwrap(),
            stock.company_map.get_name(&result.stock_no),
        );
    }
}

pub fn print_volume_list(stock: &Stocks, results: &[StockDataWithData]) {
    println!("日期       台股    成交張數   收盤價   VolChg 公司名稱",);
    for result in results {
        println!(
            "{:<11}{:<6}{:>10}{:>9.2}{:>9.2}  {:<20}",
            result.stock_data.date,
            result.stock_no,
            common::str_volume(result.stock_data.volume),
            result.stock_data.close,
            result.volume_change_result.unwrap(),
            stock.company_map.get_name(&result.stock_no),
        );
    }
}

pub fn print_lower_30_percent_list(stock: &Stocks, results: &[StockDataWithData]) {
    println!("日期       台股    成交張數   收盤價   -30%     公司名稱",);
    for result in results {
        println!(
            "{:<11}{:<6}{:>10}{:>9.2}{:>9.2}  {:<20}",
            result.stock_data.date,
            result.stock_no,
            common::str_volume(result.stock_data.volume),
            result.stock_data.close,
            result.stock_data.close * consts::SWING_DOWN_RATIO,
            stock.company_map.get_name(&result.stock_no),
        );
    }
}

pub fn print_upper_30_percent_list(stock: &Stocks, results: &[StockDataWithData]) {
    println!("日期       台股    成交張數   收盤價   +30%     公司名稱",);
    for result in results {
        println!(
            "{:<11}{:<6}{:>10}{:>9.2}{:>9.2}  {:<20}",
            result.stock_data.date,
            result.stock_no,
            common::str_volume(result.stock_data.volume),
            result.stock_data.close,
            result.stock_data.close * consts::SWING_UP_RATIO,
            stock.company_map.get_name(&result.stock_no),
        );
    }
}

pub fn print_macd_list(stock: &Stocks, results: &[StockDataWithData]) {
    println!(
        "日期       台股    成交張數   收盤價     MACD   Signal  MACD交叉                 公司名稱",
    );
    for result in results {
        println!(
            "{:<11}{:<6}{:>10}{:>9.2} {:>8.2} {:>8.2}  {:<15} {:<20}",
            result.stock_data.date,
            result.stock_no,
            common::str_volume(result.stock_data.volume),
            result.stock_data.close,
            result.macd_result.as_ref().unwrap().dif,
            result.macd_result.as_ref().unwrap().signal,
            result.macd_result.as_ref().unwrap().macd_cross.to_string(),
            stock.company_map.get_name(&result.stock_no),
        );
    }
}
