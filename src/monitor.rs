use crate::common;
use crate::conditions;
use crate::data::stocks::Stocks;
use crate::menu::candlestick;

use serde::{Deserialize, Serialize};

const MONITOR_STOCK_LIST_FILE: &str = "monitor_stock_list.json";

#[derive(Serialize, Deserialize, Default)]
struct MonitorStockSource {
    stock_no: String,
    buy_price: Option<f64>,
}

#[derive(Serialize, Deserialize, Default)]
struct MonitorStockResult {
    gain_loss: f64,
    long_red_candle: bool,
    long_upper_shadow: bool,
    doji: bool,
    gravestone_doji: bool,
    bullish_engulfing: bool,
    bearish_engulfing: bool,
    dark_cloud_cover: bool,
    bullish_harami: bool,
    bearish_harami: bool,
    bullish_harami_three_day_reversal: bool,
    bearish_harami_three_day_reversal: bool,
    upside_gap_two_crows: bool,
    three_white_soldiers: bool,
    hanging_man: bool,
}

#[derive(Serialize, Deserialize, Default)]
struct MonitorStock {
    source: MonitorStockSource,
    result: Option<MonitorStockResult>,
}

#[derive(Serialize, Deserialize)]
pub struct MonitorStockList {
    stocks: Vec<MonitorStock>,
    date: String,
}

impl MonitorStockList {
    pub fn new(date: &str) -> Self {
        let source = read_from_file();
        let stocks = source
            .into_iter()
            .map(|s| MonitorStock {
                source: s,
                ..Default::default()
            })
            .collect();

        MonitorStockList {
            stocks,
            date: date.to_string(),
        }
    }

    // pub fn write_to_file(&self) {
    //     let json = serde_json::to_string_pretty(&self.stocks).expect("序列化失敗");
    //     std::fs::write(MONITOR_STOCK_LIST_FILE, json).expect("寫入檔案失敗");
    // }

    pub fn run(&mut self, stocks: &Stocks) {
        for stock in &mut self.stocks {
            let stock_company = stocks
                .companies
                .get(&stock.source.stock_no)
                .expect("找不到股票資料");
            let date_fugle = common::get_fugle_format(&self.date);
            let curr_date_index = stock_company
                .get_index_by_date(&date_fugle)
                .expect("找不到日期");

            let curr_stock_data = &stock_company
                .stock_data
                .get(curr_date_index)
                .expect("找不到日期");

            stock.result = Some(MonitorStockResult {
                gain_loss: stock
                    .source
                    .buy_price
                    .map(|buy_price| (curr_stock_data.close - buy_price) / buy_price * 100.0)
                    .unwrap(),
                long_red_candle: run_general_condition(
                    stocks,
                    &stock.source.stock_no,
                    &self.date,
                    conditions::Condition::LongRedCandle,
                ),
                long_upper_shadow: run_general_condition(
                    stocks,
                    &stock.source.stock_no,
                    &self.date,
                    conditions::Condition::LongUpperShadow,
                ),
                doji: run_general_condition(
                    stocks,
                    &stock.source.stock_no,
                    &self.date,
                    conditions::Condition::Doji,
                ),
                gravestone_doji: run_general_condition(
                    stocks,
                    &stock.source.stock_no,
                    &self.date,
                    conditions::Condition::GravestoneDoji,
                ),
                bullish_engulfing: run_general_condition(
                    stocks,
                    &stock.source.stock_no,
                    &self.date,
                    conditions::Condition::BullishEngulfing,
                ),
                bearish_engulfing: run_general_condition(
                    stocks,
                    &stock.source.stock_no,
                    &self.date,
                    conditions::Condition::BearishEngulfing,
                ),
                dark_cloud_cover: run_general_condition(
                    stocks,
                    &stock.source.stock_no,
                    &self.date,
                    conditions::Condition::DarkCloudCover,
                ),
                bullish_harami: run_general_condition(
                    stocks,
                    &stock.source.stock_no,
                    &self.date,
                    conditions::Condition::BullishHarami,
                ),
                bearish_harami: run_general_condition(
                    stocks,
                    &stock.source.stock_no,
                    &self.date,
                    conditions::Condition::BearishHarami,
                ),
                bullish_harami_three_day_reversal: run_general_condition(
                    stocks,
                    &stock.source.stock_no,
                    &self.date,
                    conditions::Condition::BullishHaramiThreeDayReversal,
                ),
                bearish_harami_three_day_reversal: run_general_condition(
                    stocks,
                    &stock.source.stock_no,
                    &self.date,
                    conditions::Condition::BearishHaramiThreeDayReversal,
                ),
                upside_gap_two_crows: run_general_condition(
                    stocks,
                    &stock.source.stock_no,
                    &self.date,
                    conditions::Condition::UpsideGapTwoCrows,
                ),
                three_white_soldiers: run_general_condition(
                    stocks,
                    &stock.source.stock_no,
                    &self.date,
                    conditions::Condition::ThreeWhiteSoldiers,
                ),
                hanging_man: run_general_condition(
                    stocks,
                    &stock.source.stock_no,
                    &self.date,
                    conditions::Condition::HangingMan,
                ),
            });
        }
    }

    pub fn print(&self, stocks: &Stocks) {
        println!("追蹤日期: {}", self.date);
        for stock in &self.stocks {
            println!(
                "股票: {} {}, {} => {:3.2}%",
                stock.source.stock_no,
                stocks.company_map.get_name(&stock.source.stock_no),
                stock
                    .source
                    .buy_price
                    .map_or("N/A".to_string(), |p| format!("{:.2}", p)),
                stock.result.as_ref().unwrap().gain_loss
            );
            println!(
                "    {}: {}",
                candlestick::LONG_RED_CANDLE_HELP,
                stock.result.as_ref().unwrap().long_red_candle
            );
            println!(
                "    {}: {}",
                candlestick::LONG_UPPER_SHADOW_HELP,
                stock.result.as_ref().unwrap().long_upper_shadow
            );
            println!(
                "    {}: {}",
                candlestick::DOJI_HELP,
                stock.result.as_ref().unwrap().doji
            );
            println!(
                "    {}: {}",
                candlestick::GRAVESTONE_DOJI_HELP,
                stock.result.as_ref().unwrap().gravestone_doji
            );
            println!(
                "    {}: {}",
                candlestick::BULLISH_ENGULFING_HELP,
                stock.result.as_ref().unwrap().bullish_engulfing
            );
            println!(
                "    {}: {}",
                candlestick::BEARISH_ENGULFING_HELP,
                stock.result.as_ref().unwrap().bearish_engulfing
            );
            println!(
                "    {}: {}",
                candlestick::DARK_CLOUD_COVER_HELP,
                stock.result.as_ref().unwrap().dark_cloud_cover
            );
            println!(
                "    {}: {}",
                candlestick::BULLISH_HARAMI_HELP,
                stock.result.as_ref().unwrap().bullish_harami
            );
            println!(
                "    {}: {}",
                candlestick::BEARISH_HARAMI_HELP,
                stock.result.as_ref().unwrap().bearish_harami
            );
            println!(
                "    {}: {}",
                candlestick::BULLISH_HARAMI_THREE_DAY_REVERSAL_HELP,
                stock
                    .result
                    .as_ref()
                    .unwrap()
                    .bullish_harami_three_day_reversal
            );
            println!(
                "    {}: {}",
                candlestick::BEARISH_HARAMI_THREE_DAY_REVERSAL_HELP,
                stock
                    .result
                    .as_ref()
                    .unwrap()
                    .bearish_harami_three_day_reversal
            );
            println!(
                "    {}: {}",
                candlestick::UPSIDE_GAP_TWO_CROWS_HELP,
                stock.result.as_ref().unwrap().upside_gap_two_crows
            );
            println!(
                "    {}: {}",
                candlestick::THREE_WHITE_SOLDIERS_HELP,
                stock.result.as_ref().unwrap().three_white_soldiers
            );
            println!(
                "    {}: {}",
                candlestick::HANGING_MAN_HELP,
                stock.result.as_ref().unwrap().hanging_man
            );
        }
    }
}

fn read_from_file() -> Vec<MonitorStockSource> {
    // 從 JSON 檔案讀取
    let json = std::fs::read_to_string(MONITOR_STOCK_LIST_FILE).expect("讀取檔案失敗");
    serde_json::from_str(&json).expect("反序列化失敗")
}

fn run_general_condition(
    stocks: &Stocks,
    stock_no: &str,
    date: &str,
    condition: conditions::Condition,
) -> bool {
    let mut conditions = conditions::generate_conditions(&condition, date);
    let result = conditions.run_single(stocks, stock_no, date);

    result.is_some()
}
