pub struct OptsSet {
    api_key:    String,// = "none".to_string()        
    ticker:     String,
    exchange:   String,
    in_file:    String,
    out_file:   String,
    is_stock:   bool,
    is_list:    bool,
    is_verbose: bool,// = false;
    is_help:    bool,
    opts:       getopts::Options,
}
impl OptsSet {
    //Methods
    pub fn new() -> OptsSet{
        OptsSet{
            api_key:    "none".to_string(),
            ticker:     "none".to_string(),
            exchange:   "none".to_string(),
            in_file:    "none".to_string(),
            out_file:   "none".to_string(),
            is_stock:   false,
            is_list:    false,
            is_verbose: false,
            is_help:    false,
            opts:       getopts::Options::new()
        }
    }
    pub fn set(&mut self){
        //Set up the program flags and options
        self.opts.optopt("k", "apikey", "(required) specifies your api key", "APIKEY");
        self.opts.optopt("t", "ticker", "(required unless using -i) specifies the ticker to be used", "TICKER");
        self.opts.optopt("e", "exchange", "(required - crypto) specifies the exchange from which to look up tickers", "EXCHNG");
        self.opts.optopt("i", "infile", "(required unless using -t) specifies an input file containing comma-separated tickers", "INFL");
        self.opts.optopt("o", "outfile", "(optional) specifies an output file to save comma-separated outputs", "OUTFL");
        self.opts.optflag("s", "stock", "specifies whether the program must look up crypto (default) or stock (with -s/--stock)");
        self.opts.optflag("l", "list", "displays available crypto exchanges");
        self.opts.optflag("v", "verbose", "displays more information on the terminal duing outputs");
        self.opts.optflag("h", "help", "displays this help menu");
    }
    pub fn parse(&mut self, args: &Vec<String>){
        //Parse the args passed into the program to set options/flags
        let matches = match self.opts.parse(&args[1..]) {
            Ok(m) => { m }
            Err(_e) => { panic!("Please use the -h /--help for usage instructions") }
        };
        //Process FLAGS
        match matches.opt_present("h") { true => self.is_help    = true, false => self.is_help    = false }
        match matches.opt_present("l") { true => self.is_list    = true, false => self.is_list    = false }
        match matches.opt_present("s") { true => self.is_stock   = true, false => self.is_stock   = false }
        match matches.opt_present("v") { true => self.is_verbose = true, false => self.is_verbose = false }
        //Process OPTS
        match matches.opt_str("k") { Some(x) => self.api_key = x,  None => self.api_key  = "none".to_string() }
        match matches.opt_str("t") { Some(x) => self.ticker = x,   None => self.ticker   = "none".to_string() }
        match matches.opt_str("i") { Some(x) => self.in_file = x,  None => self.in_file  = "none".to_string() } 
        match matches.opt_str("o") { Some(x) => self.out_file = x, None => self.out_file = "none".to_string() }  
        match matches.opt_str("e") { Some(x) => self.exchange = x, None => self.exchange = "none".to_string() }   
    }
    pub fn print(&self, program: &str) {
        let brief = format!("Usage: {} [options]", program);
        print!("{}", self.opts.usage(&brief));
    }
    //Immutable accessors
    pub fn api_key(&self)    -> &str   { &self.api_key    }
    pub fn ticker(&self)     -> &str   { &self.ticker     }
    pub fn exchange(&self)   -> &str   { &self.exchange   }
    pub fn in_file(&self)    -> &str   { &self.in_file    }
    pub fn out_file(&self)   -> &str   { &self.out_file   }
    pub fn is_stock(&self)   -> bool   { self.is_stock    }        
    pub fn is_list(&self)    -> bool   { self.is_list     }
    pub fn is_verbose(&self) -> bool   { self.is_verbose  }
    pub fn is_help(&self)    -> bool   { self.is_help     }
}
