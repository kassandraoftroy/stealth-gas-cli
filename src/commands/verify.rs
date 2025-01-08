use eth_stealth_gas_tickets::{TicketsVerifier, SignedTicket};
use serde_json::from_reader;
use std::fs::File;
use std::io::BufReader;

pub fn run(pubkey_hex: String, input_path: String) {
    let pubkey = TicketsVerifier::from_hex_string(&pubkey_hex).expect("Invalid public key");

    // Read the input JSON file
    let input_file = File::open(&input_path).expect("Failed to open input file");
    let reader = BufReader::new(input_file);
    let signed_tickets: Vec<SignedTicket> =
        from_reader(reader).expect("Failed to parse signed tickets JSON");

    // Verify the signed tickets
    pubkey
        .verify_signed_tickets(signed_tickets)
        .expect("Ticket verification failed");

    println!("Ticket verification passed!");
}
