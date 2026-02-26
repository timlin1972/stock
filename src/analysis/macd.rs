use std::fmt;

use serde::{Deserialize, Serialize};
use ta::Next;
use ta::indicators::ExponentialMovingAverage as Ema;

use crate::data::company::Company;

const MODULE_NAME: &str = "analysis::macd";

// 在實際交易系統中，MACD 指標對初始數據量非常敏感。
// 建議餵給指標的數據量（K 線數量）至少要有 100 根以上，算出來的 EMA 才會與看盤軟體（如 TradingView 或券商 APP）一致。
const INTERVALS: usize = 100;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Ord, Eq, PartialOrd)]
pub enum MacdCross {
    DeathCrossBelowZero,  // 零軸下死亡交叉 (弱勢)
    DeathCrossAboveZero,  // 零軸上死亡交叉 (警訊)
    GoldenCrossBelowZero, // 零軸下黃金交叉 (反彈)
    GoldenCrossAboveZero, // 零軸上黃金交叉 (強勢)
    None,
}

impl fmt::Display for MacdCross {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            MacdCross::GoldenCrossAboveZero => "零軸上黃金交叉 (強勢)",
            MacdCross::GoldenCrossBelowZero => "零軸下黃金交叉 (反彈)",
            MacdCross::DeathCrossAboveZero => "零軸上死亡交叉 (警訊)",
            MacdCross::DeathCrossBelowZero => "零軸下死亡交叉 (弱勢)",
            MacdCross::None => "無交叉",
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MacdResult {
    pub dif: f64,
    pub signal: f64,
    pub macd_cross: MacdCross,
}

pub fn macd_cross(stock_company: &Company, curr_date_index: usize) -> MacdResult {
    // 初始化指標：MACD 標準參數 (12, 26, 9)
    let mut fast_ema = Ema::new(12).unwrap();
    let mut slow_ema = Ema::new(26).unwrap();
    let mut signal_ema = Ema::new(9).unwrap();

    let mut prev_dif = 0.0;
    let mut prev_signal = 0.0;
    let mut last_cross = MacdCross::None;

    if curr_date_index < INTERVALS {
        println!(
            "[{MODULE_NAME}] {} 資料不足，無法計算 MACD",
            stock_company.stock_no
        );
        // 如果資料不足以計算 MACD，直接返回 None
        return MacdResult {
            dif: 0.0,
            signal: 0.0,
            macd_cross: MacdCross::None,
        };
    }

    for daily in
        &stock_company.stock_data[curr_date_index.saturating_sub(INTERVALS)..=curr_date_index]
    {
        let close_price = daily.close;

        let f = fast_ema.next(close_price);
        let s = slow_ema.next(close_price);

        // DIF (快線) = EMA(12) - EMA(26)
        let dif = f - s;
        // Signal (DEA/慢線) = EMA(DIF, 9)
        let signal = signal_ema.next(dif);

        let mut current_event = MacdCross::None;

        // 判斷黃金交叉：前一天 DIF <= Signal，今天 DIF > Signal
        if prev_dif <= prev_signal && dif > signal {
            if dif > 0.0 {
                current_event = MacdCross::GoldenCrossAboveZero;
            } else {
                current_event = MacdCross::GoldenCrossBelowZero;
            }
        }
        // 判斷死亡交叉：前一天 DIF >= Signal，今天 DIF < Signal
        else if prev_dif >= prev_signal && dif < signal {
            if dif > 0.0 {
                current_event = MacdCross::DeathCrossAboveZero;
            } else {
                current_event = MacdCross::DeathCrossBelowZero;
            }
        }

        // 更新前一天的數值供下一輪使用
        prev_dif = dif;
        prev_signal = signal;
        last_cross = current_event;
    }

    MacdResult {
        dif: prev_dif,
        signal: prev_signal,
        macd_cross: last_cross,
    }
}
