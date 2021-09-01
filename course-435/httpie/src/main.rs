use std::str::FromStr;
use clap::Clap;
use anyhow::{anyhow, Result};
use reqwest::Url;

#[derive(Clap, Debug)]
#[clap(name = "httpie")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

#[derive(Clap, Debug)]
struct Get { 
    /// HTTP 请求的 URL
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}

#[derive(Clap, Debug)]
struct Post { 
    /// HTTP 请求的 URL
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    
    /// HTTP 请求的 body
    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
}

#[derive(Debug)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!("Failed to parse {}", s);
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
    
}

fn parse_url(s: &str) -> Result<String> {
    let _: Url = s.parse()?;
    Ok(s.into())
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}
