use crate::analysis;
use crate::common;
use crate::scripts;
use crate::twse::company_map::CompanyMap;

pub struct MacdGoldenVolumeLargerResult {
    pub macd_cross: analysis::macd::MacdCross,
    pub volume_result: analysis::volume::VolumeAnalysisResult,
}

impl MacdGoldenVolumeLargerResult {
    pub fn print(&self, company_map: &CompanyMap) {
        println!(
            "{:<10} {:<10} {:>10} {}",
            self.macd_cross.stock_no,
            self.macd_cross.date,
            common::format_commas(common::divide_by_1000(self.volume_result.daily_data.volume)),
            company_map.get(&self.macd_cross.stock_no)
        );
    }
}

pub async fn anal_macd_golden_volume_larger_date(
    company_map: &CompanyMap,
    year_month_from: &str,
    year_month_to: &str,
    year_month_date: &str,
    volume_threshold: u64,
) -> Vec<MacdGoldenVolumeLargerResult> {
    let mut ret_results = Vec::new();

    let crosses = scripts::macd::anal_date_all_companies(
        company_map,
        year_month_from,
        year_month_to,
        year_month_date,
    )
    .await;

    let volume_results = scripts::volume::volume_larger_than_threshold(
        company_map,
        volume_threshold,
        year_month_date,
    )
    .await;

    // if stock_no is same as in crosses and volume_results, then collect both results
    for cross in &crosses {
        if cross.cross_type == analysis::macd::MacdCrossType::GoldenCross {
            // find volume result for the same stock_no
            for volume_result in &volume_results {
                if volume_result.stock_no == cross.stock_no {
                    let cross_clone = cross.clone();
                    let volume_result_clone = volume_result.clone();
                    ret_results.push(MacdGoldenVolumeLargerResult {
                        macd_cross: cross_clone,
                        volume_result: volume_result_clone,
                    });
                }
            }
        }
    }

    ret_results.sort_by(|a, b| {
        b.volume_result
            .daily_data
            .volume
            .cmp(&a.volume_result.daily_data.volume)
    });

    ret_results
}
