use crate::data::monthly_data::MonthlyData;
use crate::twse::company_map::CompanyMap;

#[derive(Debug)]
pub struct RangeHighLow {
    // pub stock_no: String,
    // pub year_month_from: String,
    // pub year_month_to: String,
    pub highest_price: f64,
    pub lowest_price: f64,
}

pub async fn anal_range_high_low_company(
    company_map: &CompanyMap,
    stock_no: &str,
    year_month_from: &str,
    year_month_to: &str,
) -> RangeHighLow {
    let (year_str, month_str) = year_month_from.split_at(4);
    let mut year: u32 = year_str.parse().unwrap();
    let mut month: u32 = month_str.parse().unwrap();

    let mut highest_price: f64 = 0.0;
    let mut lowest_price: f64 = f64::MAX;

    loop {
        let year_month = format!("{:04}{:02}", year, month);
        let monthly_data = MonthlyData::new(company_map, stock_no, &year_month).await;

        for daily in &monthly_data.daily_data {
            if daily.close > highest_price {
                highest_price = daily.close;
            }
            if daily.close < lowest_price {
                lowest_price = daily.close;
            }
        }

        if year_month == year_month_to {
            break;
        }

        // 月份 +1
        month += 1;
        if month > 12 {
            month = 1;
            year += 1;
        }
    }

    RangeHighLow {
        // stock_no: stock_no.to_string(),
        // year_month_from: year_month_from.to_string(),
        // year_month_to: year_month_to.to_string(),
        highest_price,
        lowest_price,
    }
}
