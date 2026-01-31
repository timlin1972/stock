use std::io;

use crate::analysis;
use crate::common;
use crate::scripts;
use crate::twse::company_map::CompanyMap;

pub fn print_line() {
    println!("--------------------------------------------------------------------------------");
}

pub async fn main_menu(company_map: &CompanyMap) {
    loop {
        println!("Main Menu");
        println!("1. 抓取資料 (TWSE Data Fetch)");
        println!("2. 長紅 K 棒分析 (Long Red Candle Analysis)");
        println!("3. MACD 單日黃金交叉");
        println!("4. 單日大成交量分析 (Volume Larger Than Threshold Analysis)");
        println!(
            "5. MACD黃金交叉且成交量大的綜合分析範例 (MACD Golden Cross and Large Volume Combined Analysis)"
        );

        println!("q/e. 退出 (Quit/Exit)");
        println!("請輸入選項：");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("讀取失敗");

        // 去掉換行符號
        let input = input.trim();

        match input {
            "1" => menu_fetch_data(company_map).await,
            "2" => menu_long_red_candle_analysis(company_map).await,
            "3" => menu_macd_golden_cross_analysis(company_map).await,
            "4" => menu_volume_larger_analysis(company_map).await,
            "5" => menu_macd_golden_cross_volume_larger_analysis(company_map).await,
            "q" | "e" => {
                println!("退出程式");
                break;
            }
            _ => {
                println!("無效的選項，請重新輸入。");
            }
        }
    }
}

async fn menu_fetch_data(company_map: &CompanyMap) {
    println!("請輸入月份 (YYYYMM): ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("讀取失敗");

    // 去掉換行符號
    let input = input.trim();

    scripts::data::fetch_data_monthly_all_companies(&company_map, input).await;
}

async fn menu_long_red_candle_analysis(company_map: &CompanyMap) {
    println!("請輸入日期 (YYYYMMDD): ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("讀取失敗");

    // 去掉換行符號
    let input = input.trim();

    print_line();
    analysis::long_red_candle::anal_date_all_companies(&company_map, input).await;
    print_line();
}

async fn menu_macd_golden_cross_analysis(company_map: &CompanyMap) {
    println!("請輸入起始月份 (YYYYMM): ");
    let mut input_from = String::new();

    io::stdin().read_line(&mut input_from).expect("讀取失敗");

    // 去掉換行符號
    let input_from = input_from.trim();

    println!("請輸入結束月份 (YYYYMM): ");
    let mut input_to = String::new();

    io::stdin().read_line(&mut input_to).expect("讀取失敗");

    // 去掉換行符號
    let input_to = input_to.trim();

    println!("請輸入日期 (YYYYMMDD): ");
    let mut input_date = String::new();

    io::stdin().read_line(&mut input_date).expect("讀取失敗");

    // 去掉換行符號
    let input_date = input_date.trim();

    print_line();

    let crosses =
        scripts::macd::anal_date_all_companies(&company_map, input_from, input_to, input_date)
            .await;

    for cross in &crosses {
        if cross.cross_type == analysis::macd::MacdCrossType::GoldenCross {
            cross.print(&company_map);
        }
    }
    print_line();
}

async fn menu_volume_larger_analysis(company_map: &CompanyMap) {
    println!("請輸入日期 (YYYYMMDD): ");
    let mut input_date = String::new();

    io::stdin().read_line(&mut input_date).expect("讀取失敗");

    // 去掉換行符號
    let input_date = input_date.trim();

    println!("請輸入成交量閾值 (數字): ");
    let mut input_threshold = String::new();

    io::stdin()
        .read_line(&mut input_threshold)
        .expect("讀取失敗");

    // 去掉換行符號
    let input_threshold = input_threshold.trim();

    let volume_threshold: u64 = match input_threshold.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("無效的數字，請重新輸入。");
            return;
        }
    };

    print_line();

    let volume_results =
        scripts::volume::volume_larger_than_threshold(&company_map, volume_threshold, input_date)
            .await;
    println!(
        "{:<8}{:<6}{:>4}{:>5}{:>5}{:>5}{:>5}{:>6} {}",
        "日期", "股號", "成交股數", "開盤價", "收盤價", "最高價", "最低價", "漲跌", "公司名稱"
    );
    for volume_result in &volume_results {
        volume_result
            .daily_data
            .print(&company_map, &volume_result.stock_no);
    }

    print_line();
}

async fn menu_macd_golden_cross_volume_larger_analysis(company_map: &CompanyMap) {
    println!("請輸入起始月份 (YYYYMM): ");
    let mut input_from = String::new();

    io::stdin().read_line(&mut input_from).expect("讀取失敗");

    // 去掉換行符號
    let input_from = input_from.trim();

    println!("請輸入結束月份 (YYYYMM): ");
    let mut input_to = String::new();

    io::stdin().read_line(&mut input_to).expect("讀取失敗");

    // 去掉換行符號
    let input_to = input_to.trim();

    println!("請輸入日期 (YYYYMMDD): ");
    let mut input_date = String::new();

    io::stdin().read_line(&mut input_date).expect("讀取失敗");

    // 去掉換行符號
    let input_date = input_date.trim();

    println!("請輸入成交量閾值 (數字): ");
    let mut input_threshold = String::new();

    io::stdin()
        .read_line(&mut input_threshold)
        .expect("讀取失敗");

    // 去掉換行符號
    let input_threshold = input_threshold.trim();

    let volume_threshold: u64 = match input_threshold.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("無效的數字，請重新輸入。");
            return;
        }
    };

    print_line();

    println!(
        "{} MACD 黃金交叉且成交量大:",
        common::to_roc_date(input_date)
    );

    let results = scripts::complex::anal_macd_golden_volume_larger_date(
        company_map,
        input_from,
        input_to,
        input_date,
        volume_threshold,
    )
    .await;

    println!(
        "{:<8} {:<10} {:<4} {}",
        "股號", "日期", "成交張數", "公司名稱"
    );
    for result in &results {
        result.print(&company_map);
    }

    print_line();
}
