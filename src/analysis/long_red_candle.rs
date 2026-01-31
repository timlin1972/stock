use crate::common;
use crate::data::monthly_data::MonthlyData;
use crate::twse::company_map::CompanyMap;

pub async fn anal_date_company(company_map: &CompanyMap, stock_no: &str, year_month_date: &str) {
    let monthly_data = MonthlyData::new(company_map, stock_no, &year_month_date[..6]).await;
    let roc_date = common::to_roc_date(year_month_date);

    // Filter daily data for the specific date
    let daily_data_for_date: Vec<_> = monthly_data
        .daily_data
        .iter()
        .filter(|daily| daily.date == roc_date)
        .cloned()
        .collect();

    for daily in daily_data_for_date {
        let open = daily.open;
        let close = daily.close;

        // A simple Long Red Candle condition: Close price significantly higher than Open price
        if close > open * 1.05 {
            daily.print(company_map, &monthly_data.stock_no);
        }
    }
}

pub async fn anal_date_all_companies(company_map: &CompanyMap, year_month_date: &str) {
    println!(
        "{:<8}{:<5}{:>6}{:>5}{:>5}{:>5}{:>5}{:>6} {}",
        "日期", "台股", "成交股數", "開盤價", "收盤價", "最高價", "最低價", "漲跌", "公司名稱"
    );
    for company in &company_map.stock_map {
        anal_date_company(company_map, &company.stock_no, year_month_date).await;
    }
}

pub fn long_red_candle_analysis(company_map: &CompanyMap, monthly_data: &MonthlyData) {
    // Placeholder for Long Red Candle analysis implementation
    println!("Performing Long Red Candle analysis...");
    println!(
        "{:<8}{:<5}{:>6}{:>5}{:>5}{:>5}{:>5}{:>6} {}",
        "日期", "台股", "成交股數", "開盤價", "收盤價", "最高價", "最低價", "漲跌", "公司名稱"
    );

    // Example: Iterate through daily data and identify Long Red Candle patterns
    for daily in &monthly_data.daily_data {
        let open = daily.open;
        let close = daily.close;

        // A simple Long Red Candle condition: Close price significantly higher than Open price
        if close > open * 1.05 {
            daily.print(company_map, &monthly_data.stock_no);
        }
    }
}
