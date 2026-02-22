use std::fs;
use std::fs::File;
use std::io::BufWriter;

use crate::consts;
use crate::data::company::StockData;

const MODULE_NAME: &str = "storage";
const DATA_DIR: &str = "data";

pub fn read(stock_no: &str) -> Vec<StockData> {
    let mut stock_data = Vec::new();
    for year in consts::YEAR_FROM..=consts::YEAR_TO {
        let stock_data_file = format!("{DATA_DIR}/{stock_no}/{year}.json");
        // println!("[{MODULE_NAME}] Reading data from {stock_data_file}");
        if fs::metadata(&stock_data_file).is_ok() {
            let file = File::open(&stock_data_file).unwrap();
            let reader = std::io::BufReader::new(file);
            let stock_data_year: Vec<StockData> = serde_json::from_reader(reader).unwrap();
            stock_data.extend(stock_data_year);
        } else {
            println!(
                "[{MODULE_NAME}] Stock data file not found for stock: {stock_no}, year: {year}. Please run the fetch function first."
            );
        }
    }

    stock_data.sort_by(|a, b| a.date.cmp(&b.date));
    stock_data
}

pub fn save(stock_no: &str, year: &str, data: &Vec<StockData>) {
    let stock_data_file = format!("{DATA_DIR}/{stock_no}");
    if fs::metadata(&stock_data_file).is_err() {
        fs::create_dir_all(&stock_data_file).unwrap();
    }

    let data_company_file = format!("{stock_data_file}/{year}.json");

    let file = File::create(&data_company_file).unwrap();
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &data).unwrap();
}
