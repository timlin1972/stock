use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::api::twse;

const MODULE_NAME: &str = "data::company_map";
const COMPANY_LIST: &str = "company_list.json";

#[derive(Debug, Deserialize, Serialize)]
pub struct CompanyInfo {
    #[serde(rename = "公司代號")]
    pub stock_no: String,
    #[serde(rename = "公司簡稱")]
    pub name: String,
    #[serde(rename = "產業別")]
    pub industry: String,
}

impl fmt::Display for CompanyInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.stock_no)
    }
}

pub struct CompanyMap {
    pub company_list: Vec<CompanyInfo>,
    industry_map: HashMap<String, String>,
}

impl fmt::Display for CompanyMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        let unknown_industry = "未知產業".to_string();
        for company in &self.company_list {
            let industry_name = self
                .industry_map
                .get(&company.industry)
                .unwrap_or(&unknown_industry);
            output.push_str(&format!(
                "{} ({}) - {}\n",
                company.name, company.stock_no, industry_name
            ));
        }
        write!(f, "{}", output)
    }
}

impl CompanyMap {
    pub async fn new() -> Self {
        let industry_map = build_industry_map();
        let company_list = get_company_list().await;

        CompanyMap {
            company_list,
            industry_map,
        }
    }

    pub fn get_name(&self, stock_no: &str) -> String {
        for company in &self.company_list {
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

        panic!("[{MODULE_NAME}] Cannot find company name for stock no: {stock_no}");
    }
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

async fn get_company_list() -> Vec<CompanyInfo> {
    if Path::new(COMPANY_LIST).exists() {
        let file = File::open(COMPANY_LIST).unwrap();
        let reader = io::BufReader::new(file);
        let company_list: Vec<CompanyInfo> = serde_json::from_reader(reader).unwrap();
        return company_list;
    }

    let mut company_list = twse::fetch().await.unwrap();

    // 過濾掉不需要的股票
    company_list.retain(|company| {
        // skip 金融保險業
        company.industry != "17"
    });

    company_list.sort_by(|a, b| a.stock_no.cmp(&b.stock_no));

    // Save the company list to a JSON file for future use
    let json = serde_json::to_string_pretty(&company_list).unwrap();
    std::fs::write(COMPANY_LIST, json).unwrap();

    company_list
}
