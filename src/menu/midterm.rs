use crate::conditions;
use crate::data::stocks::Stocks;
use crate::menu::common::{get_choice, get_date_input, print_line, print_macd_list};

const MACD_GOLDEN_CROSS_HELP: &str = "MACD 黃金交叉且有 2000 張以上的成交量";

pub fn menu(stocks: &mut Stocks) {
    loop {
        print_line();
        println!("中期選單");
        print_line();
        println!("  1. {MACD_GOLDEN_CROSS_HELP}");
        println!("  0/q/e. 退出 (Quit/Exit)");

        let choice = get_choice();

        match choice.as_str() {
            "1" => menu_macd_golden_cross_date(stocks),
            "0" | "q" | "e" => break,
            _ => println!("無效的選項，請重新輸入"),
        }
    }
}

fn menu_macd_golden_cross_date(stocks: &mut Stocks) {
    let input = get_date_input();

    // set conditions
    let mut conditions = conditions::Conditions::new(&input);
    conditions.add_condition(conditions::Condition::MacdGoldenCross);
    conditions.add_condition(conditions::Condition::Volume { volume: 2000 }); // 成交量大於等於 2000 張
    conditions.run(stocks);
    let mut results = conditions.get_results().clone();
    results.sort_by(|a, b| {
        b.macd_result
            .clone()
            .unwrap()
            .macd_cross
            .cmp(&a.macd_result.clone().unwrap().macd_cross)
            .then_with(|| b.stock_data.volume.cmp(&a.stock_data.volume)) // 按照成交量排序
    });

    print_line();
    println!(
        "總共有 {} 支股票在 {} 是 {MACD_GOLDEN_CROSS_HELP}",
        results.len(),
        input
    );
    print_macd_list(stocks, &results);
    print_line();
}
