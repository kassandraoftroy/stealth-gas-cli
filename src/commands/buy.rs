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

sol! {
    #[sol(rpc)]
    interface IStealthGasStation {
        function ticketCost() external view returns (uint256);
        function shippingCost() external view returns (uint256);
        function buyGasTickets(bytes[] calldata blindedMessages) external payable;
    }
}

pub async fn run(rpc_url: String, contract_address: String, input: String, private_key: String) -> Result<(), Box<dyn std::error::Error>> {
    // Parse contract address
    let contract_address = Address::from_slice(&hex::decode(contract_address.replace("0x", ""))?);

    // Set up the provider and wallet
    let eth_signer: PrivateKeySigner = private_key.parse().expect("Failed to parse private key");
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
