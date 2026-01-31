mod analysis;
mod common;
mod consts;
mod data;
mod menu;
mod scripts;
mod twse;

#[tokio::main]
async fn main() {
    let company_map = twse::company_map::CompanyMap::new().await;

    // let monthly_data = data::monthly_data::MonthlyData::new(&company_map, "2317", "202512").await;
    // monthly_data.print_summary(&company_map);

    // scripts::doji::anal_month_all_compies(&company_map, "202601").await;
    let results = scripts::doji::anal_date_all_companies(&company_map, "20260130").await;
    println!(
        "{:<8}{:<6}{:>4}{:>5}{:>5}{:>5}{:>5}{:>6} {}",
        "日期", "股號", "成交股數", "開盤價", "收盤價", "最高價", "最低價", "漲跌", "公司名稱"
    );
    for result in &results {
        result.print(&company_map);
    }

    menu::main_menu::main_menu(&company_map).await;
}
