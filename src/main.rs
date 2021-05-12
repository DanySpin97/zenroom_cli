use std::{
    ffi::CString,
    fs,
    io::{self, Read},
};

use anyhow::Result;
use clap::Clap;
use zenroom::{zencode_exec, zenroom_exec};

#[derive(Clap)]
#[clap(version = "0.1", author = "Danilo Spinella <oss@danyspin97.org>")]
struct Opts {
    #[clap(short = 'c', long)]
    config: Option<String>,
    #[clap(short = 'a', long)]
    data: Option<String>,
    #[clap(short = 'k', long)]
    keys: Option<String>,
    script: Option<String>,
    #[clap(short = 'z', long)]
    zencode: bool,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let script = if let Some(script) = opts.script {
        CString::new(fs::read(script)?)?
    } else {
        let mut script = String::new();
        let mut stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_to_string(&mut script)?;
        CString::new(script)?
    };

    println!("{}", script.clone().into_string()?);

    let config = if let Some(config) = opts.config {
        CString::new(fs::read(config)?)?
    } else {
        CString::new("")?
    };

    let data = if let Some(data) = opts.data {
        CString::new(fs::read(data)?)?
    } else {
        CString::new("")?
    };

    let keys = if let Some(keys) = opts.keys {
        CString::new(fs::read(keys)?)?
    } else {
        CString::new("")?
    };

    let (res, success) = if opts.zencode {
        zencode_exec(script, config, keys, data)
    } else {
        zenroom_exec(script, config, keys, data)
    };

    println!("{}", res.output);
    println!("{}", res.logs);

    Ok(())
}
