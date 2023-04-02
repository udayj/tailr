use crate::TakeValue::*;
use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box< dyn Error>>;

#[derive(Debug, PartialEq)]
enum TakeValue {

    PlusZero,
    TakeNum(i64),
}

#[derive(Debug)]
pub struct Config {

    files: Vec<String>,
    lines: TakeValue,
    bytes: Option<TakeValue>,
    quiet: bool,
}

pub fn get_args() -> MyResult<Config> {

    let matches = App::new("tailr")
                    .version("0.1.0")
                    .author("udayj")
                    .about("Rust tail")
                    .arg(

                        Arg::with_name("files")
                            .value_name("FILE")
                            .multiple(true)
                            .help("Input file(s)")
                            .default_value("-")
                    )
                    .arg(

                        Arg::with_name("lines")
                            .short("n")
                            .long("lines")
                            .takes_value(true)
                            .default_value("10")
                    )
                    .arg(

                        Arg::with_name("bytes")
                            .short("c")
                            .long("bytes")
                            .takes_value(true)
                    )
                    .arg(
                        Arg::with_name("quiet")
                            .short("q")
                            .takes_value(false)
                    )
                    .get_matches();
    
    let files = matches.values_of_lossy("files").unwrap();
    let lines = parse_num(&matches.value_of_lossy("lines").unwrap().into_owned())?;
    let mut bytes = None;

    if matches.is_present("bytes") {

        bytes = Some(parse_num(&matches.value_of_lossy("bytes").unwrap().into_owned())?);
    }

    let quiet = matches.is_present("quiet");

    Ok(Config {

        files,
        lines,
        bytes,
        quiet,


    })

}

// take the lines val
// match - if PlusZero then make it 0
// if TakeNum then get the num and parse it (for positive integers)

pub fn run(config: Config) -> MyResult<()> {

    println!("{:#?}", config);

    Ok(())
}

fn parse_num(num: &String) -> MyResult<TakeValue> {

    let mut x;
    if num.starts_with('+') {

        x = TakeNum(num.trim_start_matches('+').parse()?);
    }
    else {

        x = TakeNum(-num.parse()?);
    }

    Ok(x)

}