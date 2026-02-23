use crate::consts;
use crate::data::stocks::Stocks;
use crate::menu::common::{
    get_choice, get_date_input, print_detail_list, print_line, print_lower_upper_30_percent_list,
    print_swing_list,
};
use crate::scripts;

pub fn menu(stocks: &mut Stocks) {
    loop {
        print_line();
        println!("K線選單");
        print_line();
        println!("  1. 長紅K");
        println!("  2. 十字線");
        println!(
            "  3. 十字線 (前面 {} 天要有 {}% 以上的漲跌幅)",
            consts::RANGE_20_DAYS,
            consts::SWING_MIN_CHANGE_PERCENT * 100.0
        );
        println!("  4. 陽吞噬");
        println!("  q/e. 退出 (Quit/Exit)");

        let choice = get_choice();

        match choice.as_str() {
            "1" => menu_long_red_candle_date(stocks),
            "2" => menu_doji_date(stocks),
            "3" => menu_doji_date_with_condition(stocks),
            "4" => menu_bullish_engulfing_date(stocks),
            "q" | "e" => break,
            _ => println!("無效的選項，請重新輸入"),
        }
    }
}

fn menu_long_red_candle_date(stocks: &mut Stocks) {
    let input = get_date_input();

    print_line();
    let mut results = scripts::candlestick::find_long_red_candle_date(stocks, &input);
    results.sort_by(|a, b| b.stock_data.volume.cmp(&a.stock_data.volume)); // 按照成交量排序
    println!("總共有 {} 支股票在 {} 是長紅 K 棒", results.len(), input);
    print_detail_list(stocks, &results);
    print_line();
}

fn menu_doji_date(stocks: &mut Stocks) {
    let input = get_date_input();

    print_line();
    let mut results = scripts::candlestick::find_doji_date(stocks, &input);
    results.sort_by(|a, b| b.stock_data.volume.cmp(&a.stock_data.volume)); // 按照成交量排序
    println!("總共有 {} 支股票在 {} 是十字線", results.len(), input);
    print_lower_upper_30_percent_list(stocks, &results);
    print_line();
}

fn menu_doji_date_with_condition(stocks: &mut Stocks) {
    let input = get_date_input();

    let mut results = scripts::candlestick::find_doji_date_with_condition(stocks, &input);
    results.sort_by(|a, b| b.stock_data.volume.cmp(&a.stock_data.volume)); // 按照成交量排序

    print_line();
    println!(
        "總共有 {} 支股票在 {} 是十字線，且前面 {} 天有 {}% 以上的漲跌幅",
        results.len(),
        input,
        consts::RANGE_20_DAYS,
        consts::SWING_MIN_CHANGE_PERCENT * 100.0
    );
    print_swing_list(stocks, &results);
    print_line();
}

fn menu_bullish_engulfing_date(stocks: &mut Stocks) {
    let input = get_date_input();

    let results = scripts::bullish_engulfing::find_bullish_engulfing_date(stocks, &input);

    print_line();
    println!("總共有 {} 支股票在 {input} 是 陽吞噬形態", results.len());
    print_lower_upper_30_percent_list(stocks, &results);
    print_line();
}
