use crate::commands::utils::{get_default_pubkey, get_default_tickets_number};
use dirs;
use eth_stealth_gas_tickets::TicketsVerifier;
use rand::thread_rng;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn run(
    pubkey_hex: Option<String>,
    count: Option<usize>,
    output_path: Option<String>,
    chain_id: Option<u64>,
) {
    // Default to chain ID 17000 if not specified
    let chain_id = chain_id.unwrap_or(17000);

    let pubkey_hex = pubkey_hex.unwrap_or(get_default_pubkey(chain_id));
    let count = count.unwrap_or(get_default_tickets_number(chain_id));
    let mut output_path = output_path.unwrap_or_else(|| "".to_string());
    if output_path.is_empty() {
        // Create ~/.stealthereum directory if it doesn't exist
        let home_dir = dirs::home_dir().expect("Could not find home directory");
        let stealth_dir = home_dir.join(".stealthereum");
        if !stealth_dir.exists() {
            std::fs::create_dir_all(&stealth_dir)
                .expect("Failed to create .stealthereum directory");
        }
        output_path = stealth_dir
            .join(format!("unsigned_tickets_{}.json", chain_id))
            .to_str()
            .expect("Failed to convert path to string")
            .to_string();
    }

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
    file.write_all(json.as_bytes())
        .expect("Failed to write to file");

    println!("Generated {} tickets and saved to {}", count, output_path);
}
