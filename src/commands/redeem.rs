use eth_stealth_gas_tickets::SignedTicket;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use alloy::primitives::{U256, Address};
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

pub async fn run(url: String, input: String, spends_json: String) -> Result<(), Box<dyn std::error::Error>> {
    // Load signed tickets
    let signed_tickets: Vec<SignedTicket> = serde_json::from_str(&fs::read_to_string(input)?)?;
    
    // Load spends from raw JSON
    let spends: Vec<SpendInput> = serde_json::from_str(&spends_json)?;

    // Convert SpendInput to Spend
    let spends: Vec<Spend> = spends.into_iter().map(|s| Spend {
        amount: U256::from_str(&s.amount).unwrap(),
        receiver: s.receiver,
    }).collect();

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

