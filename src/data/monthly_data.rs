use std::fmt;
use std::fs;
use std::fs::File;
use std::io::BufWriter;

use chrono::{Datelike, Duration, Local, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::common;
use crate::twse;
use crate::twse::company_map::CompanyMap;

const DATA_DIR: &str = "data";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DailyData {
    pub date: String,      //民國日期 (113+1911 = 西元 2024 年)
    pub volume: u64,       //成交股數
    pub amount: u64,       //成交金額
    pub open: f64,         //開盤價 (早上 9:00)
    pub high: f64,         //最高價
    pub low: f64,          //最低價
    pub close: f64,        //收盤價 (下午 1:30)
    pub change: f64,       //與前一個交易日收盤價相比的差額
    pub transactions: u64, //總共撮合成功的交易次數
    pub note: String,      //備註
}

impl DailyData {
    pub fn print(&self, company_map: &CompanyMap, stock_no: &str) {
        println!(
            "{:<10}{:<6}{:>10}{:>8.2}{:>8.2}{:>8.2}{:>8.2}{:>8.2}  {:<20}",
            self.date,
            stock_no,
            common::format_commas(common::divide_by_1000(self.volume)),
            self.open,
            self.close,
            self.high,
            self.low,
            self.change,
            company_map.get(stock_no),
        );
    }
}

impl fmt::Display for DailyData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<10}{:>10}{:>8.2}{:>8.2}{:>8.2}{:>8.2}{:>8.2}",
            self.date,
            common::format_commas(common::divide_by_1000(self.volume)),
            self.open,
            self.close,
            self.high,
            self.low,
            self.change,
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthlyData {
    pub stock_no: String,
    pub year_month: String,
    pub daily_data: Vec<DailyData>,
}

impl MonthlyData {
    pub async fn new(company_map: &CompanyMap, stock_no: &str, year_month: &str) -> Self {
        let mut fetch_again = false;

        // if file is not exists or cannot be parsed, fetch again
        let path = format!("{DATA_DIR}/{stock_no}/{year_month}.json");
        if let Ok(file) = File::open(&path) {
            let reader = std::io::BufReader::new(file);
            if let Ok(monthly_data) = serde_json::from_reader::<_, MonthlyData>(reader) {
                // println!("Loaded data for {stock_no}/{year_month} from storage.");
                if is_in_year_month(year_month) {
                    let current_nearest_workday = nearest_workday(Local::now().date_naive());
                    let current_roc_date = common::naive_to_roc_date(current_nearest_workday);
                    // if current_roc_date is not in daily_data, then fetch again
                    if !monthly_data
                        .daily_data
                        .iter()
                        .any(|d| d.date == current_roc_date)
                    {
                        println!(
                            "Data {} for {stock_no}/{year_month} is outdated, fetching fresh data...",
                            current_roc_date
                        );
                        fetch_again = true;
                    }
                }
            } else {
                println!(
                    "Failed to parse data from storage for {stock_no}/{year_month}, fetching fresh data..."
                );
                fetch_again = true;
            }
        } else {
            println!("No stored data for {stock_no}/{year_month}, fetching fresh data...");
            fetch_again = true;
        }

        if fetch_again {
            let twse_response =
                twse::stock_data::TwseResponse::new(company_map, stock_no, year_month).await;

            let mut daily_data = Vec::new();
            for entry in twse_response.data.unwrap_or_default() {
                // 113/02/01", "46,924,943", "29,237,425,981", "625.00", "628.00", "619.00", "628.00", " 0.00", "51,671", ""
                // println!("Raw entry: {:?}", entry);
                let daily_data_item = DailyData {
                    date: entry[0].clone(),
                    volume: entry[1].replace(",", "").parse().unwrap_or(0),
                    amount: entry[2].replace(",", "").parse().unwrap_or(0),
                    open: entry[3].replace(",", "").parse().unwrap_or(0.0),
                    high: entry[4].replace(",", "").parse().unwrap_or(0.0),
                    close: entry[6].replace(",", "").parse().unwrap_or(0.0),
                    low: entry[5].replace(",", "").parse().unwrap_or(0.0),
                    change: entry[7].replace(",", "").parse().unwrap_or(0.0),
                    transactions: entry[8].replace(",", "").parse().unwrap_or(0),
                    note: entry[9].clone(),
                };

                daily_data.push(daily_data_item);
            }

            let monthly_data = MonthlyData {
                stock_no: stock_no.to_string(),
                year_month: year_month.to_string(),
                daily_data,
            };

            monthly_data.write_to_storage();
            monthly_data
        } else {
            // read from storage
            let path = format!("{DATA_DIR}/{stock_no}/{year_month}.json");
            let file = File::open(&path).unwrap();
            let reader = std::io::BufReader::new(file);
            serde_json::from_reader::<_, MonthlyData>(reader).unwrap()
        }
    }

    pub fn write_to_storage(&self) {
        // if DATA_DIR does not exist, create it
        if fs::metadata(DATA_DIR).is_err() {
            fs::create_dir_all(DATA_DIR).unwrap();
        }

        // create stock_no folder if not exists
        let stock_folder = format!("{}/{}", DATA_DIR, self.stock_no);
        if fs::metadata(&stock_folder).is_err() {
            fs::create_dir_all(&stock_folder).unwrap();
        }

        let path = format!("{}/{}/{}.json", DATA_DIR, self.stock_no, self.year_month);
        println!("Writing data to {}", path);

        // Implementation for writing data to storage
        let file = File::create(path).unwrap();
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, self).unwrap();
    }

    /*
    pub fn print_summary(&self, company_map: &twse::company_map::CompanyMap) {
        println!(
            "{} ({}) / {}",
            self.stock_no,
            company_map.get(&self.stock_no),
            self.year_month
        );
        println!(
            "{:<6}{:<8}{:>6}{:>5}{:>5}{:>5}{:>5}{:>6}",
            "台股", "日期", "成交股數", "開盤價", "收盤價", "最高價", "最低價", "漲跌"
        );
        for daily in &self.daily_data {
            println!("{}", daily);
        }
    }
     */
}

fn is_in_year_month(year_month: &str) -> bool {
    // 取得今天日期
    let today = Local::now().date_naive();

    // 把今天的年月組成字串，例如 202601
    let current_year_month = format!("{:04}{:02}", today.year(), today.month());

    current_year_month == year_month
}

fn nearest_workday(date: NaiveDate) -> NaiveDate {
    match date.weekday() {
        chrono::Weekday::Sat => date - Duration::days(1), // 昨天 (週五)
        chrono::Weekday::Sun => date - Duration::days(2), // 前天 (週五)
        _ => date,                                        // 週一到週五 → 今天
    }
}
