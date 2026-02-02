use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

use crate::analysis;
use crate::common;
use crate::data::monthly_data::MonthlyData;
use crate::scripts;
use crate::twse::company_map::CompanyMap;

pub fn print_line() {
    println!("--------------------------------------------------------------------------------");
}

pub async fn main_menu(company_map: &CompanyMap) {
    loop {
        println!("Main Menu");
        println!("每日工作: 1/2/6/8/10");
        println!("1. 抓取 TWSE 資料");
        println!("2. 整理資料");
        println!("3. 單日長紅 K 棒");
        println!("4. 單日 MACD 黃金交叉");
        println!("5. 單日大成交量");
        println!("6. 單日 MACD 黃金交叉且大成交量");
        println!("7. 單日十字線");
        println!("8. 單日十字線波段驗證");
        println!("9. 多日陽吞噬形態");
        println!("10. 單日陽吞噬形態");

        println!("q/e. 退出 (Quit/Exit)");
        println!("請輸入選項：");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("讀取失敗");

        // 去掉換行符號
        let input = input.trim();

        match input {
            "1" => menu_fetch_data(company_map).await,
            "2" => menu_refactor_data(company_map).await,
            "3" => menu_long_red_candle_analysis(company_map).await,
            "4" => menu_macd_golden_cross_analysis(company_map).await,
            "5" => menu_volume_larger_analysis(company_map).await,
            "6" => menu_macd_golden_cross_volume_larger_analysis(company_map).await,
            "7" => menu_doji_analysis(company_map).await,
            "8" => menu_doji_in_swing_analysis(company_map).await,
            "9" => menu_bullish_engulfing_analysis(company_map).await,
            "10" => menu_bullish_engulfing_analysis_date(company_map).await,
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

    scripts::data::fetch_data_monthly_all_companies(company_map, input).await;
}

async fn menu_refactor_data(_company_map: &CompanyMap) {
    let root = Path::new("data");

    // 遞迴走訪
    fn visit_dir(path: &Path) {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    // 如果是子目錄，印出目錄名稱
                    // println!("目錄: {}", p.display());
                    // 再遞迴進去
                    visit_dir(&p);
                } else {
                    // 如果是檔案，印出檔案名稱
                    let file = File::open(&p).unwrap();
                    let reader = std::io::BufReader::new(file);
                    let mut monthly_data =
                        serde_json::from_reader::<_, MonthlyData>(reader).unwrap();
                    let mut refactor = false;
                    for daily in &monthly_data.daily_data {
                        if daily.open == 0.0
                            || daily.high == 0.0
                            || daily.low == 0.0
                            || daily.close == 0.0
                        {
                            println!(
                                "將移除: date={} open={} high={} low={} close={}",
                                daily.date, daily.open, daily.high, daily.low, daily.close
                            );
                            refactor = true;
                        }
                    }
                    if refactor {
                        monthly_data.daily_data.retain(|d| {
                            d.open != 0.0 && d.high != 0.0 && d.low != 0.0 && d.close != 0.0
                        });
                        monthly_data.write_to_storage();
                    }
                }
            }
        }
    }

    visit_dir(root);
}

async fn menu_long_red_candle_analysis(company_map: &CompanyMap) {
    println!("請輸入日期 (YYYYMMDD): ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("讀取失敗");

    // 去掉換行符號
    let input = input.trim();

    print_line();
    analysis::long_red_candle::anal_date_all_companies(company_map, input).await;
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
        scripts::macd::anal_date_all_companies(company_map, input_from, input_to, input_date).await;

    for cross in &crosses {
        if cross.cross_type == analysis::macd::MacdCrossType::GoldenCross {
            cross.print(company_map);
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
        scripts::volume::volume_larger_than_threshold(company_map, volume_threshold, input_date)
            .await;
    println!(
        "{:<8}{:<6}{:>4}{:>5}{:>5}{:>5}{:>5}{:>6} 公司名稱",
        "日期", "股號", "成交股數", "開盤價", "收盤價", "最高價", "最低價", "漲跌",
    );
    for volume_result in &volume_results {
        volume_result
            .daily_data
            .print(company_map, &volume_result.stock_no);
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

    println!("{:<8} {:<10} {:<4} 公司名稱", "股號", "日期", "成交張數",);
    for result in &results {
        result.print(company_map);
    }

    print_line();
}

async fn menu_doji_analysis(company_map: &CompanyMap) {
    println!("請輸入日期 (YYYYMMDD): ");
    let mut input_date = String::new();

    io::stdin().read_line(&mut input_date).expect("讀取失敗");

    // 去掉換行符號
    let input_date = input_date.trim();

    print_line();

    let results = scripts::doji::anal_date_all_companies(company_map, input_date).await;

    println!(
        "{:<8}{:<6}{:>4}{:>5}{:>5}{:>5}{:>5}{:>6} 公司名稱",
        "日期", "股號", "成交股數", "開盤價", "收盤價", "最高價", "最低價", "漲跌",
    );
    for result in &results {
        result.print(company_map);
    }

    print_line();
}

async fn menu_doji_in_swing_analysis(company_map: &CompanyMap) {
    println!("請輸入起始月份 (YYYYMM): ");
    let mut input_from = String::new();
    io::stdin().read_line(&mut input_from).expect("讀取失敗");
    let input_from = input_from.trim();

    println!("請輸入結束月份 (YYYYMM): ");
    let mut input_to = String::new();
    io::stdin().read_line(&mut input_to).expect("讀取失敗");
    let input_to = input_to.trim();

    println!("請輸入日期 (YYYYMMDD): ");
    let mut input_date = String::new();
    io::stdin().read_line(&mut input_date).expect("讀取失敗");
    let input_date = input_date.trim();

    let results = scripts::complex::anal_doji_in_swing_all_companies(
        company_map,
        input_from,
        input_to,
        input_date,
    )
    .await;

    print_line();
    println!(
        "{:<8}{:<6}{:>6}{:>6}{:>6}{:>5}{:>5}{:>6}{:>6} 公司名稱",
        "日期", "股號", "現價", "最高價", "最低價", "高", "低", "最高價", "最低價",
    );

    for result in &results {
        println!(
            "{:<8} {:<6} {:>8} {:>8} {:>8} {:>5} {:>5} {:>8.2} {:>8.2} {:<6}",
            result.daily_data.date,
            result.stock_no,
            result.daily_data.close,
            result.highest_price,
            result.lowest_price,
            if result.meet_high { "是" } else { "否" },
            if result.meet_low { "是" } else { "否" },
            result.daily_data.close * 1.3,
            result.daily_data.close * 0.7,
            company_map.get(&result.stock_no)
        );
    }
    print_line();
}

async fn menu_bullish_engulfing_analysis(company_map: &CompanyMap) {
    println!("請輸入起始月份 (YYYYMM): ");
    let mut input_from = String::new();
    io::stdin().read_line(&mut input_from).expect("讀取失敗");
    let input_from = input_from.trim();

    println!("請輸入結束月份 (YYYYMM): ");
    let mut input_to = String::new();
    io::stdin().read_line(&mut input_to).expect("讀取失敗");
    let input_to = input_to.trim();

    let results = scripts::bullish_engulfing_pattern::anal_range_all_companies(
        company_map,
        input_from,
        input_to,
    )
    .await;

    print_line();
    println!("{:<8}{:<6} 公司名稱", "日期", "股號",);

    for result in &results {
        println!(
            "{:<8} {:<6} {:<6}",
            result.date,
            result.stock_no,
            company_map.get(&result.stock_no)
        );
    }
    print_line();
}

async fn menu_bullish_engulfing_analysis_date(company_map: &CompanyMap) {
    println!("請輸入日期 (YYYYMMDD): ");
    let mut input_date = String::new();
    io::stdin().read_line(&mut input_date).expect("讀取失敗");
    let input_date = input_date.trim();

    let results =
        scripts::bullish_engulfing_pattern::anal_date_all_companies(company_map, input_date).await;

    print_line();
    println!("{:<8}{:<6} 公司名稱", "日期", "股號",);

    for result in &results {
        println!(
            "{:<8} {:<6} {:<6}",
            result.date,
            result.stock_no,
            company_map.get(&result.stock_no)
        );
    }
    print_line();
}
