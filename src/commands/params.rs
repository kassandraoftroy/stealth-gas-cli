use alloy::{
    hex,
    primitives::{U256, Address},
    providers::{Provider, ProviderBuilder},
    sol,
};

sol! {
    #[sol(rpc)]
    interface IStealthGasStation {
        function coordinatorPubKey() external view returns (bytes memory);
        function ticketCost() external view returns (uint256);
        function shippingCost() external view returns (uint256);
    }
}

fn get_contract_address(chain_id: u64) -> Address {
    match chain_id {
        17000 => Address::from_slice(&hex::decode("0x943285f1a29281e59514fF35Dc16E5a14E123a27".replace("0x", "")).unwrap()),
        _ => panic!("Unsupported chain ID: {}", chain_id),
    }
}

fn get_url(chain_id: u64) -> String {
    match chain_id {
        17000 => "https://0000000000.org".to_string(),
        _ => panic!("Unsupported chain ID: {}", chain_id),
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

pub async fn run(rpc_url: String) -> Result<(), Box<dyn std::error::Error>> {
    // Set up the provider using Arc for shared ownership
    let provider = ProviderBuilder::new().on_http(rpc_url.parse()?);

    // Retrieve chain ID
    let chain_id = provider.get_chain_id().await?;
    let contract_address = get_contract_address(chain_id);

    // Create contract instance with shared provider
    let contract = IStealthGasStation::new(contract_address, provider.clone());

    // Fetch the public key from the contract
    let pubkey_return = contract.coordinatorPubKey().call().await?;

    let ticket_cost_return = contract.ticketCost().call().await?;
    let shipping_cost_return = contract.shippingCost().call().await?;

    // Print the hex-encoded public key
    println!("StealthGasStation contract: {}", contract_address);
    println!("Coordinator URL: {}", get_url(chain_id));
    println!("Ticket Cost: {}", u256_to_eth(ticket_cost_return._0));
    println!("Shipping Cost: {}", u256_to_eth(shipping_cost_return._0));
    println!("Coordinator PubKey: 0x{}", hex::encode(pubkey_return._0));

    Ok(())
}


