
extern crate getopts;
use getopts::Options;
use std::env;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use reqwest::Url;
use exitfailure::ExitFailure;

#[derive(Serialize, Deserialize, Debug)]
struct StockQuote {
    c: f64,
    h: f64,
    l: f64,
    o: f64,
    pc: f64,
    t: i128,
}
impl StockQuote {
    async fn get(ticker: &String, apikey: &String) -> Result<Self, ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/quote?symbol={}&token={}",
            ticker, apikey
        );
        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<StockQuote>().await?;

        Ok(res)
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

async fn print_exchanges(apikey:&String) -> Result<(), ExitFailure> {
    if apikey.ne("none"){
        let url = format!("https://finnhub.io/api/v1/crypto/exchange?token={}", apikey);
        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<serde_json::Value>().await?.to_string();
        let res = res.replace(&['[',']','\"'], "");
        let res = res.replace(",", "\n");
        println!("Available Exchanges (use -e/--exchange)\n{}", res);        
        Ok(())
    }else{
        panic!("Please specify your API Key using [-k / -key]!\n  To get an API Key please go to: https://finnhub.io/register\n  (This program only needs a FREE key.)");
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    // this method needs to be inside main() method
    //env::set_var("RUST_BACKTRACE", "1");

    //Get Commandline args
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    //Variables
    let mut apikey:String = "none".to_string();
    let mut stock:bool = false;
    let mut ticker:String;
    let mut exchange:String;
    let mut infile:String;
    let mut outfile:String;
    let mut verbose:bool = false;
    
    //Add FLAGS
    let mut opts = Options::new();
    opts.optopt("k", "key", "(required) specifies your api key", "APIKEY");
    opts.optopt("t", "tick", "(required unless using -i) specifies the ticker to be used", "TICKER");
    opts.optopt("e", "exchange", "(required - crypto) specifies the exchange from which to look up tickers", "EXCHNG");
    opts.optopt("i", "infile", "(required unless using -t) specifies an input file containing comma-separated tickers", "INFL");
    opts.optopt("o", "outfile", "(optional) specifies an output file to save comma-separated outputs", "OUTFL");
    opts.optflag("s", "stock", "specifies whether the program must look up crypto (default) or stock (with -s/--stock)");
    opts.optflag("l", "list", "displays available crypto exchanges");
    opts.optflag("v", "verbose", "displays more information on the terminal duing outputs");
    opts.optflag("h", "help", "displays this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(e) => { panic!("Please use the -h /--help for usage instructions") }
    };

    match matches.opt_str("k") { Some(x) => apikey = x, None => apikey = "none".to_string() }
    
    //Process FLAGS
    if matches.opt_present("h") { 
        print_usage(&program, opts);
    } else if matches.opt_present("l") { 
        print_exchanges(&apikey).await?; 
    } else{
        if matches.opt_present("s") { stock = true; };
        if matches.opt_present("v") { verbose = true; }
        
        //Process OPTS
        match matches.opt_str("t") { Some(x) => ticker = x, None => ticker = "none".to_string() }
        match matches.opt_str("i") { Some(x) => infile = x, None => infile = "none".to_string() } 
        //Process what happens if neither t or i is used

        match matches.opt_str("e") { Some(x) => exchange = x, None => exchange = "none".to_string() }
        //Check if stock
            // If not stock exchange must be specified
            // Use -k -l to list exchanges
        match matches.opt_str("o") { Some(x) => outfile = x, None => outfile = "none".to_string() } 
        

        if stock{
            let res = StockQuote::get(&ticker, &apikey).await?;
            println!("{} stock price: {}USD", ticker, res.c);
        } else {
            // Crypto
            println!("Crypto!");
        }
    }
    Ok(())
}
