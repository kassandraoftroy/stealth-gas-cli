use crate::commands::utils::get_default_url;
use alloy::primitives::{Address, U256};
use dirs;
use eth_stealth_gas_tickets::SignedTicket;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub struct Spend {
    pub amount: U256,
    pub receiver: Address,
}

#[derive(Deserialize)]
pub struct SpendInput {
    pub amount: String,
    pub receiver: Address,
}

#[derive(Serialize)]
pub struct SpendRequest {
    pub signatures: Vec<SignedTicket>,
    pub spends: Vec<Spend>,
}

pub async fn run(
    url: Option<String>,
    input: Option<String>,
    spends_json: String,
    chain_id: Option<u64>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get chain ID and defaults
    let chain_id = chain_id.unwrap_or(17000);

    // Use provided values or defaults
    let url = url.unwrap_or(get_default_url(chain_id));
    let mut input_path = input.unwrap_or_else(|| "".to_string());

    // If input path is empty, use default path in ~/.stealthereum
    if input_path.is_empty() {
        let home_dir = dirs::home_dir().expect("Could not find home directory");
        let stealth_dir = home_dir.join(".stealthereum");
        input_path = stealth_dir
            .join(format!("finalized_tickets_{}.json", chain_id))
            .to_str()
            .expect("Failed to convert path to string")
            .to_string();
    }

    // Load signed tickets
    let signed_tickets: Vec<SignedTicket> =
        serde_json::from_str(&fs::read_to_string(&input_path)?)?;

    // Load spends from raw JSON
    let spends: Vec<SpendInput> = serde_json::from_str(&spends_json)?;

    // Convert SpendInput to Spend
    let spends: Vec<Spend> = spends
        .into_iter()
        .map(|s| Spend {
            amount: U256::from_str(&s.amount).unwrap(),
            receiver: s.receiver,
        })
        .collect();

    // Create spend request
    let spend_request = SpendRequest {
        signatures: signed_tickets,
        spends,
    };

    // Send POST request
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/redeem", url))
        .json(&spend_request)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Redemption successful!");
        println!("Response: {}", response.text().await?);
    } else {
        println!("Redemption failed with status: {}", response.status());
        println!("Error: {}", response.text().await?);
    }

    Ok(())
}
