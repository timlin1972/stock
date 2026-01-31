use crate::analysis::doji;
use crate::data::monthly_data::MonthlyData;
use crate::twse::company_map::CompanyMap;

pub async fn anal_month_all_compies(company_map: &CompanyMap, year_month: &str) {
    for company in &company_map.stock_map {
        let monthly_data = MonthlyData::new(company_map, &company.stock_no, year_month).await;
        doji::anal_month_company(company_map, &monthly_data);
    }
}

pub async fn anal_date_all_companies(
    company_map: &CompanyMap,
    year_month_date: &str,
) -> Vec<doji::DojiAnalysisResult> {
    let mut ret_results = Vec::new();

    let year_month = &year_month_date[0..6];

    for company in &company_map.stock_map {
        let monthly_data = MonthlyData::new(company_map, &company.stock_no, year_month).await;

        let doji_results = doji::anal_date_company(company_map, &monthly_data, year_month_date);

        for result in doji_results {
            ret_results.push(result);
        }
    }

    ret_results.sort_by(|a, b| b.daily_data.volume.cmp(&a.daily_data.volume));

    ret_results
}
