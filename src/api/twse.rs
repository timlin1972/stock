use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::data::company_map::CompanyInfo;

#[derive(Debug, Deserialize, Serialize)]
pub struct TwseCompanyInfo {
    #[serde(rename = "SecuritiesCompanyCode")]
    pub stock_no: String,
    #[serde(rename = "CompanyAbbreviation")]
    pub name: String,
    #[serde(rename = "SecuritiesIndustryCode")]
    pub industry: String,
}

/// 從 TWSE API 抓取上市公司代號 → 中文名稱
pub async fn fetch() -> Result<Vec<CompanyInfo>, Box<dyn Error>> {
    let mut company_info_map = Vec::new();

    let url = "https://www.tpex.org.tw/openapi/v1/mopsfin_t187ap03_O";
    let resp = reqwest::get(url)
        .await?
        .json::<Vec<TwseCompanyInfo>>()
        .await?;

    for company in &resp {
        company_info_map.push(CompanyInfo {
            stock_no: company.stock_no.clone(),
            name: company.name.clone(),
            industry: company.industry.clone(),
        });
    }

    let url = "https://openapi.twse.com.tw/v1/opendata/t187ap03_L";
    let resp = reqwest::get(url).await?.json::<Vec<CompanyInfo>>().await?;

    company_info_map.extend(resp);

    company_info_map.sort_by(|a, b| a.stock_no.cmp(&b.stock_no));

    Ok(company_info_map)
}
