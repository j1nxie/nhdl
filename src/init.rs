use std::{
    error::Error,
    fs::File,
    io::{prelude::*, BufReader},
};

use clap::{App, Arg, ArgMatches};

use tracing::error;

fn init_cli() -> clap::Result<ArgMatches<'static>> {
    return App::new("nhdl")
        .version("0.1.4")
        .author("j1nxie (rylieeeeexd@gmail.com)")
        .arg(
            Arg::with_name("INPUT")
                .help("nhentai id / url")
                .index(1)
                .required_unless("batch-file"),
        )
        .arg(
            Arg::with_name("batch-file")
                .help("process a list of id / url")
                .short("a")
                .long("batch-file")
                .value_name("FILE")
                .required_unless("INPUT"),
        )
        .get_matches_safe();
}

fn batch(batch_file: String) -> Result<Vec<String>, Box<dyn Error>> {
    if batch_file != *"-" {
        let f = File::open(batch_file);
        match f {
            Ok(f) => {
                let f = BufReader::new(f);
                let mut urls = vec![];
                for line in f.lines() {
                    match line.as_ref().unwrap().trim().parse::<u32>() {
                        Ok(id) => urls.push(format!("https://nhentai.net/g/{}/", id)),
                        Err(_) => match line
                            .as_ref()
                            .unwrap()
                            .trim()
                            .contains("https://nhentai.net/g/")
                        {
                            true => urls.push(line.unwrap()),
                            false => {
                                error!("error while parsing id: {}", line.as_ref().unwrap().trim())
                            }
                        },
                    }
                }
                Ok(urls)
            }
            Err(_) => panic!("unable to read file!"),
        }
    } else {
        Ok(vec![])
    }
}

pub fn get_input() -> Vec<String> {
    let args = init_cli().unwrap_or_else(|e| e.exit());
    let input;
    let batch_file;
    let mut urls = vec![];

    if let Some(text) = args.value_of("batch-file") {
        batch_file = text;
        urls = batch(batch_file.to_string()).unwrap();
    }

    if let Some(text) = args.value_of("INPUT") {
        input = text;
        match &input.trim().parse::<u32>() {
            Ok(id) => urls.push(format!("https://nhentai.net/g/{}/", id)),
            Err(_) => match &input.trim().contains("https://nhentai.net/g/") {
                true => urls.push(input.to_string()),
                false => error!("error while parsing id: {}", input),
            },
        }
    }

    urls
}
