mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "stealth-gas")]
#[command(about = "CLI for managing Ethereum blind gas tickets on client side", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate new unsigned gas tickets
    New {
        #[arg(short = 'p', long = "pubkey", help = "Coordinator public key")]
        pubkey: Option<String>,
        #[arg(short = 'n', long = "num", help = "Number of tickets to generate")]
        num: Option<usize>,
        #[arg(short = 'o', long = "output", help = "Output JSON file path of Vec<UnsignedTicket> type")]
        output: Option<String>,
        #[arg(short = 'c', long = "chain-id", help = "Chain ID")]
        chain_id: Option<u64>,
    },
    /// Finalize blind signed tickets to generate redeemable gas tickets (includes scanning)
    Finalize {
        #[arg(short = 'p', long = "pubkey", help = "Coordinator public key")]
        pubkey: Option<String>,
        #[arg(short = 'i', long = "input", help = "Input JSON file path of FinalizeInput type (unsigned tickets and blind signatures)")]
        input: Option<String>,
        #[arg(short = 'o', long = "output", help = "Output JSON file path of Vec<SignedTicket> type (redeemable gas tickets)")]
        output: Option<String>,
        #[arg(short = 'r', long = "rpc-url", help = "RPC URL (for a supported chain id)")]
        rpc: Option<String>,
        #[arg(short = 'g', long = "gas-station-address", help = "StealthGasStation contract address")]
        gas_station_address: Option<String>,
        #[arg(short = 's', long = "start-block", help = "Start block")]
        start_block: u64,
        #[arg(short = 'c', long = "chain-id", help = "Chain ID")]
        chain_id: Option<u64>,
    },
    /// Verify signatures of finalized gas tickets
    Verify {
        #[arg(short = 'p', long = "pubkey", help = "Coordinator public key")]
        pubkey: Option<String>,
        #[arg(short = 'i', long = "input", help = "Input JSON file path of Vec<SignedTicket> type (redeemable gas tickets)")]
        input: String,
        #[arg(short = 'c', long = "chain-id", help = "Chain ID")]
        chain_id: Option<u64>,
    },
    /// Fetch public params for the StealthGasStation on a supported chain
    Params {
        #[arg(short = 'r', long = "rpc-url", help = "RPC URL (for a supported chain id)")]
        rpc: Option<String>,
        #[arg(short = 'c', long = "chain-id", help = "Chain ID")]
        chain_id: Option<u64>,
    },
    /// Buy gas tickets on-chain
    Buy {
        #[arg(short = 'r', long = "rpc-url", help = "RPC URL (for a supported chain id)")]
        rpc: Option<String>,
        #[arg(short = 'g', long = "gas-station-address", help = "StealthGasStation contract address")]
        contract_address: Option<String>,
        #[arg(short = 'i', long = "input", help = "Input JSON file path of Vec<UnsignedTicket> type")]
        input: Option<String>,
        #[arg(short = 'k', long = "key", help = "Private key for transaction signing")]
        key: Option<String>,
        #[arg(short = 'a', long = "account", help = "Path to keystore file for transaction signing")]
        account: Option<String>,
        #[arg(short = 'c', long = "chain-id", help = "Chain ID")]
        chain_id: Option<u64>,
    },
    /// Redeem signed tickets through coordinator
    Redeem {
        #[arg(short = 'u', long = "url", help = "Coordinator URL endpoint")]
        url: Option<String>,
        #[arg(short = 'i', long = "input", help = "Input JSON file path of Vec<SignedTicket> type")]
        input: Option<String>,
        #[arg(short = 's', long = "spends", help = "JSON containing spend requests [{\"amount\": string, \"receiver\": string}]")]
        spends: String,
        #[arg(short = 'c', long = "chain-id", help = "Chain ID")]
        chain_id: Option<u64>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { pubkey, num, output, chain_id } => Ok(commands::new::run(pubkey, num, output, chain_id)),
        Commands::Finalize { pubkey, input, output, rpc, gas_station_address, start_block, chain_id } => 
            commands::finalize::run(pubkey, input, output, rpc, gas_station_address, start_block, chain_id).await,
        Commands::Verify { pubkey, input, chain_id } => Ok(commands::verify::run(pubkey, input, chain_id)),
        Commands::Params { rpc, chain_id } => commands::params::run(rpc, chain_id).await,
        Commands::Buy { rpc, contract_address, input, key, account, chain_id } => 
            commands::buy::run(rpc, contract_address, input, key, account, chain_id).await,
        Commands::Redeem { url, input, spends, chain_id } => 
            commands::redeem::run(url, input, spends, chain_id).await,
    }
}


