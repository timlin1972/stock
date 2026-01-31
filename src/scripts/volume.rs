use crate::analysis::volume;
use crate::analysis::volume::VolumeAnalysisResult;
use crate::twse::company_map::CompanyMap;

pub async fn volume_larger_than_threshold(
    company_map: &CompanyMap,
    threshold: u64,
    year_month_date: &str,
) -> Vec<VolumeAnalysisResult> {
    volume::volume_larger_than_threshold(company_map, threshold, year_month_date).await
}
