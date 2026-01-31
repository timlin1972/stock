use chrono::{Datelike, NaiveDate};

pub fn to_roc_date(date_str: &str) -> String {
    // 解析字串成 NaiveDate
    let date = NaiveDate::parse_from_str(date_str, "%Y%m%d").ok().unwrap();

    // 民國年 = 西元年 - 1911
    let roc_year = date.year() - 1911;
    let month = date.month();
    let day = date.day();

    format!("{}/{:02}/{:02}", roc_year, month, day)
}

pub fn naive_to_roc_date(date: NaiveDate) -> String {
    let roc_year = date.year() - 1911; // 民國年 = 西元年 - 1911
    format!("{}/{:02}/{:02}", roc_year, date.month(), date.day())
}

pub fn format_commas(value: u64) -> String {
    let s = value.to_string();
    let bytes = s.as_bytes();
    let mut result = String::new();
    let len = bytes.len();
    for (i, &b) in bytes.iter().enumerate() {
        result.push(b as char);
        if (len - i - 1).is_multiple_of(3) && i != len - 1 {
            result.push(',');
        }
    }
    result
}

pub fn divide_by_1000(value: u64) -> u64 {
    (value as f64 / 1000.0) as u64
}
