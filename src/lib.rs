use regex::Regex;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{self, prelude::*};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    px2rem(config.ratio,&config.filename, &contents);
    Ok(())
}

pub struct Config {
    pub ratio: f64,
    pub filename: String,
    // pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let ratio: f64 = args[1].clone().parse::<f64>().unwrap();

        let filename = args[2].clone();
        // let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            ratio,
            filename,
        })
    }
}

pub fn px2rem<'a>(ratio:f64,filename:&'a str,contents: &'a str){
    let re = Regex::new(r"(?P<value>\d{1,5}px)").unwrap();

    let mut res = String::new();
    res = re
        .replace_all(&contents, |captures: &regex::Captures| {
            let value = &captures["value"];

            let v2: Vec<&str> = value.split("px").collect();
            let v3: f64 = v2[0].parse::<f64>().unwrap();
            let v4 = format!("{}rem", v3 / ratio);

            captures[0].replace(&captures["value"], v4.as_str())
        })
        .to_string();
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(filename).unwrap();
    file.write(res.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_px2rem() {
        

    }

    
}
