use ethers::prelude::*;
fn main() {
    Abigen::new("UniV2Router", "./uniswap-v2-abi.json")
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file("uni-v2-router.rs")
        .unwrap();
}
