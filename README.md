# stealth-gas-cli

a command line tool for client side cryptographic operations for a Stealth Gas Station service that wraps the [eth-stealth-gas-tickets](https://github.com/kassandraoftroy/eth-stealth-gas-tickets) rust library.

## Installation

```bash
cargo install stealth-gas-cli
```

## Supported Networks

- Holesky Ethereum (chain id: 17000)

(more soon)

## Usage

step 1: fetch the coordinator pubkey and other public parameters (this is just for convenience you can fetch this onchain yourself as well)

```bash
stealth-gas params --rpc-url https://ethereum-holesky-rpc.publicnode.com
```

step 2: generate 10 unsigned tickets

```bash
stealth-gas new --key 0xCoordinatorPubKey --num 10 --output unsigned_tickets.json
```

step 3: call buyGasTickets with your 10 unsigned tickets

```bash
stealth-gas buy --rpc-url https://ethereum-holesky-rpc.publicnode.com --contract-address 0xGasStationAddress --input unsigned_tickets.json --private-key 0xPrivateKey
```

step 4: after buying gas tickets (wait for finalization), scan the chain for blind signatures that match your unsigned tickets.

```bash
stealth-gas scan --rpc-url https://ethereum-holesky-rpc.publicnode.com --contract-address 0xGasStationAddress --input unsigned_tickets.json --start-block 1000000 --output finalizeable.json
```

step 5: after finding your 10 signed (blind) tickets in the scan, finalize the blind signatures to generate redeemable gas tickets

```bash
stealth-gas finalize -r https://ethereum-holesky-rpc.publicnode.com -i finalizeable.json -o signed_tickets.json
```

step 6: user can now send a SpendRequest to the coordinator server and redeem the 10 signed tickets (or any number of tickets depending on how many SignedTickets are in the input JSON file)

```bash
stealth-gas redeem -u https://0000000000.org -i ~/Desktop/st100.json -s '[{"amount": "9900000000000000", "receiver": "0xYourAnonAddress"}]'
```

here we redeem 10 signed tickets worth 0.01 ETH in total. We send 0.0099 ETH to 0xYourAnonAddress. (Since there is leftover the coordinator will take it and transfer herself 0.0001 ETH)

Since 0xYourAnonAddress is anonymous, then redeemer retains privacy because no one knows which ticket was redeemed (not even the coordinator).
