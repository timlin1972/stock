mod data;
mod twse;

#[tokio::main]
async fn main() {
    let company_map = twse::company_map::CompanyMap::new().await;

    let monthly_data = data::monthly_data::MonthlyData::new("2317", "202512").await;
    monthly_data.print_summary(&company_map);
}
