use ta::Next;
use ta::indicators::ExponentialMovingAverage as Ema;

use crate::data;
use crate::twse::company_map::CompanyMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MacdCrossType {
    GoldenCross,
    DeathCross,
}

#[derive(Debug, Clone)]
pub struct MacdCross {
    pub stock_no: String,
    pub date: String,
    pub dif: f64,
    pub macd_signal: f64,
    pub cross_type: MacdCrossType,
}

impl MacdCross {
    pub fn print(&self, company_map: &CompanyMap) {
        match self.cross_type {
            MacdCrossType::GoldenCross => {
                println!(
                    "🚀 【黃金交叉】{} Date: {} {}",
                    self.stock_no,
                    self.date,
                    company_map.get(&self.stock_no)
                );
            }
            MacdCrossType::DeathCross => {
                println!(
                    "💀 【死亡交叉】{} Date: {} {}",
                    self.stock_no,
                    self.date,
                    company_map.get(&self.stock_no)
                );
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct MacdResult {
    dif: f64,
    macd_signal: f64,
    histogram: f64,
}

pub struct MacdCalculator {
    stock_no: String,
    year_month_from: String,
    year_month_to: String,
    ema12: Ema,
    ema26: Ema,
    signal_ema9: Ema,
    prev_dif: f64,
    prev_signal: f64,
}

impl MacdCalculator {
    pub fn new(stock_no: &str, year_month_from: &str, year_month_to: &str) -> Self {
        Self {
            stock_no: stock_no.to_string(),
            year_month_from: year_month_from.to_string(),
            year_month_to: year_month_to.to_string(),
            ema12: Ema::new(12).unwrap(),
            ema26: Ema::new(26).unwrap(),
            signal_ema9: Ema::new(9).unwrap(),
            prev_dif: 0.0,
            prev_signal: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.ema12 = Ema::new(12).unwrap();
        self.ema26 = Ema::new(26).unwrap();
        self.signal_ema9 = Ema::new(9).unwrap();
        self.prev_dif = 0.0;
        self.prev_signal = 0.0;
    }

    pub async fn calc(&mut self, company_map: &CompanyMap) -> (Vec<MacdResult>, Vec<MacdCross>) {
        self.reset();

        let (year_str, month_str) = self.year_month_from.split_at(4);
        let mut year: u32 = year_str.parse().unwrap();
        let mut month: u32 = month_str.parse().unwrap();

        let mut results = Vec::new();
        let mut macd_crosses = Vec::new();

        loop {
            let year_month = format!("{:04}{:02}", year, month);
            let monthly_data =
                data::monthly_data::MonthlyData::new(company_map, &self.stock_no, &year_month)
                    .await;

            let mut daily_data = monthly_data.daily_data.clone();
            daily_data.sort_by(|a, b| a.date.cmp(&b.date)); // 按日期排序

            for daily in &daily_data {
                let close_price = daily.close;
                let (res, cross) = self.feed(&daily.date, close_price);
                results.push(res);
                if let Some(c) = cross {
                    macd_crosses.push(c);
                }
            }

            if year_month == self.year_month_to {
                break;
            }

            // 月份 +1
            month += 1;
            if month > 12 {
                month = 1;
                year += 1;
            }
        }

        (results, macd_crosses)
    }

    fn feed(&mut self, date: &str, close_price: f64) -> (MacdResult, Option<MacdCross>) {
        let e12 = self.ema12.next(close_price);
        let e26 = self.ema26.next(close_price);

        let dif = e12 - e26;
        let signal = self.signal_ema9.next(dif);
        let histogram = dif - signal;

        let res = MacdResult {
            dif,
            macd_signal: signal,
            histogram,
        };

        // 判斷交叉邏輯
        let mut cross = None;
        if self.prev_dif <= self.prev_signal && dif > signal {
            // println!("🚀 【黃金交叉】{} Date: {} DIF({:.2}) 向上突破 MACD({:.2})", self.stock_no, date, dif, signal);
            cross = Some(MacdCross {
                stock_no: self.stock_no.clone(),
                date: date.to_string(),
                dif,
                macd_signal: signal,
                cross_type: MacdCrossType::GoldenCross,
            });
        } else if self.prev_dif >= self.prev_signal && dif < signal {
            // println!("💀 【死亡交叉】{} Date: {} DIF({:.2}) 向下貫穿 MACD({:.2})", self.stock_no, date, dif, signal);
            cross = Some(MacdCross {
                stock_no: self.stock_no.clone(),
                date: date.to_string(),
                dif,
                macd_signal: signal,
                cross_type: MacdCrossType::DeathCross,
            });
        }

        self.prev_dif = dif;
        self.prev_signal = signal;
        (res, cross)
    }
}
