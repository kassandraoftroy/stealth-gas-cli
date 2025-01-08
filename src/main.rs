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
        #[arg(short = 'k', long = "key", help = "Coordinator public key")]
        key: String,
        #[arg(short = 'n', long = "num", help = "Number of tickets to generate")]
        num: usize,
        #[arg(short = 'o', long = "output", help = "Output JSON file of Vec<UnsignedTicket> type")]
        output: String,
    },
    /// Scan the chain for blind signatures that match your unsigned tickets
    Scan {
        #[arg(short = 'r', long = "rpc-url", help = "RPC URL (for a supported chain id)")]
        rpc: String,
        #[arg(short = 'c', long = "contract-address", help = "StealthGasStation contract address")]
        contract_address: String,
        #[arg(short = 'i', long = "input", help = "Input JSON file of Vec<UnsignedTickets> type")]
        input: String,
        #[arg(short = 's', long = "start-block", help = "Start block")]
        start: u64,
        #[arg(short = 'o', long = "output", help = "Output JSON file of FinalizedInput type")]
        output: String,
    },
    /// Finalize blind signed tickets to generate redeemable gas tickets
    Finalize {
        #[arg(short = 'k', long = "key", help = "Coordinator public key")]
        key: String,
        #[arg(short = 'i', long = "input", help = "Input JSON file of FinalizeInput type (unsigned tickets and blind signatures)")]
        input: String,
        #[arg(short = 'o', long = "output", help = "Output JSON file of Vec<SignedTicket> type (redeemable gas tickets)")]
        output: String,
    },
    /// Verify signatures of finalized gas tickets
    Verify {
        #[arg(short = 'k', long = "key", help = "Coordinator public key")]
        key: String,
        #[arg(short = 'i', long = "input", help = "Input JSON file of Vec<SignedTicket> type (redeemable gas tickets)")]
        input: String,
    },
    /// Fetch public params for the StealthGasStation on a supported chain
    Params {
        #[arg(short = 'r', long = "rpc-url", help = "RPC URL (for a supported chain id)")]
        rpc: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { key, num, output } => Ok(commands::new::run(key, num, output)),
        Commands::Finalize { key, input, output } => Ok(commands::finalize::run(key, input, output)),
        Commands::Verify { key, input } => Ok(commands::verify::run(key, input)),
        Commands::Scan { rpc, contract_address, input, start, output } => commands::scan::run(rpc, contract_address, input, start, output).await,
        Commands::Params { rpc } => commands::params::run(rpc).await,
    }
}


