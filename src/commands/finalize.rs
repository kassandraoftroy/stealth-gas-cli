use eth_stealth_gas_tickets::{BlindedSignature, TicketsVerifier, UnsignedTicket};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct FinalizeInput {
    pub unsigned_tickets: Vec<UnsignedTicket>,
    pub blind_signatures: Vec<BlindedSignature>,
}

pub fn run(pubkey_hex: String, input_path: String, output_path: String) {
    let pubkey = TicketsVerifier::from_hex_string(&pubkey_hex).expect("Invalid public key");

    // Read the input JSON file
    let input_file = File::open(&input_path).expect("Failed to open input file");
    let reader = BufReader::new(input_file);
    let finalize_input: FinalizeInput =
        serde_json::from_reader(reader).expect("Failed to parse input JSON");

    // Finalize the tickets
    let signed_tickets = pubkey
        .finalize_tickets(
            finalize_input.unsigned_tickets,
            finalize_input.blind_signatures,
        )
        .expect("Failed to finalize tickets");

    if Path::new(&output_path).exists() {
        panic!("Output file {} already exists", output_path);
    }

    // Write the signed tickets to the output JSON file
    let json = serde_json::to_string_pretty(&signed_tickets).expect("Failed to serialize signed tickets");
    let mut output_file = File::create(&output_path).expect("Failed to create output file");
    output_file
        .write_all(json.as_bytes())
        .expect("Failed to write to file");

    println!("Finalized tickets and saved to {}", output_path);
}
