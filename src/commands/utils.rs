pub fn get_default_contract_address(chain_id: u64) -> String {
    match chain_id {
        17000 => "0x943285f1a29281e59514fF35Dc16E5a14E123a27".to_string(),
        _ => panic!("Unsupported chain ID: {}", chain_id),
    }
}

pub fn get_default_url(chain_id: u64) -> String {
    match chain_id {
        17000 => "https://0000000000.org".to_string(),
        _ => panic!("Unsupported chain ID: {}", chain_id),
    }
}

pub fn get_default_pubkey(chain_id: u64) -> String {
    match chain_id {
        17000 => "0x01000100e1dcd2c6b4eaefaa8360bfdab9810686e33f444818561c9f7efeb39cd577015bf9f98c56b5785315d8193f276f638d2f575842a62847e965fcc2e05c6b7242f21363158af9dbabb5642f40c121c0a9adabc1ea935f0ef12f68b794d717b78ebf1d3e4e4f5185fc8cb9fcc7b8e39c7e276ed6ac37e934a0b2bda1aa7193ce55d849deb94a4545d8d228d66a1dba96d99c236148bcc6affb2c898e869b90cbcf62b8397a93f3345f65dcbe83ae03c9ecc5aa934fc997898547136963266bb1eb29eaf62611c608b428bd72ad15614b561fdf3d16cccc1e8ee1e3b917529ee22acc8ff2f83c0114c809302381186ea6fc138060fba4bc03265b64d854bd9f536221".to_string(),
        _ => panic!("Unsupported chain ID: {}", chain_id),
    }
}

pub fn get_default_rpc(chain_id: u64) -> String {
    match chain_id {
        17000 => "https://ethereum-holesky.publicnode.com".to_string(),
        _ => panic!("Unsupported chain ID: {}", chain_id),
    }
}

pub fn get_default_tickets_number(chain_id: u64) -> usize {
    match chain_id {
        17000 => 10usize,
        _ => panic!("Unsupported chain ID: {}", chain_id),
    }
}