use crate::conditions;
use crate::data::stocks::Stocks;
use crate::menu::common::{
    get_choice, get_date_input, print_line, print_lower_upper_30_percent_list,
};

const BREAKAWAY_HELP: &str = "突破缺口";

// Common Gap
// Breakaway Gap
// Runaway Gap / Continuation Gap
// Exhaustion Gap
pub fn menu(stocks: &mut Stocks) {
    loop {
        print_line();
        println!("缺口選單");
        print_line();
        println!("  1. {BREAKAWAY_HELP}");
        println!("  0/q/e. 退出 (Quit/Exit)");

        let choice = get_choice();

        match choice.as_str() {
            "1" => menu_breakaway_gap(stocks),
            "0" | "q" | "e" => break,
            _ => println!("無效的選項，請重新輸入"),
        }
    }
}

fn menu_breakaway_gap(stocks: &mut Stocks) {
    let input = get_date_input();

    // set conditions
    let mut conditions = conditions::Conditions::new(&input);
    conditions.add_condition(conditions::Condition::BreakawayGap);
    conditions.run(stocks);
    let results = conditions.get_results();

    print_line();
    println!(
        "總共有 {} 支股票在 {} 是 {BREAKAWAY_HELP}",
        results.len(),
        input
    );
    print_lower_upper_30_percent_list(stocks, results);
    print_line();
}
