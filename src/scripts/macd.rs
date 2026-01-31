use crate::analysis;
use crate::common;
use crate::twse::company_map::CompanyMap;

pub async fn anal_date_all_companies(
    company_map: &CompanyMap,
    year_month_from: &str,
    year_month_to: &str,
    year_month_date: &str,
) -> Vec<analysis::macd::MacdCross> {
    let mut ret_crosses = Vec::new();
    for company in &company_map.stock_map {
        let mut macd_calculator =
            analysis::macd::MacdCalculator::new(&company.stock_no, year_month_from, year_month_to);
        let (_res, crosses) = macd_calculator.calc(&company_map).await;
        for cross in &crosses {
            if cross.date == common::to_roc_date(year_month_date) {
                let clone_cross = analysis::macd::MacdCross {
                    stock_no: cross.stock_no.clone(),
                    date: cross.date.clone(),
                    dif: cross.dif,
                    macd_signal: cross.macd_signal,
                    cross_type: cross.cross_type.clone(),
                };
                ret_crosses.push(clone_cross);
            }
        }
    }

    ret_crosses
}
