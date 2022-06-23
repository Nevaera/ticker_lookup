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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Candles {
    c:  Vec<f64>,
    h:  Vec<f64>,
    l:  Vec<f64>,
    o:  Vec<f64>,
    s:  String,
    t:  Vec<i128>,
    v:  Vec<i128>,
}
impl Candles {
    //Methods
    pub async fn get_stock(ticker: &str, resolution: &str, from: u128, to: u128, api_key: &str) -> Result<Self, ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/stock/candle?symbol={}&resolution={}&from={}&to={}&token={}",
            ticker, resolution, from, to, api_key
        );
        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<Candles>().await?;
        if res.s != "ok"{
            panic!("\n\tError with Stock Candles! Please check your options (-e, -r)\n");
        }
        Ok(res)
    }
    pub async fn get_crypto(ticker: &str, resolution: &str, from: u128, to: u128, api_key: &str) -> Result<Self, ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/crypto/candle?symbol={}&resolution={}&from={}&to={}&token={}",
            ticker, resolution, from, to, api_key
        );
        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<Candles>().await?;

        Ok(res)
    }
    //Immutable accessors
    pub fn c(&self) -> &Vec<f64>  { &self.c  }
    pub fn h(&self) -> &Vec<f64>  { &self.h  }
    pub fn l(&self) -> &Vec<f64>  { &self.l  }
    pub fn o(&self) -> &Vec<f64>  { &self.o  }
    pub fn s(&self) -> &str       { &self.s  }
    pub fn t(&self) -> &Vec<i128> { &self.t  }
    pub fn v(&self) -> &Vec<i128> { &self.v  }
}

pub async fn print_exchanges(api_key: &str) -> Result<(), ExitFailure> {    
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

pub async fn print_symbols(exchange: &str, api_key: &str) -> Result<(), ExitFailure> {    
    let url = format!("https://finnhub.io/api/v1/forex/symbol?exchange={}&token={}", exchange, api_key);
    let url = Url::parse(&*url)?;
    let res = reqwest::get(url).await?.json::<serde_json::Value>().await?.to_string();
    if res.contains("error") {
        panic!("\n\tError looking up Exchanges! Please check your API key\n");
    } else {
        let res = res.replace(&['[',']','\"'], "");
        let res = res.replace(",", "\n\t");
        print!("Available Symbols @ {} (for use with -t/--ticker):\n\t{}", exchange, res);
    }        
    Ok(())    
}
