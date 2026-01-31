use crate::data::monthly_data::MonthlyData;
use crate::twse::company_map::CompanyMap;

pub async fn fetch_data_monthly_all_companies(company_map: &CompanyMap, year_month: &str) {
    for company in &company_map.stock_map {
        let _monthly_data = MonthlyData::new(company_map, &company.stock_no, year_month).await;
    }
}

pub async fn fetch_data_yearly(company_map: &CompanyMap, year: &str) {
    for month in 1..=12 {
        let year_month = format!("{}{:02}", year, month);
        fetch_data_monthly_all_companies(company_map, &year_month).await;
    }
}
