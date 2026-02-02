use crate::analysis::bullish_engulfing_pattern;
use crate::twse::company_map::CompanyMap;

pub async fn anal_range_all_companies(
    company_map: &CompanyMap,
    year_month_from: &str,
    year_month_to: &str,
) -> Vec<bullish_engulfing_pattern::BullishEngulfingPattern> {
    let mut all_results = Vec::new();
    for company in &company_map.stock_map {
        let results = bullish_engulfing_pattern::anal_range_company(
            company_map,
            &company.stock_no,
            year_month_from,
            year_month_to,
        )
        .await;
        all_results.extend(results);
    }
    all_results
}

pub async fn anal_date_all_companies(
    company_map: &CompanyMap,
    date: &str,
) -> Vec<bullish_engulfing_pattern::BullishEngulfingPattern> {
    let mut all_results = Vec::new();
    for company in &company_map.stock_map {
        let results =
            bullish_engulfing_pattern::anal_date_company(company_map, &company.stock_no, date)
                .await;
        all_results.extend(results);
    }
    all_results
}
