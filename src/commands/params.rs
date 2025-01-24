use alloy::{
    hex,
    primitives::{U256, Address},
    providers::ProviderBuilder,
    sol,
};
use crate::commands::utils::{get_default_contract_address, get_default_url, get_default_rpc};

sol! {
    #[sol(rpc)]
    interface IStealthGasStation {
        function coordinatorPubKey() external view returns (bytes memory);
        function ticketCost() external view returns (uint256);
        function shippingCost() external view returns (uint256);
    }
}

// Convert U256 (wei) to a human-readable ETH amount as a string
fn u256_to_eth(wei: U256) -> String {
    // 10^18 in U256
    let eth_unit = U256::from(10).pow(U256::from(18));

    // Get the whole part and the remainder
    let whole = wei / eth_unit;
    let remainder = wei % eth_unit;

    // Convert remainder to a fractional part by scaling it to 18 decimal places
    let fractional_str = format!("{:018}", remainder);

    // Trim trailing zeros from the fractional part
    format!("{}.{} ETH", whole, fractional_str.trim_end_matches('0'))
}

pub async fn run(rpc_url: Option<String>, chain_id: Option<u64>) -> Result<(), Box<dyn std::error::Error>> {
    // Use provided chain ID or default to 17000
    let chain_id = chain_id.unwrap_or(17000);
    
    // Use provided RPC URL or get default based on chain ID
    let rpc_url = rpc_url.unwrap_or(get_default_rpc(chain_id));
    
    // Set up the provider using Arc for shared ownership
    let provider = ProviderBuilder::new().on_http(rpc_url.parse()?);

    let contract_address_string = get_default_contract_address(chain_id);
    let contract_address = Address::from_slice(&hex::decode(contract_address_string.replace("0x", "")).unwrap());
    // Create contract instance with shared provider
    let contract = IStealthGasStation::new(contract_address, provider.clone());

    // Fetch the public key from the contract
    let pubkey_return = contract.coordinatorPubKey().call().await?;

    let ticket_cost_return = contract.ticketCost().call().await?;
    let shipping_cost_return = contract.shippingCost().call().await?;

    // Print the hex-encoded public key
    println!("StealthGasStation contract: {}", contract_address);
    println!("Coordinator URL: {}", get_default_url(chain_id));
    println!("Ticket Cost: {}", u256_to_eth(ticket_cost_return._0));
    println!("Shipping Cost: {}", u256_to_eth(shipping_cost_return._0));
    println!("Coordinator PubKey: 0x{}", hex::encode(pubkey_return._0));

    Ok(())
}


