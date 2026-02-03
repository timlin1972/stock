use std::error::Error;

use serde::{Deserialize, Serialize};
use tokio::time::Duration;

use crate::twse::company_map::CompanyMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TwseResponse {
    pub stat: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub data: Option<Vec<Vec<String>>>,
}

impl TwseResponse {
    pub async fn new(company_map: &CompanyMap, stock_no: &str, year_month: &str) -> Self {
        loop {
            let fetch_result = fetch_stock_data(stock_no, year_month).await.unwrap();
            let parsed: TwseResponse = serde_json::from_str(&fetch_result).unwrap();

            if parsed.stat != "OK" {
                if parsed.stat.contains("查詢日期大於")
                    || parsed.stat.contains("查詢日期小於")
                    || parsed.stat.contains("很抱歉，沒有符合條件的資料")
                {
                    println!(
                        "TWSE 回傳訊息: {} for stock_no: {}({}), year_month: {}. Try again!",
                        parsed.stat,
                        stock_no,
                        company_map.get(stock_no),
                        year_month
                    );
                    continue;
                } else {
                    panic!(
                        "TWSE 回傳錯誤狀態: {} for stock_no: {}({}), year_month: {}",
                        parsed.stat,
                        stock_no,
                        company_map.get(stock_no),
                        year_month
                    );
                }
            } else {
                return TwseResponse {
                    stat: parsed.stat,
                    title: parsed.title,
                    data: parsed.data,
                };
            }
        }
    }
}

pub async fn fetch_stock_data(stock_no: &str, year_month: &str) -> Result<String, Box<dyn Error>> {
    let date_str = format!("{year_month}01"); // TWSE API 需要完整日期

    let url = format!(
        "https://www.twse.com.tw/exchangeReport/STOCK_DAY?response=json&date={}&stockNo={}",
        date_str, stock_no
    );

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36")
        .header("Accept", "application/json")
        .send()
        .await?;

    // 先拿到原始文字，避免解析 JSON 失敗時看不到原因
    let body_text = response.text().await?;

    // 檢查是否被封鎖 (通常被封鎖會回傳 HTML)
    if body_text.contains("<html>") {
        return Err("被 TWSE 暫時封鎖 IP 了（回傳了 HTML）。請增加延遲時間或更換 IP。".into());
    }

    Ok(body_text)
}
