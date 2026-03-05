use crate::conditions;
use crate::data::stocks::Stocks;
use crate::menu::common::{
    get_choice, get_date_input, print_detail_list, print_line, print_lower_30_percent_list,
    print_lower_30_percent_volume_list, print_lower_upper_30_percent_list,
    print_upper_30_percent_list, print_upper_30_percent_volume_list, print_volume_list,
};

pub const LONG_RED_CANDLE_HELP: &str = "長紅K";
pub const LONG_UPPER_SHADOW_HELP: &str = "長上影線";
pub const DOJI_HELP: &str = "十字線";
pub const GRAVESTONE_DOJI_HELP: &str = "墓碑線";
pub const BULLISH_ENGULFING_HELP: &str = "陽吞噬 (爆量, 前面要有波段下跌)";
pub const BEARISH_ENGULFING_HELP: &str = "陰吞噬 (前面要有波段上漲)";
pub const DARK_CLOUD_COVER_HELP: &str = "烏雲罩頂 (前面要有波段下跌)";
pub const BULLISH_HARAMI_HELP: &str = "多頭母子 (前面要有波段下跌)";
pub const BEARISH_HARAMI_HELP: &str = "空頭母子 (前面要有波段上漲)";
pub const BULLISH_HARAMI_THREE_DAY_REVERSAL_HELP: &str = "內困三日翻紅 (前面要有波段下跌)";
pub const BEARISH_HARAMI_THREE_DAY_REVERSAL_HELP: &str = "內困三日翻黑 (前面要有波段上漲)";
pub const UPSIDE_GAP_TWO_CROWS_HELP: &str = "烏鴉躍空 (前面要有波段上漲)";
pub const THREE_WHITE_SOLDIERS_HELP: &str = "紅三兵 (看第三天有沒有爆量)";
pub const THREE_BLACK_CROWS_HELP: &str = "黑三兵";
pub const BULLISH_CHARIOT_HELP: &str = "多頭戰車 (前面要有漲一段)";
pub const BEARISH_CHARIOT_HELP: &str = "空頭戰車 (前面要有跌一段)";
pub const RISING_THREE_METHODS_HELP: &str = "連續排列: 上升三法";
pub const FALLING_THREE_METHODS_HELP: &str = "連續排列: 下降三法";
pub const HANGING_MAN_HELP: &str = "吊人線";

pub fn menu(stocks: &mut Stocks) {
    loop {
        print_line();
        println!("K線選單");
        print_line();
        println!("  1. {LONG_RED_CANDLE_HELP}");
        println!("  2. {LONG_UPPER_SHADOW_HELP}");
        println!("  3. {DOJI_HELP}");
        println!("  4. {GRAVESTONE_DOJI_HELP}");
        println!("  5. {BULLISH_ENGULFING_HELP}");
        println!("  6. {BEARISH_ENGULFING_HELP}");
        println!("  7. {DARK_CLOUD_COVER_HELP}");
        println!("  8. {BULLISH_HARAMI_HELP}");
        println!("  9. {BEARISH_HARAMI_HELP}");
        println!("  10. {BULLISH_HARAMI_THREE_DAY_REVERSAL_HELP}");
        println!("  11. {BEARISH_HARAMI_THREE_DAY_REVERSAL_HELP}");
        println!("  12. {UPSIDE_GAP_TWO_CROWS_HELP}");
        println!("  13. {THREE_WHITE_SOLDIERS_HELP}");
        println!("  14. {THREE_BLACK_CROWS_HELP}");
        println!("  15. {BULLISH_CHARIOT_HELP}");
        println!("  16. {BEARISH_CHARIOT_HELP}");
        println!("  17. {RISING_THREE_METHODS_HELP}");
        println!("  18. {FALLING_THREE_METHODS_HELP}");
        println!("  19. {HANGING_MAN_HELP}");
        println!("  0/q/e. 退出 (Quit/Exit)");

        let choice = get_choice();

        match choice.as_str() {
            "1" => menu_long_red_candle_date(stocks),
            "2" => menu_long_upper_shadow_date(stocks),
            "3" => menu_doji_date(stocks),
            "4" => menu_gravestone_doji_date(stocks),
            "5" => menu_bullish_engulfing_date(stocks),
            "6" => menu_bearish_engulfing_date(stocks),
            "7" => menu_dark_cloud_cover_date(stocks),
            "8" => menu_bullish_harami_date(stocks),
            "9" => menu_bearish_harami_date(stocks),
            "10" => menu_bullish_harami_three_day_reversal_date(stocks),
            "11" => menu_bearish_harami_three_day_reversal_date(stocks),
            "12" => menu_upside_gap_two_crows_date(stocks),
            "13" => menu_three_white_soldiers_date(stocks),
            "14" => menu_three_black_crows_date(stocks),
            "15" => menu_bullish_chariot_date(stocks),
            "16" => menu_bearish_chariot_date(stocks),
            "17" => menu_rising_three_methods_date(stocks),
            "18" => menu_falling_three_methods_date(stocks),
            "19" => menu_hanging_man_date(stocks),
            "0" | "q" | "e" => break,
            _ => println!("無效的選項，請重新輸入"),
        }
    }
}

fn menu_long_red_candle_date(stocks: &mut Stocks) {
    let input = get_date_input();

    let mut conditions =
        conditions::generate_conditions(&conditions::Condition::LongRedCandle, input.as_str());
    conditions.run(stocks);
    let mut results = conditions.get_results().clone();
    results.sort_by(|a, b| b.stock_data.volume.cmp(&a.stock_data.volume)); // 按照成交量排序

    print_line();
    println!(
        "總共有 {} 支股票在 {input} 是 {LONG_RED_CANDLE_HELP}",
        results.len(),
    );
    print_detail_list(stocks, &results);
    print_line();
}

fn menu_long_upper_shadow_date(stocks: &mut Stocks) {
    let input = get_date_input();

    let mut conditions =
        conditions::generate_conditions(&conditions::Condition::LongUpperShadow, input.as_str());
    conditions.run(stocks);
    let mut results = conditions.get_results().clone();
    results.sort_by(|a, b| b.stock_data.volume.cmp(&a.stock_data.volume)); // 按照成交量排序

    print_line();
    println!(
        "總共有 {} 支股票在 {input} 是 {LONG_UPPER_SHADOW_HELP}",
        results.len(),
    );
    print_detail_list(stocks, &results);
    print_line();
}

fn menu_doji_date(stocks: &mut Stocks) {
    let input = get_date_input();

    let mut conditions =
        conditions::generate_conditions(&conditions::Condition::Doji, input.as_str());
    conditions.run(stocks);
    let mut results = conditions.get_results().clone();
    results.sort_by(|a, b| b.stock_data.volume.cmp(&a.stock_data.volume)); // 按照成交量排序

    print_line();
    println!("總共有 {} 支股票在 {input} 是 {DOJI_HELP}", results.len(),);
    print_lower_upper_30_percent_list(stocks, &results);
    print_line();
}

fn menu_gravestone_doji_date(stocks: &mut Stocks) {
    let input = get_date_input();

    let mut conditions =
        conditions::generate_conditions(&conditions::Condition::GravestoneDoji, input.as_str());
    conditions.run(stocks);
    let mut results = conditions.get_results().clone();
    results.sort_by(|a, b| b.stock_data.volume.cmp(&a.stock_data.volume)); // 按照成交量排序

    print_line();
    println!(
        "總共有 {} 支股票在 {input} 是 {GRAVESTONE_DOJI_HELP}",
        results.len(),
    );
    print_lower_upper_30_percent_list(stocks, &results);
    print_line();
}

// 陽吞噬
fn menu_bullish_engulfing_date(stocks: &mut Stocks) {
    let input = get_date_input();

    let mut conditions =
        conditions::generate_conditions(&conditions::Condition::BullishEngulfing, input.as_str());
    conditions.run(stocks);
    let mut results = conditions.get_results().clone();
    results.sort_by(|a, b| b.stock_data.volume.cmp(&a.stock_data.volume)); // 按照成交量排序

    print_line();
    println!(
        "總共有 {} 支股票在 {input} 是 {BULLISH_ENGULFING_HELP}",
        results.len()
    );
    print_upper_30_percent_volume_list(stocks, &results);
    print_line();
}

// 陰吞噬
fn menu_bearish_engulfing_date(stocks: &mut Stocks) {
    let input = get_date_input();

    let mut conditions =
        conditions::generate_conditions(&conditions::Condition::BearishEngulfing, input.as_str());
    conditions.run(stocks);
    let mut results = conditions.get_results().clone();
    results.sort_by(|a, b| b.stock_data.volume.cmp(&a.stock_data.volume)); // 按照成交量排序

    print_line();
    println!(
        "總共有 {} 支股票在 {input} 是 {BEARISH_ENGULFING_HELP}",
        results.len()
    );
    print_lower_30_percent_list(stocks, &results);
    print_line();
}

// 烏雲罩頂
fn menu_dark_cloud_cover_date(stocks: &mut Stocks) {
    let input = get_date_input();

    let mut conditions =
        conditions::generate_conditions(&conditions::Condition::DarkCloudCover, input.as_str());
    conditions.run(stocks);
    let mut results = conditions.get_results().clone();
    // 按照成交量變化排序
    results.sort_by(|a, b| {
        let a_change = a.volume_change_result.unwrap();
        let b_change = b.volume_change_result.unwrap();
        b_change
            .partial_cmp(&a_change)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    print_line();
    println!(
        "總共有 {} 支股票在 {input} 是 {DARK_CLOUD_COVER_HELP}",
        results.len()
    );
    print_lower_30_percent_volume_list(stocks, &results);
    print_line();
}

// 多頭母子
fn menu_bullish_harami_date(stocks: &mut Stocks) {
    let input = get_date_input();

    let mut conditions =
        conditions::generate_conditions(&conditions::Condition::BullishHarami, input.as_str());
    conditions.run(stocks);
    let results = conditions.get_results().clone();

    print_line();
    println!(
        "總共有 {} 支股票在 {input} 是 {BULLISH_HARAMI_HELP}",
        results.len()
    );
    print_upper_30_percent_list(stocks, &results);
    print_line();
}

// 空頭母子
fn menu_bearish_harami_date(stocks: &mut Stocks) {
    let input = get_date_input();

    let mut conditions =
        conditions::generate_conditions(&conditions::Condition::BearishHarami, input.as_str());
    conditions.run(stocks);
    let results = conditions.get_results().clone();

    print_line();
    println!(
        "總共有 {} 支股票在 {input} 是 {BEARISH_HARAMI_HELP}",
        results.len()
    );
    print_lower_30_percent_list(stocks, &results);
    print_line();
}

// 內困三日翻紅
fn menu_bullish_harami_three_day_reversal_date(stocks: &mut Stocks) {
    let input = get_date_input();

    let mut conditions = conditions::generate_conditions(
        &conditions::Condition::BullishHaramiThreeDayReversal,
        input.as_str(),
    );
    conditions.run(stocks);
    let mut results = conditions.get_results().clone();
    // 按照成交量變化排序
    results.sort_by(|a, b| {
        let a_change = a.volume_change_result.unwrap();
        let b_change = b.volume_change_result.unwrap();
        b_change
            .partial_cmp(&a_change)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    print_line();
    println!(
        "總共有 {} 支股票在 {input} 是 {BULLISH_HARAMI_THREE_DAY_REVERSAL_HELP}",
        results.len()
    );
    print_upper_30_percent_volume_list(stocks, &results);
    print_line();
}

// 內困三日翻黑
fn menu_bearish_harami_three_day_reversal_date(stocks: &mut Stocks) {
    let input = get_date_input();

    let mut conditions = conditions::generate_conditions(
        &conditions::Condition::BearishHaramiThreeDayReversal,
        input.as_str(),
    );
    conditions.run(stocks);
    let mut results = conditions.get_results().clone();
    // 按照成交量變化排序
    results.sort_by(|a, b| {
        let a_change = a.volume_change_result.unwrap();
        let b_change = b.volume_change_result.unwrap();
        b_change
            .partial_cmp(&a_change)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    print_line();
    println!(
        "總共有 {} 支股票在 {input} 是 {BEARISH_HARAMI_THREE_DAY_REVERSAL_HELP}",
        results.len(),
    );
    print_lower_30_percent_volume_list(stocks, &results);
    print_line();
}

// 烏鴉躍空
fn menu_upside_gap_two_crows_date(stocks: &mut Stocks) {
    let input = get_date_input();

    let mut conditions =
        conditions::generate_conditions(&conditions::Condition::UpsideGapTwoCrows, input.as_str());
    conditions.run(stocks);
    let results = conditions.get_results().clone();

    print_line();
    println!(
        "總共有 {} 支股票在 {input} 是 {UPSIDE_GAP_TWO_CROWS_HELP}",
        results.len(),
    );
    print_lower_30_percent_list(stocks, &results);
    print_line();
}

// 紅三兵
fn menu_three_white_soldiers_date(_stocks: &mut Stocks) {
    let input = get_date_input();

    let mut conditions =
        conditions::generate_conditions(&conditions::Condition::ThreeWhiteSoldiers, input.as_str());
    conditions.run(_stocks);
    let results = conditions.get_results().clone();

    print_line();
    println!(
        "總共有 {} 支股票在 {input} 是 {THREE_WHITE_SOLDIERS_HELP}",
        results.len(),
    );
    print_volume_list(_stocks, &results);
    print_line();
}

// 黑三兵
fn menu_three_black_crows_date(_stocks: &mut Stocks) {
    println!("功能尚未實作");
}

// 多頭戰車
fn menu_bullish_chariot_date(_stocks: &mut Stocks) {
    println!("功能尚未實作");
}

// 空頭戰車
fn menu_bearish_chariot_date(_stocks: &mut Stocks) {
    println!("功能尚未實作");
}

// 連續排列: 上升三法
fn menu_rising_three_methods_date(_stocks: &mut Stocks) {
    println!("功能尚未實作");
}

// 連續排列: 下降三法
fn menu_falling_three_methods_date(_stocks: &mut Stocks) {
    println!("功能尚未實作");
}

// 吊人線
fn menu_hanging_man_date(stocks: &mut Stocks) {
    let input = get_date_input();

    let mut conditions =
        conditions::generate_conditions(&conditions::Condition::HangingMan, input.as_str());
    conditions.run(stocks);
    let results = conditions.get_results().clone();

    print_line();
    println!(
        "總共有 {} 支股票在 {input} 是 {HANGING_MAN_HELP}",
        results.len()
    );
    print_lower_upper_30_percent_list(stocks, &results);
    print_line();
}
