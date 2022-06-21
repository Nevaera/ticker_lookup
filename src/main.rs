
extern crate getopts;
use std::env;
use exitfailure::ExitFailure;
//My imports
pub mod lib;
pub mod api_interact;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    //env::set_var("RUST_BACKTRACE", "1");

    //Get Commandline args
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    //Process OPTS/FLAGS
    let mut opts = lib::OptsSet::new();
    opts.set();
    opts.parse(&args);
    
    //Process FLAGS
    if opts.is_help() { 
        opts.print(&program);
    } else if opts.api_key() == "none" { //API key is needed for everything except looking at help
        panic!("\n\tPlease specify your API Key using [-k / -key]!\n  To get an API Key please go to: https://finnhub.io/register\n  (This program only needs a FREE key.)\n");
    } else if opts.is_list() { 
        api_interact::print_exchanges(opts.api_key()).await?; 
    } else {
              
        //Process what happens if neither -t or -i is used
        let mut tickers = Vec::new();
        if opts.in_file() != "none" {
            println!("Opening {}", opts.in_file());
        } else if opts.ticker() != "none"{
            tickers.push(opts.ticker());
        } else {
            //Neither -t or -i options used
            panic!("\n\tNo ticker or input CSV file specified, please use -h/--help for program options!\n");
        }
        if opts.is_quote() {
            //Quotes
            if opts.is_stock() {
                //Stocks
                for t in &tickers{
                    let res = api_interact::StockQuote::get(t, opts.api_key()).await?;
                    if opts.is_verbose() {
                        println!("{} [ Current: ${:.2}, Change: ${:.2}({}%), Hi/Lo: ${:.2}/${:.2}, Open: ${:.2}, Prev. Close: ${:.2} ]", t, res.c(), res.d(), res.dp(), res.h(), res.l(), res.o(), res.pc());
                    } else {
                        println!("{},{},{},{},{},{},{},{}", t, res.c(), res.d(), res.dp(), res.h(), res.l(), res.o(), res.pc());
                    }
                }
            } else {
                //Crypto
                println!("The API does not support quotes for Crypto (remove -q/--quote to access crypto candles)");
            }
        } else {
            //Candles
            if opts.is_stock() {
                //Stocks
                println!("ToDo: Implement stock candles");
                /*for t in &tickers{
                    
                }*/
            } else {
                //Crypto
                println!("ToDo: Implement crypto candles");
            }
        }
    }
    Ok(())
}
