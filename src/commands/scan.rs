use alloy::{
    hex,
    primitives::{Address, FixedBytes},
    providers::{Provider,ProviderBuilder},
    rpc::types::Filter,
    sol,
    sol_types::SolEvent
};
use eth_stealth_gas_tickets::{BlindedSignature, UnsignedTicket};
use std::fs;
use std::io::Write;
use std::path::Path;
use crate::commands::finalize::FinalizeInput;

sol! {
    #[sol(rpc)]
    interface IStealthGasStation {
        event SendGasTickets(bytes32[] ids, bytes[] signed);
        event NativeTransfers(uint256[] amounts, address[] targets, bytes d);
    }
}

/// Main function to scan the blockchain and match ticket IDs
pub async fn run(
    rpc_url: String,
    contract: String,
    unsigned_tickets_file: String,
    start_block: u64,
    output_path: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // Load the unsigned tickets from the file
    let all_unsigned_tickets: Vec<UnsignedTicket> =
        serde_json::from_str(&fs::read_to_string(unsigned_tickets_file)?)?;
    let ticket_ids: Vec<FixedBytes<32>> = all_unsigned_tickets.iter().map(|t| t.id).collect();

    // Set up the provider
    let provider = ProviderBuilder::new()
        .on_http(rpc_url.parse().unwrap());

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
                    unsigned_tickets.push(all_unsigned_tickets.iter().find(|t| t.id == *id).unwrap().clone());
                }
            }
        }
    }

    println!("Scan Finished. Found {} matching tickets.", blind_signatures.len());

    // Create the FinalizeInput struct
    let finalize_input = FinalizeInput {
        unsigned_tickets,
        blind_signatures,
    };

    if Path::new(&output_path).exists() {
        panic!("Output file {} already exists", output_path);
    }

    // Write the finalize input to the output file
    let mut file = fs::File::create(&output_path).expect("Failed to create output file");
    let json_output = serde_json::to_string_pretty(&finalize_input)?;
    file.write_all(json_output.as_bytes()).expect("Failed to write output file");

    println!("Results written to {}", output_path);

    Ok(())
}
