use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::error::Error;

use clap::{App, Arg, ArgMatches};

fn init_cli() -> ArgMatches<'static> {
    return App::new("nhdl")
        .version("0.1.3")
        .author("j1nxie (rylieeeeexd@gmail.com)")
        .arg(Arg::with_name("INPUT")
            .help("nhentai id / url")
            .index(1))
        .arg(Arg::with_name("batch-file")
            .help("process a list of id / url")
            .short("a")
            .long("batch-file")
            .value_name("FILE")
            .default_value("-"))
        .get_matches();
}

fn batch(batch_file: String) -> Result<Vec<String>, Box<dyn Error>> {
    if batch_file != "-".to_string() {
        let f = File::open(batch_file);
        match f {
            Ok(f) => {
                let f = BufReader::new(f);
                let mut urls = vec![];
                for line in f.lines() {
                    match line.as_ref().unwrap().trim().parse::<u32>() {
                        Ok(id) => urls.push(format!("https://nhentai.net/g/{}/", id)),
                        Err(_) => match line.as_ref().unwrap().trim().contains("https://nhentai.net/g/") {
                            true => urls.push(line.unwrap()),
                            false => println!("[error] invalid url or id!")
                        }
                    }
                }
                Ok(urls)
            },
            Err(_) => panic!("[error] unable to read file!")
        }
    } else {
        Ok(vec![])
    }
}

pub fn get_input() -> Vec<String> {
    let args = init_cli();
    let input = args.value_of("INPUT").unwrap().to_string();
    let batch_file = args.value_of("batch-file").unwrap().to_string();
    let mut urls = batch(batch_file).unwrap();
    loop {
        match &input.trim().parse::<u32>() {
            Ok(id) => {
                urls.push(format!("https://nhentai.net/g/{}/", id));
                break;
            },
            Err(_) => {
                match &input.trim().contains("https://nhentai.net/g/") {
                    true => {
                        urls.push(input);
                        break;
                    },
                    false => println!("[error] invalid url or id!")
                }
            }
        }
    }

    return urls;
}
