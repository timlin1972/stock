use crate::data::monthly_data::DailyData;
use crate::data::monthly_data::MonthlyData;
use crate::twse::company_map::CompanyMap;

#[derive(Clone)]
pub struct DojiAnalysisResult {
    pub stock_no: String,
    pub daily_data: DailyData,
}

impl DojiAnalysisResult {
    pub fn print(&self, company_map: &CompanyMap) {
        self.daily_data.print(company_map, &self.stock_no);
    }
}

/*
pub fn anal_month_company(company_map: &CompanyMap, monthly_data: &MonthlyData) {
    // Placeholder for Doji analysis implementation
    println!("Performing Doji analysis...");
    println!(
        "{:<8}{:<5}{:>6}{:>5}{:>5}{:>5}{:>5}{:>6} 公司名稱",
        "日期", "台股", "成交股數", "開盤價", "收盤價", "最高價", "最低價", "漲跌",
    );

    // Example: Iterate through daily data and identify Doji patterns
    for daily in &monthly_data.daily_data {
        let open = daily.open;
        let close = daily.close;
        let high = daily.high;
        let low = daily.low;

        // A simple Doji condition: Open and Close prices are very close
        if (open - close).abs() < 0.01 * ((high - low).max(1.0)) {
            daily.print(company_map, &monthly_data.stock_no);
        }
    }
}
 */

pub fn anal_date_company(
    _company_map: &CompanyMap,
    monthly_data: &MonthlyData,
    year_month_date: &str,
) -> Vec<DojiAnalysisResult> {
    let mut ret_results = Vec::new();

    for daily in &monthly_data.daily_data {
        if daily.date == crate::common::to_roc_date(year_month_date) {
            let open = daily.open;
            let close = daily.close;
            let high = daily.high;
            let low = daily.low;

            // A simple Doji condition: Open and Close prices are very close
            // And, open is not equal to high and low
            // And, close is not equal to high and low
            if (open - close).abs() < 0.01 * ((high - low).max(1.0)) &&
               open != high && open != low &&
               close != high && close != low {
                ret_results.push(DojiAnalysisResult {
                    stock_no: monthly_data.stock_no.clone(),
                    daily_data: daily.clone(),
                });
            }
        }
    }

    ret_results
}
