use std::{error::Error, str::FromStr, sync::Arc};

use ethers::{
    providers::{Ws, Provider},
    types::H160,
};

use cfmms::{
    dex::{Dex, DexVariant},
    sync,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //Add rpc endpoint here:
    let rpc_endpoint = "ws://localhost:8545";
    let ws = Ws::connect(rpc_endpoint).await.unwrap();
    let provider = Arc::new(Provider::new(ws));

    let dexes = vec![
        //UniswapV2
        Dex::new(
            H160::from_str("0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f").unwrap(),
            DexVariant::UniswapV2,
            2638438,
            Some(300),
        ),
        //Add Sushiswap
        Dex::new(
            H160::from_str("0xC0AEe478e3658e2610c5F7A4A2E1777cE9e4f2Ac").unwrap(),
            DexVariant::UniswapV2,
            10794229,
            Some(300),
        ),
        //Add UniswapV3
        Dex::new(
            H160::from_str("0x1F98431c8aD98523631AE4a59f267346ea31F984").unwrap(),
            DexVariant::UniswapV3,
            12369621,
            None,
        ),
    ];

    //Sync pairs
    sync::sync_pairs(dexes, provider, None).await?;
    println!("good job!");
    Ok(())
}
