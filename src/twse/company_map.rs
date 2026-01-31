use std::collections::HashMap;

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use serde::Deserialize;

fn read_lines_to_vec<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut result = Vec::new();

    for line in reader.lines() {
        if let Ok(l) = line {
            // 如果有註解，用 "//" 切開，只取前半段
            let clean = l.split("//").next().unwrap().trim();
            if !clean.is_empty() {
                result.push(clean.to_string());
            }
        }
    }

    Ok(result)
}

pub struct CompanyMap {
    pub stock_map: Vec<CompanyInfo>,
    industry_map: HashMap<String, String>,
}

impl CompanyMap {
    pub async fn new() -> Self {
        let industry_map = build_industry_map();

        let ignore_stocks = read_lines_to_vec("ignore_stocks.txt").unwrap();
        let mut stock_map = fetch_twse_company_map(&ignore_stocks).await.unwrap();

        stock_map.sort_by(|a, b| a.stock_no.cmp(&b.stock_no));

        CompanyMap {
            stock_map,
            industry_map,
        }
    }

    pub fn get(&self, stock_no: &str) -> String {
        for company in &self.stock_map {
            if company.stock_no == stock_no {
                return format!(
                    "{}/{}",
                    company.name,
                    self.industry_map
                        .get(&company.industry)
                        .unwrap_or(&"未知產業".to_string())
                );
            }
        }

        panic!("Cannot find company name for stock no: {stock_no}");
    }
}

#[derive(Debug, Deserialize)]
pub struct CompanyInfo {
    #[serde(rename = "公司代號")]
    pub stock_no: String,
    #[serde(rename = "公司簡稱")]
    pub name: String,
    #[serde(rename = "產業別")]
    pub industry: String,
}

/// 從 TWSE API 抓取上市公司代號 → 中文名稱
async fn fetch_twse_company_map(
    ignore_stocks: &Vec<String>,
) -> Result<Vec<CompanyInfo>, Box<dyn Error>> {
    let url = "https://openapi.twse.com.tw/v1/opendata/t187ap03_L";
    let resp = reqwest::get(url).await?.json::<Vec<CompanyInfo>>().await?;

    let mut ret_results = Vec::new();

    for company in resp {
        if ignore_stocks.contains(&company.stock_no) {
            continue;
        }
        if company.industry == "17" {
            // skip 金融保險業
            continue;
        }

        ret_results.push(company);
    }

    Ok(ret_results)
}
fn build_industry_map() -> HashMap<String, String> {
    let industries = vec![
        ("01", "水泥工業"),
        ("02", "食品工業"),
        ("03", "塑膠工業"),
        ("04", "紡織纖維"),
        ("05", "電機機械"),
        ("06", "電器電纜"),
        ("08", "玻璃陶瓷"),
        ("09", "造紙工業"),
        ("10", "鋼鐵工業"),
        ("11", "橡膠工業"),
        ("12", "汽車工業"),
        ("13", "電子工業"),
        ("14", "建材營造業"),
        ("15", "航運業"),
        ("16", "觀光餐旅"),
        ("17", "金融保險業"),
        ("18", "貿易百貨業"),
        ("19", "綜合"),
        ("20", "其他業"),
        ("21", "化學工業"),
        ("22", "生技醫療業"),
        ("23", "油電燃氣業"),
        ("24", "半導體業"),
        ("25", "電腦及週邊設備業"),
        ("26", "光電業"),
        ("27", "通信網路業"),
        ("28", "電子零組件業"),
        ("29", "電子通路業"),
        ("30", "資訊服務業"),
        ("31", "其他電子業"),
        ("32", "文化創意業"),
        ("33", "農業科技業"),
        ("34", "電子商務"),
        ("35", "綠能環保"),
        ("36", "數位雲端"),
        ("37", "運動休閒"),
        ("38", "居家生活"),
    ];

    // 將 Vec 轉換為 HashMap
    industries
        .into_iter()
        .map(|(code, name)| (code.to_string(), name.to_string()))
        .collect()
}
