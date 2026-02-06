use crate::common;
use crate::data;
use crate::data::monthly_data::DailyData;
use crate::twse::company_map::CompanyMap;

pub struct BullishEngulfingPattern {
    pub stock_no: String,
    pub date: String,
    // pub prev_day: DailyData,
    pub curr_day: DailyData,
}

fn is_bullish_engulfing(prev: &DailyData, curr: &DailyData) -> bool {
    // 前一天是黑K
    let prev_black = prev.close < prev.open;
    // 當天是紅K
    let curr_red = curr.close > curr.open;

    // 當天的實體完全包覆前一天的實體和影線
    let engulf = curr.open < prev.low && curr.close > prev.high;

    prev_black && curr_red && engulf
}

pub async fn anal_range_company(
    company_map: &CompanyMap,
    stock_no: &str,
    year_month_from: &str,
    year_month_to: &str,
) -> Vec<BullishEngulfingPattern> {
    let (year_str, month_str) = year_month_from.split_at(4);
    let mut year: u32 = year_str.parse().unwrap();
    let mut month: u32 = month_str.parse().unwrap();

    let mut results = Vec::new();
    let mut prev = None;
    #[allow(unused_assignments)]
    let mut curr = None;
    loop {
        let year_month = format!("{:04}{:02}", year, month);
        let monthly_data =
            data::monthly_data::MonthlyData::new(company_map, stock_no, &year_month).await;

        let mut daily_data = monthly_data.daily_data.clone();
        daily_data.sort_by(|a, b| a.date.cmp(&b.date)); // 按日期排序

        for daily in &daily_data {
            curr = Some(daily.clone());
            if let (Some(prev_day), Some(curr_day)) = (prev.as_ref(), curr.as_ref())
                && is_bullish_engulfing(prev_day, curr_day)
            {
                // println!(
                //     "{} Bullish Engulfing Pattern detected on {}: Prev Day: {:?}, Curr Day: {:?}",
                //     stock_no, curr_day.date, prev_day, curr_day
                // );
                results.push(BullishEngulfingPattern {
                    stock_no: stock_no.to_string(),
                    date: curr_day.date.clone(),
                    // prev_day: prev_day.clone(),
                    curr_day: curr_day.clone(),
                });
            }
            prev = curr.clone();
        }

        if year_month == year_month_to {
            break;
        }

        // 月份 +1
        month += 1;
        if month > 12 {
            month = 1;
            year += 1;
        }
    }
    results
}

fn get_prev_month(year_month: &str) -> String {
    let (year_str, month_str) = year_month.split_at(4);
    let mut year: u32 = year_str.parse().unwrap();
    let mut month: u32 = month_str.parse().unwrap();

    // 月份 -1
    if month == 1 {
        month = 12;
        year -= 1;
    } else {
        month -= 1;
    }

    format!("{:04}{:02}", year, month)
}

pub async fn anal_date_company(
    company_map: &CompanyMap,
    stock_no: &str,
    date: &str,
) -> Vec<BullishEngulfingPattern> {
    let year_month = &date[0..6]; // 提取年月部分
    let monthly_data =
        data::monthly_data::MonthlyData::new(company_map, stock_no, year_month).await;

    let mut results = Vec::new();

    let mut daily_data = monthly_data.daily_data.clone();
    daily_data.sort_by(|a, b| a.date.cmp(&b.date)); // 按日期排序
    let daily = daily_data
        .iter()
        .find(|d| d.date == common::to_roc_date(date));
    if daily.is_none() {
        println!("==========> No data for {} on date {}", stock_no, date);
        return results;
    }

    let curr = daily.unwrap();
    let prev_index = daily_data.iter().position(|d| d.date == curr.date).unwrap();
    if prev_index == 0 {
        // get prev month's last day
        let prev_month = get_prev_month(year_month);
        let prev_monthly_data =
            data::monthly_data::MonthlyData::new(company_map, stock_no, &prev_month).await;
        let mut prev_daily_data = prev_monthly_data.daily_data.clone();
        prev_daily_data.sort_by(|a, b| a.date.cmp(&b.date)); // 按日期排序
        if let Some(prev) = prev_daily_data.last() {
            if is_bullish_engulfing(prev, curr) {
                results.push(BullishEngulfingPattern {
                    stock_no: stock_no.to_string(),
                    date: curr.date.clone(),
                    // prev_day: prev.clone(),
                    curr_day: curr.clone(),
                });
            }
        } else {
            panic!(
                "==========> No previous data for {} on date {}",
                stock_no, date
            );
        }
    } else {
        let prev = &daily_data[prev_index - 1];
        if is_bullish_engulfing(prev, curr) {
            results.push(BullishEngulfingPattern {
                stock_no: stock_no.to_string(),
                date: curr.date.clone(),
                // prev_day: prev.clone(),
                curr_day: curr.clone(),
            });
        }
    }

    results
}
