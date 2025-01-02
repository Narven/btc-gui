use rust_decimal::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct PriceResponse {
    data: CryptoRate,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct CryptoRate {
    rateUsd: String,
}

pub fn get_bitcoin_price() -> Result<Decimal, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://api.coincap.io/v2/rates/bitcoin")?;
    let body = resp.json::<PriceResponse>()?;

    let price = match Decimal::from_str(&body.data.rateUsd) {
        Ok(num) => num,
        Err(_) => {
            println!("Error on converting");
            
            Decimal::new(0, 1)
        }
    };

    Ok(price)
}
