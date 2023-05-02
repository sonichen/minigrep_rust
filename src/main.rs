use minigrep::Config;
use std::env;
use std::process;
//主程序
fn main() {
    let args:Vec<String>=env::args().collect();

    let config=Config::new(&args).unwrap_or_else(|err|
        {
            // println!("Problem parsing arguments:{}",err);
            eprintln!("Problem parsing arguments:{}",err);
            process::exit(0);//终止程序
        }
    );
    if let Err(e)=minigrep::run(config){
        // println!("Application error:{}",e);
        eprintln!("Application error:{}",e);
        process::exit(0);
    }
}
