use crate::data::company::StockData;

#[derive(Debug, PartialEq)]
pub enum CandlestickType {
    LongRedCandle,
    LongGreenCandle,
    Doji,
    // LongLowerShadow,
    // LongUpperShadow,
    // HangingMan,
    // // ShootingStar,
    // // SpinningTop,
    Unknown,
}

pub fn candlestick_type(stock_data: &StockData) -> CandlestickType {
    let open = stock_data.open;
    let close = stock_data.close;
    let high = stock_data.high;
    let low = stock_data.low;

    let body_length = (close - open).abs();
    let upper_shadow = high - open.max(close);
    let lower_shadow = open.min(close) - low;

    // 實體長度大於收盤價的5%
    if body_length > 0.05 * close {
        // 長紅K棒的條件：且收盤價高於開盤價
        if close > open {
            return CandlestickType::LongRedCandle;
        }
        // 長黑K棒的條件：且收盤價低於開盤價
        else {
            return CandlestickType::LongGreenCandle;
        }
    }

    // 1. 必須完全沒有顏色 (開盤=收盤)
    // 2. 要有上影線和下影線
    if open == close && upper_shadow != 0.0 && lower_shadow != 0.0 {
        return CandlestickType::Doji;
    }

    // } else if (open - close).abs() < 0.01 * ((high - low).max(1.0))
    //     && open != high
    //     && open != low
    //     && close != high
    //     && close != low
    // {
    //     CandlestickType::Doji
    // } else if lower_shadow > 0.05 * close {
    //     CandlestickType::LongLowerShadow
    // } else if upper_shadow > 0.05 * close {
    //     CandlestickType::LongUpperShadow
    // } else if lower_shadow > 2.0 * body_length && close == high {
    //     CandlestickType::HangingMan
    // }
    // /*
    // else if upper_shadow > 2.0 * body_length && lower_shadow > 2.0 * body_length {
    //     CandlestickType::ShootingStar
    // } else {
    //     CandlestickType::SpinningTop
    // }
    //  */
    // else {
    //     CandlestickType::Unknown
    // }

    CandlestickType::Unknown
}
