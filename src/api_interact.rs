
use serde_derive::{Deserialize, Serialize};
use serde_json;
use exitfailure::ExitFailure;
use reqwest::Url;
    
#[derive(Serialize, Deserialize, Debug)]
pub struct StockQuote {
    c:  f64,
    d:  f64,
    dp: f64,
    h:  f64,
    l:  f64,
    o:  f64,
    pc: f64,
    t:  i128,
    error: String,
}
impl StockQuote {
    //Methods
    pub async fn get(ticker: &str, api_key: &str) -> Result<Self, ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/quote?symbol={}&token={}",
            ticker, api_key
        );
        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<StockQuote>().await?;

        Ok(res)
    }
    //Immutable accessors
    pub fn c(&self)  -> f64  { self.c  }
    pub fn d(&self)  -> f64  { self.d  }
    pub fn dp(&self) -> f64  { self.dp }
    pub fn h(&self)  -> f64  { self.h  }
    pub fn l(&self)  -> f64  { self.l  }
    pub fn o(&self)  -> f64  { self.o  }
    pub fn pc(&self) -> f64  { self.pc }
    pub fn t(&self)  -> i128 { self.t  }
    pub fn error(&self) -> &str { &self.error }
}

pub async fn print_exchanges(api_key:&str) -> Result<(), ExitFailure> {    
    let url = format!("https://finnhub.io/api/v1/crypto/exchange?token={}", api_key);
    let url = Url::parse(&*url)?;
    let res = reqwest::get(url).await?.json::<serde_json::Value>().await?.to_string();
    if res.contains("error") {
        panic!("\n\tError looking up Exchanges! Please check your API key\n");
    } else {
        let res = res.replace(&['[',']','\"'], "");
        let res = res.replace(",", "\n\t");
        print!("Available Exchanges (for use with -e/--exchange):\n\t{}", res);
    }        
    Ok(())    
}
