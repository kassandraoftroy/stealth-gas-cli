use crate::commands::utils::{get_default_contract_address, get_default_pubkey, get_default_rpc};
use alloy::{
    hex,
    primitives::{Address, FixedBytes},
    providers::{Provider, ProviderBuilder},
    rpc::types::Filter,
    sol,
    sol_types::SolEvent,
};
use dirs;
use eth_stealth_gas_tickets::{BlindedSignature, TicketsVerifier, UnsignedTicket};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;

sol! {
    #[sol(rpc)]
    interface IStealthGasStation {
        event SendGasTickets(bytes32[] ids, bytes[] signed);
        event NativeTransfers(uint256[] amounts, address[] targets, bytes d);
    }
}

#[derive(Serialize, Deserialize)]
pub struct FinalizeInput {
    pub unsigned_tickets: Vec<UnsignedTicket>,
    pub blind_signatures: Vec<BlindedSignature>,
}

pub async fn run(
    pubkey: Option<String>,
    input: Option<String>,
    output: Option<String>,
    rpc: Option<String>,
    contract_address: Option<String>,
    start_block: u64,
    chain_id: Option<u64>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get chain ID and defaults
    let chain_id = chain_id.unwrap_or(17000);

    // Use provided values or defaults
    let rpc_url = rpc.unwrap_or(get_default_rpc(chain_id));
    let contract = contract_address.unwrap_or(get_default_contract_address(chain_id));
    let pubkey_hex = pubkey.unwrap_or(get_default_pubkey(chain_id));
    let mut input_path = input.unwrap_or_else(|| "".to_string());
    let mut output_path = output.unwrap_or_else(|| "".to_string());

    // If input path is empty, use default path in ~/.stealthereum
    if input_path.is_empty() {
        let home_dir = dirs::home_dir().expect("Could not find home directory");
        let stealth_dir = home_dir.join(".stealthereum");
        input_path = stealth_dir
            .join(format!("unsigned_tickets_{}.json", chain_id))
            .to_str()
            .expect("Failed to convert path to string")
            .to_string();
    }

    // If output path is empty, use default path in ~/.stealthereum
    if output_path.is_empty() {
        let home_dir = dirs::home_dir().expect("Could not find home directory");
        let stealth_dir = home_dir.join(".stealthereum");
        if !stealth_dir.exists() {
            std::fs::create_dir_all(&stealth_dir)
                .expect("Failed to create .stealthereum directory");
        }
        output_path = stealth_dir
            .join(format!("finalized_tickets_{}.json", chain_id))
            .to_str()
            .expect("Failed to convert path to string")
            .to_string();
    }

    // Load the unsigned tickets from the file
    let all_unsigned_tickets: Vec<UnsignedTicket> =
        serde_json::from_str(&fs::read_to_string(&input_path)?)?;
    let ticket_ids: Vec<FixedBytes<32>> = all_unsigned_tickets.iter().map(|t| t.id).collect();

    // Set up the provider
    let provider = ProviderBuilder::new().on_http(rpc_url.parse().unwrap());

    let contract_address = Address::from_slice(&hex::decode(contract.replace("0x", "")).unwrap());

    // Set up the event filter for SendGasTickets
    let filter = Filter::new()
        .address(vec![contract_address])
        .event_signature(IStealthGasStation::SendGasTickets::SIGNATURE_HASH)
        .from_block(start_block);

    println!("Scanning for events from block {}", start_block);

    // Fetch and filter events
    let logs = provider.get_logs(&filter).await?;
    let mut blind_signatures = Vec::new();
    let mut unsigned_tickets = Vec::new();
    for log in logs {
        if let Ok(decoded) = log.log_decode::<IStealthGasStation::SendGasTickets>() {
            for (id, signed_data) in decoded.inner.ids.iter().zip(decoded.inner.signed.iter()) {
                if ticket_ids.contains(id) {
                    blind_signatures.push(BlindedSignature {
                        id: *id,
                        blind_sig: signed_data.clone(),
                    });
                    unsigned_tickets.push(
                        all_unsigned_tickets
                            .iter()
                            .find(|t| t.id == *id)
                            .unwrap()
                            .clone(),
                    );
                }
            }
        }
    }

    println!(
        "Scan Finished. Found {} matching tickets.",
        blind_signatures.len()
    );

    // Initialize the ticket verifier
    let pubkey = TicketsVerifier::from_hex_string(&pubkey_hex).expect("Invalid public key");

    // Finalize the tickets
    let signed_tickets = pubkey
        .finalize_tickets(unsigned_tickets, blind_signatures)
        .expect("Failed to finalize tickets");

    if Path::new(&output_path).exists() {
        panic!("Output file {} already exists", output_path);
    }

    // Write the signed tickets to the output JSON file
    let json =
        serde_json::to_string_pretty(&signed_tickets).expect("Failed to serialize signed tickets");
    let mut output_file = fs::File::create(&output_path).expect("Failed to create output file");
    output_file
        .write_all(json.as_bytes())
        .expect("Failed to write to file");

    println!("Finalized tickets and saved to {}", output_path);

    Ok(())
}
