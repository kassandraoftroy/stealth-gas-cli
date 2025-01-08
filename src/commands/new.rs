use eth_stealth_gas_tickets::TicketsVerifier;
use rand::thread_rng;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn run(pubkey_hex: String, count: usize, output_path: String) {
    let pubkey = TicketsVerifier::from_hex_string(&pubkey_hex).expect("Invalid public key");
    let mut rng = thread_rng();

    let tickets = pubkey
        .new_blind_tickets(&mut rng, count)
        .expect("Failed to generate tickets");

    // Check if file exists first
    if Path::new(&output_path).exists() {
        panic!("Output file {} already exists", output_path);
    }

    let json = serde_json::to_string_pretty(&tickets).expect("Failed to serialize tickets");
    let mut file = File::create(&output_path).expect("Failed to create output file");
    file.write_all(json.as_bytes()).expect("Failed to write to file");

    println!("Generated {} tickets and saved to {}", count, output_path);
}
