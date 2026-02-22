use std::collections::HashMap;

use crate::cfg::CfgData;
use crate::data::company::Company;
use crate::data::company_map::CompanyMap;

const MODULE_NAME: &str = "data::stocks";

pub struct Stocks {
    cfg: CfgData,
    pub company_map: CompanyMap,
    pub companies: HashMap<String, Company>,
}

impl Stocks {
    pub async fn new() -> Self {
        let cfg = CfgData::new();
        let company_map = CompanyMap::new().await;
        let mut companies = HashMap::new();

        println!("[{MODULE_NAME}] Reading data for all companies...");
        for company in &company_map.company_list {
            companies.insert(
                company.stock_no.clone(),
                Company::new(company.stock_no.clone()),
            );
        }

        Stocks {
            cfg,
            company_map,
            companies,
        }
    }

    pub async fn fetch_year(&mut self, year: &str) {
        println!("[{MODULE_NAME}] Fetching data for all companies for year: {year}...");
        let mut index = 1;
        let total = self.companies.len();
        for company in self.companies.values_mut() {
            println!(
                "[{MODULE_NAME}] [{index}/{total}] Fetching and writing data year({year}) for stock: {} ({})...",
                company.stock_no,
                self.company_map.get_name(&company.stock_no)
            );
            company.fetch_year(&self.cfg, year).await;

            // sleep to avoid hitting API rate limits
            tokio::time::sleep(std::time::Duration::from_millis(950)).await;
            index += 1;
        }
    }
}
