use crate::TakeValue::*;
use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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
                            .required(true)
                    )
                    .arg(

                        Arg::with_name("lines")
                            .short("n")
                            .long("lines")
                            .value_name("LINES")
                            .default_value("10")
                    )
                    .arg(

                        Arg::with_name("bytes")
                            .short("c")
                            .long("bytes")
                            .value_name("BYTES")
                            .conflicts_with("lines")
                    )
                    .arg(
                        Arg::with_name("quiet")
                            .short("q")
                            .takes_value(false)
                    )
                    .get_matches();
    
    let files = matches.values_of_lossy("files").unwrap();
    let lines = parse_num_1(&matches.value_of("lines").unwrap())
                                                .map_err(|e| {
                                                    format!("illegal line count -- {}",e)
                                                }
    )?;
    let mut bytes = None;

    if matches.is_present("bytes") {

        bytes = Some(parse_num_1(&matches.value_of("bytes").unwrap())
                        .map_err(|e| {
                            format!("illegal byte count -- {}",e)
                        })?);
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

   for filename in config.files {

    match File::open(&filename) {

        Err(err) => eprintln!("{}: {}",filename,err),
        Ok(_) =>{
            
            let (total_lines, total_bytes) = count_lines_and_bytes(&filename)?;
            println!("{} has {} lines and {} bytes", filename, total_lines, total_bytes);
        }
    }
   }
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

// Function that takes in a filename and returns a count of total lines and bytes in it

fn count_lines_and_bytes(filename: &str) -> MyResult<(usize, usize)> {
    
        let mut file = Box::new(BufReader::new(File::open(filename)?));
        let mut contents = String::new();
        let mut num_lines = 0;
        let mut num_bytes = 0;
        loop {

            let bytes = file.read_line(&mut contents)?;
            if bytes == 0 {
                break;
            }
            num_lines+=1;
            num_bytes+= bytes;
            contents.clear();
        }
        
        let lines = num_lines;
        let bytes = num_bytes;
        Ok((lines, bytes))
    
}


/*
A function called parse_num_1 that takes in a &str value and returns a TakeValue
The function should return a PlusZero if string is '+0' else it should return a TakeNum
with the value of the string parsed as an i64 with negative wrapping
Any error should be returned with string value of the number
*/
fn parse_num_1(num: &str) -> MyResult<TakeValue> {

    let mut x;
    if num == "+0" {

        x = PlusZero;
    }
    else {

        x = TakeNum(num.parse()?);
    }

    Ok(x)

}

/*
A function called parse_num_2 that takes in a &str value and returns a TakeValue
The function should return a PlusZero if string is '+0' else it should return a TakeNum
with the value of the string (strings without '+' should be treated as negative numbers) parsed as an i64 
Any error should be returned with string value of the number
*/


