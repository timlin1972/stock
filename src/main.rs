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

    menu::main_menu::main_menu(&company_map).await;
}
