mod analysis;
mod api;
mod backtest;
mod cfg;
mod common;
mod consts;
mod data;
mod menu;
mod scripts;
mod storage;

#[tokio::main]
async fn main() {
    let mut stocks = data::stocks::Stocks::new().await;

    menu::main::main_menu(&mut stocks).await;
}
