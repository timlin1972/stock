use crate::data::company::StockData;

#[derive(Debug, PartialEq)]
pub enum CandlestickType {
    LongRedCandle,
    LongGreenCandle,
    Doji,
    HangingMan,
    LongUpperShadow,
    LongLowerShadow,
    GravestoneDoji,
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

    // 長紅K棒或長黑K棒的條件：實體長度大於收盤價的5%
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

    // 十字線的條件：
    // 1. 必須完全沒有顏色 (開盤=收盤)
    // 2. 要有上影線和下影線
    if open == close && upper_shadow != 0.0 && lower_shadow != 0.0 {
        return CandlestickType::Doji;
    }

    // 吊人線的條件：
    // 1. 下影線至少是實體的兩倍長
    // 2. 收盤價等於最高價
    // 3. 收盤價等於開盤價 (沒有顏色)
    if lower_shadow > 2.0 * body_length && close == high && open == close {
        return CandlestickType::HangingMan;
    }

    // 長上影線的條件：上影線長度大於收盤價的5%
    if upper_shadow > 0.05 * close {
        return CandlestickType::LongUpperShadow;
    }

    // 長下影線的條件：下影線長度大於收盤價的5%
    if lower_shadow > 0.05 * close {
        return CandlestickType::LongLowerShadow;
    }

    // 墓碑十字線的條件：
    // 1. 上影線至少是實體的兩倍長
    // 2. 收盤價等於最低價
    // 3. 收盤價等於開盤價 (沒有顏色)
    if upper_shadow > 2.0 * body_length && close == low && open == close {
        return CandlestickType::GravestoneDoji;
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

pub fn is_limit_up(prev_stock_data: &StockData, curr_stock_data: &StockData) -> bool {
    curr_stock_data.close / prev_stock_data.close >= 1.093
}

// 判斷是否為看跌 K 棒 (黑K)
pub fn is_bearish_candlestick(stock_data: &StockData) -> bool {
    stock_data.close < stock_data.open
}

// 判斷是否為看漲 K 棒 (紅K)
pub fn is_bullish_candlestick(stock_data: &StockData) -> bool {
    stock_data.close > stock_data.open
}
