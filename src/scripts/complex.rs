use crate::analysis;
use crate::common;
use crate::data::monthly_data::DailyData;
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

pub struct DojiInSwingResult {
    pub stock_no: String,
    pub daily_data: DailyData,
    pub highest_price: f64,
    pub lowest_price: f64,
    pub meet_high: bool,
    pub meet_low: bool,
}

pub async fn anal_doji_in_swing_all_companies(
    company_map: &CompanyMap,
    year_month_from: &str,
    year_month_to: &str,
    year_month_date: &str,
) -> Vec<DojiInSwingResult> {
    let mut doji_in_swing_results = Vec::new();

    let results = scripts::doji::anal_date_all_companies(company_map, year_month_date).await;

    for result in &results {
        // get range analysis
        let range_result = analysis::range::anal_range_high_low_company(
            company_map,
            &result.stock_no,
            year_month_from,
            year_month_to,
        )
        .await;

        let mut meet_high = false;
        let mut meet_low = false;

        if range_result.highest_price >= result.daily_data.close * 1.3 {
            meet_high = true;
        }

        if range_result.lowest_price <= result.daily_data.close * 0.7 {
            meet_low = true;
        }

        if meet_high || meet_low {
            let doji_in_swing_result = DojiInSwingResult {
                stock_no: result.stock_no.clone(),
                daily_data: result.daily_data.clone(),
                highest_price: range_result.highest_price,
                lowest_price: range_result.lowest_price,
                meet_high,
                meet_low,
            };
            doji_in_swing_results.push(doji_in_swing_result);
        }
    }
    doji_in_swing_results.sort_by(|a, b| b.daily_data.volume.cmp(&a.daily_data.volume));

    doji_in_swing_results
}
