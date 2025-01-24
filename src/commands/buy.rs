use alloy::{
    hex,
    primitives::{U256, Address, Bytes},
    providers::ProviderBuilder,
    network::EthereumWallet,
    sol,
    signers::local::PrivateKeySigner,
};
use eth_stealth_gas_tickets::UnsignedTicket;
use serde_json;
use std::fs;
use crate::commands::utils::{get_default_rpc, get_default_contract_address};

sol! {
    #[sol(rpc)]
    interface IStealthGasStation {
        function ticketCost() external view returns (uint256);
        function shippingCost() external view returns (uint256);
        function buyGasTickets(bytes[] calldata blindedMessages) external payable;
    }
}

pub async fn run(rpc_url: Option<String>, contract_address: Option<String>, input: Option<String>, private_key: Option<String>, account: Option<String>, chain_id: Option<u64>) -> Result<(), Box<dyn std::error::Error>> {
    // Get chain ID and defaults
    let chain_id = chain_id.unwrap_or(17000);

    // Use provided values or defaults
    let rpc_url = rpc_url.unwrap_or(get_default_rpc(chain_id));
    let contract_address = contract_address.unwrap_or(get_default_contract_address(chain_id));
    let input = input.ok_or("Input file must be provided")?;

    if private_key.is_none() && account.is_none() {
        return Err("Either private key or account path must be provided".into());
    }
    if private_key.is_some() && account.is_some() {
        return Err("Only one of private key or account path can be provided".into());
    }

    // Parse contract address
    let contract_address = Address::from_slice(&hex::decode(contract_address.replace("0x", ""))?);

    // Set up the provider and wallet
    let eth_signer = if let Some(private_key) = private_key {
        private_key.parse().expect("Failed to parse private key")
    } else if let Some(account) = account {
        if !std::fs::exists(&account)? {
            return Err("Account file does not exist".into());
        };
        let password = rpassword::prompt_password("Enter keystore password:")?;
        PrivateKeySigner::decrypt_keystore(account, password).expect("failed to unlock keystore")
    } else {
        return Err("Neither private key or keystore provided".into())
    };
    let signer_provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(EthereumWallet::from(eth_signer))
        .on_http(rpc_url.parse()?);

    // Create contract instance
    let contract = IStealthGasStation::new(contract_address, signer_provider.clone());

    // Load unsigned tickets
    let unsigned_tickets: Vec<UnsignedTicket> = serde_json::from_str(&fs::read_to_string(input)?)?;
    
    // Get costs from contract
    let ticket_cost = contract.ticketCost().call().await?._0;
    let shipping_cost = contract.shippingCost().call().await?._0;
    
    // Calculate total cost
    let total_cost = ticket_cost * U256::from(unsigned_tickets.len()) + shipping_cost;

    // Prepare blinded messages for contract call
    let blinded_messages: Vec<Bytes> = unsigned_tickets
        .iter()
        .map(|t| Bytes::from(t.blind_msg.clone()))
        .collect();

    // Create and send transaction
    let tx = contract
        .buyGasTickets(blinded_messages)
        .value(total_cost)
        .send()
        .await?;

    println!("Transaction sent! Hash: {}", tx.tx_hash());

    Ok(())
}
