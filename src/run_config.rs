use std::env;
use std::io;
use std::io::{Error, ErrorKind};

pub struct Config {
    pub file_in: String,
    pub file_out: String,
    pub mode: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> io::Result<Config> {
        args.next(); //skip app name

        let err_not_enough_params = "Not enough actual parameters";

        let mode = match args.next() {
            Some(arg) => arg,
            None => return Err(Error::new(ErrorKind::Other, err_not_enough_params)),
        };

        let file_in = match args.next() {
            Some(arg) => arg,
            None => return Err(Error::new(ErrorKind::Other, err_not_enough_params)),
        };

        let file_out = match args.next() {
            Some(arg) => arg,
            None => return Err(Error::new(ErrorKind::Other, err_not_enough_params)),
        };

        if &mode != "compress" && &mode != "decompress" {
            return Err(io::Error::new(ErrorKind::Other, "Unknown mode"));
        }

        Ok(Config {
            file_in,
            file_out,
            mode,
        })
    }
}
