use std::{
    env,
    fs::{create_dir, read_dir, File},
};

use clap::Parser;
use dotenv::dotenv;
use ethers::prelude::*;
use serde_json::Value;
use tokio::io::Result;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    contract_address: String,

    // #[clap(short, long)]
    // output_folder: String,

    // #[clap(long)]
    // contract_name: String,
    #[clap(long, default_value = "etherscan")]
    chain_prefix: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let api_key = env::var("API_KEY").expect("Please add API_KEY env variable");
    let args = Args::parse();

    let url = format!(
        "https://api.{}.io/api\
        ?module=contract\
        &action=getabi\
        &address={}\
        &apikey={}",
        args.chain_prefix, args.contract_address, api_key
    );
    let resp = reqwest::get(url).await.unwrap().text().await.unwrap();

    let mut json: Value = serde_json::from_str(&resp).unwrap();
    let result_str = json["result"].as_str().unwrap();
    json = serde_json::from_str(&result_str).unwrap();

    let filename = "example";

    let dir = "./generated";

    match read_dir(dir) {
        Ok(_) => (),
        Err(_) => {
            create_dir(dir).unwrap();
            ()
        }
    }

    serde_json::to_writer(
        &File::create(format!("{}/{}.json", dir, filename)).unwrap(),
        &json,
    )
    .unwrap();

    Abigen::new(filename, format!("{}/{}.json", dir, filename))
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file(format!("{}/{}.rs", dir, filename))
        .unwrap();

    Ok(())
}
