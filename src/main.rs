use anchor_client::solana_sdk::pubkey::Pubkey;
use anyhow::Result;
use clap::Parser;
mod eightfish_cli;

#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(long)]
    pid: Pubkey,
}

#[tokio::main]
async fn main() -> Result<()> {
    let result = eightfish_cli::main().await;
    result
}
