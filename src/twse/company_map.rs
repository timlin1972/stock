use std::collections::HashMap;
use std::error::Error;

use serde::Deserialize;

pub struct CompanyMap {
    stock_map: HashMap<String, String>,
    alias_map: HashMap<&'static str, &'static str>,
}

impl CompanyMap {
    pub async fn new() -> Self {
        let stock_map = fetch_twse_company_map().await.unwrap();
        let alias_map = build_alias_map();

        CompanyMap {
            stock_map,
            alias_map,
        }
    }

    pub fn get(&self, stock_no: &str) -> String {
        if let Some(full_name) = self.stock_map.get(stock_no) {
            if let Some(&alias) = self.alias_map.get(full_name.as_str()) {
                alias.to_string()
            } else {
                full_name.clone()
            }
        } else {
            panic!("Cannot find company name for stock no: {stock_no}");
        }
    }
}

#[derive(Debug, Deserialize)]
struct CompanyInfo {
    #[serde(rename = "公司代號")]
    stock_no: String,
    #[serde(rename = "公司名稱")]
    name: String,
}

/// 從 TWSE API 抓取上市公司代號 → 中文名稱
async fn fetch_twse_company_map() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let url = "https://openapi.twse.com.tw/v1/opendata/t187ap03_L";
    let resp = reqwest::get(url).await?.json::<Vec<CompanyInfo>>().await?;

    let mut map = HashMap::new();
    for company in resp {
        map.insert(company.stock_no, company.name);
    }

    Ok(map)
}

fn build_alias_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    // 半導體
    map.insert("台灣積體電路製造股份有限公司", "台積電");
    map.insert("聯華電子股份有限公司", "聯電");
    map.insert("聯發科技股份有限公司", "聯發科");
    map.insert("日月光投控股份有限公司", "日月光");
    map.insert("力晶科技股份有限公司", "力晶");

    // 電子/硬體
    map.insert("鴻海精密工業股份有限公司", "鴻海");
    map.insert("廣達電腦股份有限公司", "廣達");
    map.insert("仁寶電腦工業股份有限公司", "仁寶");
    map.insert("和碩聯合科技股份有限公司", "和碩");
    map.insert("華碩電腦股份有限公司", "華碩");
    map.insert("宏碁股份有限公司", "宏碁");

    // 金融
    map.insert("國泰金融控股股份有限公司", "國泰金");
    map.insert("富邦金融控股股份有限公司", "富邦金");
    map.insert("中信金融控股股份有限公司", "中信金");
    map.insert("玉山金融控股股份有限公司", "玉山金");
    map.insert("兆豐金融控股股份有限公司", "兆豐金");
    map.insert("第一金融控股股份有限公司", "第一金");
    map.insert("台新金融控股股份有限公司", "台新金");

    // 其他產業
    map.insert("台灣水泥股份有限公司", "台泥");
    map.insert("亞洲水泥股份有限公司", "亞泥");
    map.insert("中華電信股份有限公司", "中華電");
    map.insert("台灣高鐵股份有限公司", "高鐵");
    map.insert("長榮海運股份有限公司", "長榮");
    map.insert("陽明海運股份有限公司", "陽明");
    map.insert("萬海航運股份有限公司", "萬海");
    map.insert("台灣塑膠工業股份有限公司", "台塑");
    map.insert("南亞塑膠工業股份有限公司", "南亞");
    map.insert("台灣化學纖維股份有限公司", "台化");
    map.insert("台灣石油化學股份有限公司", "台石化");

    map
}
