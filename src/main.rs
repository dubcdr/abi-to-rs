// use ethers::prelude::*;

#[tokio::main]
async fn main() {
    // Abigen::new("UniV2Router", "./uniswap-v2-abi.json")
    //     .unwrap()
    //     .generate()
    //     .unwrap()
    //     .write_to_file("uni-v2-router.rs")
    //     .unwrap();

    let url = format!(
        "https://api.{}.io/api\
        ?module=contract\
        &action=getabi\
        &address={}\
        &apikey={}",
    );
    let resp = reqwest::get(url).await;
}
