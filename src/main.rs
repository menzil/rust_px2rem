use px2rem::Config;
use std::env;
use std::process;

fn main(){
   let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem when parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Search {:?} from {:?} with case sensitive: ", config.ratio, config.filename);

    if let Err(e)=px2rem::run(config){
        eprintln!("Application error: {}",e);
        process::exit(1);
    }; 
}
