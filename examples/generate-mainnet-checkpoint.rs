use std::{error::Error, str::FromStr, sync::Arc};

use ethers::{
    providers::{Ws, Provider},
    types::H160,
};

use cfmms::{
    checkpoint::generate_checkpoint,
    dex::{Dex, DexVariant},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //Add rpc endpoint here:
    let rpc_endpoint = "ws://localhost:8545";
    let ws = Ws::connect(rpc_endpoint).await.unwrap();
    let provider = Arc::new(Provider::new(ws));

    let dexes = vec![
        //Add Uniswap V2
        Dex::new(
            H160::from_str("0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f").unwrap(),
            DexVariant::UniswapV2,
            2638438,
            Some(300),
        ),
    ];

    // Sync pools and generate checkpoint
    generate_checkpoint(dexes, provider.clone(), "checkpoint.json").await?;

    Ok(())
}
