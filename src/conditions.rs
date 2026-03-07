use crate::analysis;
use crate::common;
use crate::consts;
use crate::data::company::Company;
use crate::data::company::StockDataWithData;
use crate::data::stocks::Stocks;

pub enum Condition {
    LongRedCandle,
    LongUpperShadow,
    GravestoneDoji,
    Doji,
    HangingMan,
    Volume { volume: u64 }, // 成交量大於等於 volume (單位: 張)
    MacdGoldenCross,
    BullishEngulfing,                         // 陽吞噬
    BearishEngulfing,                         // 陰吞噬
    VolumeSpike { ratio: f64 },               // 成交量較前一天均量的變化
    PriceReach { period: usize, ratio: f64 }, // 過去 period 天，價格有碰到當天收盤價的 ratio (例如 1.05 表示 5% 的區間)
    DarkCloudCover,                           // 烏雲罩頂
    BullishHarami,                            // 多頭母子
    BearishHarami,                            // 空頭母子
    BullishHaramiThreeDayReversal,            // 內困三日翻紅
    BearishHaramiThreeDayReversal,            // 內困三日翻黑
    UpsideGapTwoCrows,                        // 烏鴉躍空
    ThreeWhiteSoldiers,                       // 紅三兵
    BreakawayGap,                             // 突破缺口
}

pub struct Conditions {
    period_id: String,
    conditions: Vec<Condition>,
    results: Vec<StockDataWithData>,
}

// period_id: could be yyyy, yyyymm, yyyymmdd
impl Conditions {
    pub fn new(period_id: &str) -> Self {
        Self {
            period_id: period_id.to_string(),
            conditions: Vec::new(),
            results: Vec::new(),
        }
    }

    pub fn add_condition(&mut self, condition: Condition) {
        self.conditions.push(condition);
    }

    pub fn get_results(&self) -> &Vec<StockDataWithData> {
        &self.results
    }

    pub fn run_single(
        &mut self,
        stocks: &Stocks,
        stock_no: &str,
        date: &str,
    ) -> Option<StockDataWithData> {
        let stock_company = stocks.companies.get(stock_no).expect("找不到股票資料");

        let date_fugle = common::get_fugle_format(date);
        let curr_date_index = stock_company
            .get_index_by_date(&date_fugle)
            .expect("找不到日期");

        let mut stock_data_with_data = StockDataWithData {
            stock_no: stock_company.stock_no.clone(),
            stock_data: stock_company.stock_data[curr_date_index].clone(),
            ..Default::default()
        };

        if self.conditions.iter().all(|condition| {
            filter_condition(
                stock_company,
                curr_date_index,
                condition,
                &mut stock_data_with_data,
            )
        }) {
            return Some(stock_data_with_data);
        }
        None
    }

    pub fn run(&mut self, stocks: &Stocks) {
        self.results.clear();

        for company in &stocks.company_map.company_list {
            let stock_company = stocks
                .companies
                .get(&company.stock_no)
                .expect("找不到股票資料");

            for stock_data in &stock_company.stock_data {
                let period_id = common::get_fugle_format(&self.period_id);
                if stock_data.date.starts_with(&period_id) {
                    let curr_date_index = stock_company
                        .get_index_by_date(&stock_data.date)
                        .expect("找不到日期");

                    let mut stock_data_with_data = StockDataWithData {
                        stock_no: company.stock_no.clone(),
                        stock_data: stock_data.clone(),
                        ..Default::default()
                    };

                    if self.conditions.iter().all(|condition| {
                        filter_condition(
                            stock_company,
                            curr_date_index,
                            condition,
                            &mut stock_data_with_data,
                        )
                    }) {
                        self.results.push(stock_data_with_data);
                    }
                }
            }
        }
    }
}

fn filter_condition(
    stock_company: &Company,
    curr_date_index: usize,
    condition: &Condition,
    stock_data_with_data: &mut StockDataWithData,
) -> bool {
    match condition {
        Condition::LongRedCandle => condition_long_red_candle(stock_company, curr_date_index),
        Condition::LongUpperShadow => condition_long_upper_shadow(stock_company, curr_date_index),
        Condition::GravestoneDoji => condition_gravestone_doji(stock_company, curr_date_index),
        Condition::BullishEngulfing => {
            condition_bullish_engulfing(stock_company, curr_date_index, stock_data_with_data)
        }
        Condition::BearishEngulfing => condition_bearish_engulfing(stock_company, curr_date_index),
        Condition::Doji => condition_doji(stock_company, curr_date_index),
        Condition::HangingMan => condition_hanging_man(stock_company, curr_date_index),
        Condition::Volume { volume } => condition_volume(stock_company, curr_date_index, *volume),
        Condition::MacdGoldenCross => {
            condition_macd_golden_cross(stock_company, curr_date_index, stock_data_with_data)
        }
        Condition::VolumeSpike { ratio } => {
            condition_volume_spike(stock_company, curr_date_index, *ratio)
        }
        Condition::PriceReach { period, ratio } => {
            condition_price_reach(stock_company, curr_date_index, *period, *ratio)
        }
        Condition::DarkCloudCover => {
            condition_dark_cloud_cover(stock_company, curr_date_index, stock_data_with_data)
        }
        Condition::BullishHarami => condition_bullish_harami(stock_company, curr_date_index),
        Condition::BearishHarami => condition_bearish_harami(stock_company, curr_date_index),
        Condition::BullishHaramiThreeDayReversal => condition_bullish_harami_three_day_reversal(
            stock_company,
            curr_date_index,
            stock_data_with_data,
        ),
        Condition::BearishHaramiThreeDayReversal => condition_bearish_harami_three_day_reversal(
            stock_company,
            curr_date_index,
            stock_data_with_data,
        ),
        Condition::UpsideGapTwoCrows => {
            condition_upside_gap_two_crows(stock_company, curr_date_index)
        }
        Condition::ThreeWhiteSoldiers => {
            condition_three_white_soldiers(stock_company, curr_date_index, stock_data_with_data)
        }
        Condition::BreakawayGap => condition_breakaway_gap(stock_company, curr_date_index),
    }
}

// 突破缺口
// 1. 買紅K
// 2. 漲帶量
// 3. 缺口加收盤要一次站上 5/10/20 日均線
// 4. 缺口越大越好
// 5. 不可以回到缺口 (連碰都不能碰) ⇒ 假突破真跌破
fn condition_breakaway_gap(stock_company: &Company, curr_date_index: usize) -> bool {
    if curr_date_index == 0 {
        return false; // 如果是第一筆資料，則無法判斷前一天的 K 線，直接返回 false
    }

    let curr_stock_data = &stock_company
        .stock_data
        .get(curr_date_index)
        .expect("找不到日期");
    let prev_1_stock_data = &stock_company
        .stock_data
        .get(curr_date_index - 1)
        .expect("找不到日期");

    // 買紅K
    if !analysis::candlestick::is_bullish_candlestick(curr_stock_data) {
        return false;
    }

    // 漲帶量
    let (mv5, mv10, mv20) =
        match analysis::volume::find_prev_date_mv(stock_company, curr_date_index) {
            Some(mv) => mv,
            None => return false, // 如果找不到均量資料，則無法判斷，直接返回 false
        };
    let max_mv = mv5.max(mv10).max(mv20);
    if (curr_stock_data.volume as f64) < max_mv * consts::VOLUME_SPIKE_RATIO {
        return false;
    }

    // 收盤要一次站上 5/10/20 日均線
    let (ma5, ma10, ma20) = match analysis::price::find_prev_date_ma(stock_company, curr_date_index)
    {
        Some(ma) => ma,
        None => return false, // 如果找不到均線資料，則無法判斷，直接返回 false
    };
    let max_ma = ma5.max(ma10).max(ma20);
    if curr_stock_data.close <= max_ma {
        return false;
    }

    // 要有缺口，且缺口向上
    if curr_stock_data.open <= prev_1_stock_data.close.max(prev_1_stock_data.open) {
        return false;
    }

    true
}

// 紅三兵
// 5%長紅K，量要出在第1,2根K
// (紅三兵3根裡面，只有2根長紅K也可以)
// 若第3根K才出量不能買 (拉高出貨)
fn condition_three_white_soldiers(
    stock_company: &Company,
    curr_date_index: usize,
    stock_data_with_data: &mut StockDataWithData,
) -> bool {
    if curr_date_index < 2 {
        return false; // 如果前面沒有兩筆資料，則無法判斷，直接返回 false
    }

    let curr_stock_data = &stock_company
        .stock_data
        .get(curr_date_index)
        .expect("找不到日期");
    let prev_1_stock_data = &stock_company
        .stock_data
        .get(curr_date_index - 1)
        .expect("找不到日期");
    let prev_2_stock_data = &stock_company
        .stock_data
        .get(curr_date_index - 2)
        .expect("找不到日期");

    // 連續三根紅K
    if !analysis::candlestick::is_bullish_candlestick(prev_2_stock_data) {
        return false;
    }
    if !analysis::candlestick::is_bullish_candlestick(prev_1_stock_data) {
        return false;
    }
    if !analysis::candlestick::is_bullish_candlestick(curr_stock_data) {
        return false;
    }

    // 每根 K 線的實體都要比前一根高
    if prev_1_stock_data.close <= prev_2_stock_data.close {
        return false;
    }
    if curr_stock_data.close <= prev_1_stock_data.close {
        return false;
    }

    // 紅三兵3根裡面，要有2根長紅K
    let long_red_candle_count = [
        analysis::candlestick::candlestick_type(prev_2_stock_data),
        analysis::candlestick::candlestick_type(prev_1_stock_data),
        analysis::candlestick::candlestick_type(curr_stock_data),
    ]
    .iter()
    .filter(|&c| *c == analysis::candlestick::CandlestickType::LongRedCandle)
    .count();
    if long_red_candle_count < 2 {
        return false;
    }

    // 量要出在第1,2根K
    let (mv5_2, mv10_2, mv20_2) =
        match analysis::volume::find_prev_date_mv(stock_company, curr_date_index - 2) {
            Some(mv) => mv,
            None => return false, // 如果找不到均量資料，則無法判斷，直接返回 false
        };
    let max_mv_2 = mv5_2.max(mv10_2).max(mv20_2);
    if (prev_2_stock_data.volume as f64) < max_mv_2 * consts::VOLUME_SPIKE_RATIO {
        return false;
    }

    let (mv5_1, mv10_1, mv20_1) =
        match analysis::volume::find_prev_date_mv(stock_company, curr_date_index - 1) {
            Some(mv) => mv,
            None => return false, // 如果找不到均量資料，則無法判斷，直接返回 false
        };

    let max_mv_1 = mv5_1.max(mv10_1).max(mv20_1);
    if (prev_1_stock_data.volume as f64) < max_mv_1 * consts::VOLUME_SPIKE_RATIO {
        return false;
    }

    // 計算昨日均量 (MA5, MA10, MA20 的最大值)
    let (mv5, mv10, mv20) =
        match analysis::volume::find_prev_date_mv(stock_company, curr_date_index) {
            Some(mv) => mv,
            None => return false, // 如果找不到均量資料，則無法判斷，直接返回 false
        };

    stock_data_with_data.volume_change_result =
        Some(curr_stock_data.volume as f64 / mv5.max(mv10).max(mv20));

    true
}

// 烏鴉躍空
fn condition_upside_gap_two_crows(stock_company: &Company, curr_date_index: usize) -> bool {
    if curr_date_index < 2 {
        return false; // 如果前面沒有兩筆資料，則無法判斷，直接返回 false
    }

    let curr_stock_data = &stock_company
        .stock_data
        .get(curr_date_index)
        .expect("找不到日期");
    let prev_1_stock_data = &stock_company
        .stock_data
        .get(curr_date_index - 1)
        .expect("找不到日期");
    let prev_2_stock_data = &stock_company
        .stock_data
        .get(curr_date_index - 2)
        .expect("找不到日期");

    // 前兩天是紅K
    if !analysis::candlestick::is_bullish_candlestick(prev_2_stock_data) {
        return false;
    }

    // 前一天是黑K
    if !analysis::candlestick::is_bearish_candlestick(prev_1_stock_data) {
        return false;
    }

    // 當天是黑K
    if !analysis::candlestick::is_bearish_candlestick(curr_stock_data) {
        return false;
    }

    // 黑K創新高
    if curr_stock_data.open <= prev_1_stock_data.open {
        return false;
    }

    // 要有缺口
    if curr_stock_data.close <= prev_2_stock_data.close
        || prev_1_stock_data.close <= prev_2_stock_data.close
    {
        return false;
    }

    true
}

// 內困三日翻紅
fn condition_bullish_harami_three_day_reversal(
    stock_company: &Company,
    curr_date_index: usize,
    stock_data_with_data: &mut StockDataWithData,
) -> bool {
    if curr_date_index < 2 {
        return false; // 如果前面沒有兩筆資料，則無法判斷，直接返回 false
    }

    let curr_stock_data = &stock_company
        .stock_data
        .get(curr_date_index)
        .expect("找不到日期");
    let prev_1_stock_data = &stock_company
        .stock_data
        .get(curr_date_index - 1)
        .expect("找不到日期");
    let prev_2_stock_data = &stock_company
        .stock_data
        .get(curr_date_index - 2)
        .expect("找不到日期");

    // 1. 前兩天就是多頭母子

    // 1. 第一根是 **黑K**
    if !analysis::candlestick::is_bearish_candlestick(prev_2_stock_data) {
        return false;
    }

    // 2. 第二根是 **紅K**
    if !analysis::candlestick::is_bullish_candlestick(prev_1_stock_data) {
        return false;
    }

    // 紅K躲在黑K的實體內
    if prev_1_stock_data.open <= prev_2_stock_data.close
        || prev_1_stock_data.close >= prev_2_stock_data.open
    {
        return false;
    }

    // 當天是紅K
    if !analysis::candlestick::is_bullish_candlestick(curr_stock_data) {
        return false;
    }

    // 當天收盤價高於前兩天的黑K實體
    if curr_stock_data.close <= prev_2_stock_data.open {
        return false;
    }

    // 計算昨日均量 (MA5, MA10, MA20 的最大值)
    let (mv5, mv10, mv20) =
        match analysis::volume::find_prev_date_mv(stock_company, curr_date_index) {
            Some(mv) => mv,
            None => return false, // 如果找不到均量資料，則無法判斷，直接返回 false
        };

    stock_data_with_data.volume_change_result =
        Some(curr_stock_data.volume as f64 / mv5.max(mv10).max(mv20));

    true
}

// 內困三日翻黑
fn condition_bearish_harami_three_day_reversal(
    stock_company: &Company,
    curr_date_index: usize,
    stock_data_with_data: &mut StockDataWithData,
) -> bool {
    if curr_date_index < 2 {
        return false; // 如果前面沒有兩筆資料，則無法判斷，直接返回 false
    }

    let curr_stock_data = &stock_company
        .stock_data
        .get(curr_date_index)
        .expect("找不到日期");
    let prev_1_stock_data = &stock_company
        .stock_data
        .get(curr_date_index - 1)
        .expect("找不到日期");
    let prev_2_stock_data = &stock_company
        .stock_data
        .get(curr_date_index - 2)
        .expect("找不到日期");

    // 1. 前兩天就是空頭母子

    // 1. 第一根是 **紅K**
    if !analysis::candlestick::is_bullish_candlestick(prev_2_stock_data) {
        return false;
    }

    // 2. 第二根是 **黑K**
    if !analysis::candlestick::is_bearish_candlestick(prev_1_stock_data) {
        return false;
    }

    // 黑K躲在紅K的實體內
    if prev_1_stock_data.open >= prev_2_stock_data.close
        || prev_1_stock_data.close <= prev_2_stock_data.open
    {
        return false;
    }

    // 當天是黑K
    if !analysis::candlestick::is_bearish_candlestick(curr_stock_data) {
        return false;
    }

    // 當天收盤價低於前兩天的紅K實體
    if curr_stock_data.close >= prev_2_stock_data.open {
        return false;
    }

    // 計算昨日均量 (MA5, MA10, MA20 的最大值)
    let (mv5, mv10, mv20) =
        match analysis::volume::find_prev_date_mv(stock_company, curr_date_index) {
            Some(mv) => mv,
            None => return false, // 如果找不到均量資料，則無法判斷，直接返回 false
        };

    stock_data_with_data.volume_change_result =
        Some(curr_stock_data.volume as f64 / mv5.max(mv10).max(mv20));

    true
}

// 多頭母子
// 不用量，不看上下影線
// 多頭母子:
//  波段下跌後
//  紅K躲在黑K的實體內
// 母子線為強烈多空反轉之K線排列
fn condition_bullish_harami(stock_company: &Company, curr_date_index: usize) -> bool {
    if curr_date_index == 0 {
        return false; // 如果是第一筆資料，則無法判斷前一天的 K 線，直接返回 false
    }

    let curr_stock_data = &stock_company
        .stock_data
        .get(curr_date_index)
        .expect("找不到日期");
    let prev_1_stock_data = &stock_company
        .stock_data
        .get(curr_date_index - 1)
        .expect("找不到日期");

    // 波段下跌後
    // 做在外面

    // 1. 第一根是 **黑K**
    if !analysis::candlestick::is_bearish_candlestick(prev_1_stock_data) {
        return false;
    }

    // 2. 第二根是 **紅K**
    if !analysis::candlestick::is_bullish_candlestick(curr_stock_data) {
        return false;
    }

    // 紅K躲在黑K的實體內
    if curr_stock_data.open <= prev_1_stock_data.close
        || curr_stock_data.close >= prev_1_stock_data.open
    {
        return false;
    }

    true
}

// 空頭母子
// 不用量，不看上下影線
// 空頭母子:
//  波段上漲後
//  黑K躲在紅K的實體內
// 母子線為強烈多空反轉之K線排列
fn condition_bearish_harami(stock_company: &Company, curr_date_index: usize) -> bool {
    if curr_date_index == 0 {
        return false; // 如果是第一筆資料，則無法判斷前一天的 K 線，直接返回 false
    }

    let curr_stock_data = &stock_company
        .stock_data
        .get(curr_date_index)
        .expect("找不到日期");
    let prev_1_stock_data = &stock_company
        .stock_data
        .get(curr_date_index - 1)
        .expect("找不到日期");

    // 波段上漲後
    // 做在外面

    // 1. 第一根是 **紅K**
    if !analysis::candlestick::is_bullish_candlestick(prev_1_stock_data) {
        return false;
    }

    // 2. 第二根是 **黑K**
    if !analysis::candlestick::is_bearish_candlestick(curr_stock_data) {
        return false;
    }

    // 黑K躲在紅K的實體內
    if curr_stock_data.open >= prev_1_stock_data.close
        || curr_stock_data.close <= prev_1_stock_data.open
    {
        return false;
    }

    true
}

// 烏雲罩頂
// 1. 前面上漲 30%
// 2. 收黑K且創新高，前一天要紅K
// 3. 與前一天紅K有部分重疊
fn condition_dark_cloud_cover(
    stock_company: &Company,
    curr_date_index: usize,
    stock_data_with_data: &mut StockDataWithData,
) -> bool {
    if curr_date_index == 0 {
        return false; // 如果是第一筆資料，則無法判斷前一天的 K 線，直接返回 false
    }

    let curr_stock_data = &stock_company
        .stock_data
        .get(curr_date_index)
        .expect("找不到日期");
    let prev_1_stock_data = &stock_company
        .stock_data
        .get(curr_date_index - 1)
        .expect("找不到日期");

    // 1. 波段上漲創新高
    // 做在外面

    // 2. 第一根是 **紅K**
    if !analysis::candlestick::is_bullish_candlestick(prev_1_stock_data) {
        return false;
    }

    // 3. 第二根是 **黑K**，且創新高
    if !analysis::candlestick::is_bearish_candlestick(curr_stock_data) {
        return false;
    }

    // 黑K的最高價要高於前一天的最高價
    if curr_stock_data.open <= prev_1_stock_data.close {
        return false;
    }
    if curr_stock_data.close <= prev_1_stock_data.open
        || curr_stock_data.close >= prev_1_stock_data.close
    {
        return false;
    }

    // 計算昨日均量 (MA5, MA10, MA20 的最大值)
    let (mv5, mv10, mv20) =
        match analysis::volume::find_prev_date_mv(stock_company, curr_date_index) {
            Some(mv) => mv,
            None => return false, // 如果找不到均量資料，則無法判斷，直接返回 false
        };

    stock_data_with_data.volume_change_result =
        Some(curr_stock_data.volume as f64 / mv5.max(mv10).max(mv20));

    true
}

fn condition_price_reach(
    stock_company: &Company,
    curr_date_index: usize,
    period: usize,
    ratio: f64,
) -> bool {
    let stock_data = &stock_company
        .stock_data
        .get(curr_date_index)
        .expect("找不到日期");
    let target_price = stock_data.close * ratio;

    let start_index = curr_date_index.saturating_sub(period);

    for i in start_index..curr_date_index {
        let past_stock_data = &stock_company.stock_data[i];
        if ratio >= 1.0 {
            // 如果 ratio 大於等於 1，則檢查過去 period 天的收盤價是否有碰到 target_price
            if past_stock_data.close >= target_price {
                return true;
            }
        } else {
            // 如果 ratio 小於 1，則檢查過去 period 天的收盤價是否有碰到 target_price
            if past_stock_data.close <= target_price {
                return true;
            }
        }
    }

    false
}

// 陽吞噬
// 1. 波段下跌創新低
// 2. 第一根是 **黑K**
// 3. 第二根是 **長紅K**，將前一天的黑K吃光光 (包含上下影線)。
// 4. 需要出量(昨日均量的 1.5倍以上)，量越大，買越多
fn condition_bullish_engulfing(
    stock_company: &Company,
    curr_date_index: usize,
    stock_data_with_data: &mut StockDataWithData,
) -> bool {
    if curr_date_index == 0 {
        return false; // 如果是第一筆資料，則無法判斷前一天的 K 線，直接返回 false
    }

    let curr_stock_data = &stock_company
        .stock_data
        .get(curr_date_index)
        .expect("找不到日期");
    let prev_1_stock_data = &stock_company
        .stock_data
        .get(curr_date_index - 1)
        .expect("找不到日期");

    // 1. 波段下跌創新低
    // TODO

    // 2. 第一根是 **黑K**
    if !analysis::candlestick::is_bearish_candlestick(prev_1_stock_data) {
        return false;
    }

    // 3. 第二根是 **長紅K**，將前一天的黑K吃光光 (包含上下影線)。
    if analysis::candlestick::candlestick_type(curr_stock_data)
        != analysis::candlestick::CandlestickType::LongRedCandle
    {
        return false;
    }
    // 當天的實體完全包覆前一天的實體和影線
    if !(curr_stock_data.open < prev_1_stock_data.low
        && curr_stock_data.close > prev_1_stock_data.high)
    {
        return false;
    }

    // 4. 需要出量(昨日均量的 1.5倍以上)，量越大，買越多
    // 計算昨日均量 (MA5, MA10, MA20 的最大值)
    let (mv5, mv10, mv20) =
        match analysis::volume::find_prev_date_mv(stock_company, curr_date_index) {
            Some(mv) => mv,
            None => return false, // 如果找不到均量資料，則無法判斷，直接返回 false
        };

    stock_data_with_data.volume_change_result =
        Some(curr_stock_data.volume as f64 / mv5.max(mv10).max(mv20));

    true
}

// 陰吞噬
// 1. 波段上漲創新高
// 2. 第一根是 **紅K**
// 3. 第二根是 **長黑K**，將前一天的紅K吃光光 (包含上下影線)。
fn condition_bearish_engulfing(stock_company: &Company, curr_date_index: usize) -> bool {
    if curr_date_index == 0 {
        return false; // 如果是第一筆資料，則無法判斷前一天的 K 線，直接返回 false
    }

    let curr_stock_data = &stock_company
        .stock_data
        .get(curr_date_index)
        .expect("找不到日期");
    let prev_1_stock_data = &stock_company
        .stock_data
        .get(curr_date_index - 1)
        .expect("找不到日期");

    // 1. 波段上漲創新高
    // TODO

    // 2. 第一根是 **紅K**
    if !analysis::candlestick::is_bullish_candlestick(prev_1_stock_data) {
        return false;
    }

    // 3. 第二根是 **長黑K**，將前一天的紅K吃光光 (包含上下影線)。
    if analysis::candlestick::candlestick_type(curr_stock_data)
        != analysis::candlestick::CandlestickType::LongGreenCandle
    {
        return false;
    }
    // 當天的實體完全包覆前一天的實體和影線
    if !(curr_stock_data.open > prev_1_stock_data.high
        && curr_stock_data.close < prev_1_stock_data.low)
    {
        return false;
    }

    true
}

// 長上影線
fn condition_long_upper_shadow(stock_company: &Company, curr_date_index: usize) -> bool {
    let stock_data = &stock_company
        .stock_data
        .get(curr_date_index)
        .expect("找不到日期");
    analysis::candlestick::candlestick_type(stock_data)
        == analysis::candlestick::CandlestickType::LongUpperShadow
}

// 墓碑十字線
fn condition_gravestone_doji(stock_company: &Company, curr_date_index: usize) -> bool {
    let stock_data = &stock_company
        .stock_data
        .get(curr_date_index)
        .expect("找不到日期");
    analysis::candlestick::candlestick_type(stock_data)
        == analysis::candlestick::CandlestickType::GravestoneDoji
}

// MACD 黃金交叉 (MACD 線由下往上穿越訊號線)，不論是在零軸上方還是下方
fn condition_macd_golden_cross(
    stock_company: &Company,
    curr_date_index: usize,
    stock_data_with_data: &mut StockDataWithData,
) -> bool {
    let macd_result = analysis::macd::macd_cross(stock_company, curr_date_index);
    match macd_result.macd_cross {
        analysis::macd::MacdCross::GoldenCrossAboveZero
        | analysis::macd::MacdCross::GoldenCrossBelowZero => {
            stock_data_with_data.macd_result = Some(macd_result);
            true
        }
        _ => false,
    }
}

// 成交量大於等於 volume (單位: 張)
fn condition_volume(stock_company: &Company, curr_date_index: usize, volume: u64) -> bool {
    let stock_data = &stock_company
        .stock_data
        .get(curr_date_index)
        .expect("找不到日期");
    stock_data.volume >= volume * 1000 // 轉換成股
}

// 成交量較前一天均量的變化百分比
fn condition_volume_spike(stock_company: &Company, curr_date_index: usize, ratio: f64) -> bool {
    let stock_data = &stock_company
        .stock_data
        .get(curr_date_index)
        .expect("找不到日期");
    let (mv5, mv10, mv20) =
        match analysis::volume::find_prev_date_mv(stock_company, curr_date_index) {
            Some(mv) => mv,
            None => return false, // 如果找不到均量資料，則無法判斷，直接返回 false
        };
    let max_mv = mv5.max(mv10).max(mv20);
    if max_mv == 0.0 {
        return false; // 避免除以零
    }
    let volume_change_percentage = stock_data.volume as f64 / max_mv;
    volume_change_percentage >= ratio
}

// 長紅K
fn condition_long_red_candle(stock_company: &Company, curr_date_index: usize) -> bool {
    let stock_data = &stock_company
        .stock_data
        .get(curr_date_index)
        .expect("找不到日期");
    analysis::candlestick::candlestick_type(stock_data)
        == analysis::candlestick::CandlestickType::LongRedCandle
}

// 十字線
fn condition_doji(stock_company: &Company, curr_date_index: usize) -> bool {
    let stock_data = &stock_company
        .stock_data
        .get(curr_date_index)
        .expect("找不到日期");
    analysis::candlestick::candlestick_type(stock_data)
        == analysis::candlestick::CandlestickType::Doji
}

// 吊人線
// 1. 前兩天漲停 (兩根漲停+吊人線 or 一根漲停+漲停吊人線)
//     (通常吊人線都是漲停)
// 2. 觀察 1~4 天: 不要動作
// 3. 5~9 天: 如果有紅K帶量突破吊人線高點
fn condition_hanging_man(stock_company: &Company, curr_date_index: usize) -> bool {
    if curr_date_index < 3 {
        return false; // 如果是前兩筆資料，則無法判斷前一天的 K 線，直接返回 false
    }

    let curr_stock_data = &stock_company
        .stock_data
        .get(curr_date_index)
        .expect("找不到日期");
    let prev_1_stock_data = &stock_company
        .stock_data
        .get(curr_date_index - 1)
        .expect("找不到日期");
    let prev_2_stock_data = &stock_company
        .stock_data
        .get(curr_date_index - 2)
        .expect("找不到日期");
    let prev_3_stock_data = &stock_company
        .stock_data
        .get(curr_date_index - 3)
        .expect("找不到日期");

    // 兩根漲停+吊人線
    #[allow(clippy::collapsible_if)]
    if analysis::candlestick::candlestick_type(curr_stock_data)
        == analysis::candlestick::CandlestickType::HangingMan
    {
        if analysis::candlestick::is_limit_up(prev_2_stock_data, prev_1_stock_data) {
            if analysis::candlestick::is_limit_up(prev_3_stock_data, prev_2_stock_data) {
                return true;
            }
        }
    }

    // 一根漲停+當天是漲停且吊人線
    #[allow(clippy::collapsible_if)]
    if analysis::candlestick::candlestick_type(curr_stock_data)
        == analysis::candlestick::CandlestickType::HangingMan
    {
        if analysis::candlestick::is_limit_up(prev_1_stock_data, curr_stock_data) {
            if analysis::candlestick::is_limit_up(prev_2_stock_data, prev_1_stock_data) {
                return true;
            }
        }
    }

    false
}

//
//  Generate conditions
//

pub fn generate_conditions(condition: &Condition, input: &str) -> Conditions {
    let mut conditions = Conditions::new(input);

    match condition {
        Condition::LongRedCandle => conditions.add_condition(Condition::LongRedCandle),
        Condition::LongUpperShadow => conditions.add_condition(Condition::LongUpperShadow),
        Condition::Doji => conditions.add_condition(Condition::Doji),
        Condition::GravestoneDoji => conditions.add_condition(Condition::GravestoneDoji),
        Condition::BullishEngulfing => {
            conditions.add_condition(Condition::BullishEngulfing);
            conditions.add_condition(Condition::VolumeSpike {
                ratio: consts::VOLUME_SPIKE_RATIO,
            });
            conditions.add_condition(Condition::PriceReach {
                period: consts::SWING_PERIOD,
                ratio: consts::SWING_UP_RATIO,
            });
        }
        Condition::BearishEngulfing => {
            conditions.add_condition(Condition::BearishEngulfing);
            conditions.add_condition(Condition::PriceReach {
                period: consts::SWING_PERIOD,
                ratio: consts::SWING_DOWN_RATIO,
            });
        }
        Condition::DarkCloudCover => {
            conditions.add_condition(Condition::DarkCloudCover);
            conditions.add_condition(Condition::PriceReach {
                period: consts::SWING_PERIOD,
                ratio: consts::SWING_DOWN_RATIO,
            });
        }
        Condition::BullishHarami => {
            conditions.add_condition(Condition::BullishHarami);
            conditions.add_condition(Condition::PriceReach {
                period: consts::SWING_PERIOD,
                ratio: consts::SWING_UP_RATIO,
            });
        }
        Condition::BearishHarami => {
            conditions.add_condition(Condition::BearishHarami);
            conditions.add_condition(Condition::PriceReach {
                period: consts::SWING_PERIOD,
                ratio: consts::SWING_DOWN_RATIO,
            });
        }
        Condition::BullishHaramiThreeDayReversal => {
            conditions.add_condition(Condition::BullishHaramiThreeDayReversal);
            conditions.add_condition(Condition::PriceReach {
                period: consts::SWING_PERIOD,
                ratio: consts::SWING_UP_RATIO,
            });
            conditions.add_condition(Condition::VolumeSpike {
                ratio: consts::VOLUME_SPIKE_RATIO,
            });
        }
        Condition::BearishHaramiThreeDayReversal => {
            conditions.add_condition(Condition::BearishHaramiThreeDayReversal);
            conditions.add_condition(Condition::PriceReach {
                period: consts::SWING_PERIOD,
                ratio: consts::SWING_DOWN_RATIO,
            });
        }
        Condition::UpsideGapTwoCrows => {
            conditions.add_condition(Condition::UpsideGapTwoCrows);
            conditions.add_condition(Condition::PriceReach {
                period: consts::SWING_PERIOD,
                ratio: consts::SWING_DOWN_RATIO,
            });
        }
        Condition::ThreeWhiteSoldiers => {
            conditions.add_condition(Condition::ThreeWhiteSoldiers);
        }
        Condition::HangingMan => {
            conditions.add_condition(Condition::HangingMan);
        }
        _ => panic!("目前只支援 部分 條件的生成"),
    }

    conditions
}
