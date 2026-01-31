use crate::common;
use crate::data::monthly_data::{DailyData, MonthlyData};
use crate::twse::company_map::CompanyMap;

#[derive(Clone)]
pub struct VolumeAnalysisResult {
    pub stock_no: String,
    pub daily_data: DailyData,
}

pub async fn volume_larger_than_threshold(
    company_map: &CompanyMap,
    threshold: u64,
    year_month_date: &str,
) -> Vec<VolumeAnalysisResult> {
    let year_month = &year_month_date[0..6];
    let mut daily_data_all = Vec::new();

    for company in &company_map.stock_map {
        let monthly_data = MonthlyData::new(company_map, &company.stock_no, year_month).await;

        for daily in &monthly_data.daily_data {
            // println!("Checking {} on date {} with {}", stock_no, daily.date, common::to_roc_date(year_month_date));
            if daily.date == common::to_roc_date(year_month_date) && daily.volume > threshold * 1000
            {
                daily_data_all.push(VolumeAnalysisResult {
                    stock_no: company.stock_no.clone(),
                    daily_data: daily.clone(),
                });
            }
        }
    }

    // sort by volume descending
    daily_data_all.sort_by(|a, b| b.daily_data.volume.cmp(&a.daily_data.volume));

    // println!(
    //     "{:<8}{:<5}{:>6}{:>5}{:>5}{:>5}{:>5}{:>6} {}",
    //     "日期", "台股", "成交股數", "開盤價", "收盤價", "最高價", "最低價", "漲跌", "公司名稱"
    // );

    // for result in &daily_data_all {
    //     result.daily_data.print(company_map, &result.stock_no);
    // }

    daily_data_all
}
