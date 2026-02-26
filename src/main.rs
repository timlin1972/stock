mod analysis;
mod api;
mod cfg;
mod common;
mod conditions;
mod consts;
mod data;
mod menu;
mod monitor;
mod storage;

#[tokio::main]
async fn main() {
    let mut stocks = data::stocks::Stocks::new().await;

    menu::main::main_menu(&mut stocks).await;
}
