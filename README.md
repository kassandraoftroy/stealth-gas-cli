# stealth-gas-cli

a command line tool for client side operations with a Stealth Gas Station service. It wraps the [eth-stealth-gas-tickets](https://github.com/kassandraoftroy/eth-stealth-gas-tickets) rust library for cryptographic operations.

## Installation

```bash
cargo install stealth-gas-cli
```

## Supported Networks

- Holesky Ethereum (chain id: 17000)

(more soon)

## Basic Usage

step 1: generate 10 unsigned tickets and store them locally

```bash
stealth-gas new
```

step 2: call buyGasTickets onchain to take your 10 locally created unsigned tickets and buy them from the coordinator

```bash
stealth-gas buy --key 0xYourPrivateKey
```

step 3: after buying gas tickets and waiting (~15 min must wait usually, but up to ~1 hour at worst) finalize your tickets

```bash
stealth-gas finalize --start-block 3213163
```

step 4: user can now send a SpendRequest to the coordinator server and redeem the 10 signed tickets (or any number of tickets depending on how many SignedTickets are in the input JSON file of finalized tickets)

```bash
stealth-gas redeem --spends '[{"amount": "9900000000000000", "receiver": "0xYourAnonAddress"}]'
```

here we redeem 10 signed tickets worth 0.01 ETH in total. We send 0.0099 ETH to 0xYourAnonAddress. (Since there is leftover the coordinator will take it and transfer herself 0.0001 ETH assuming it's a tip)

Since 0xYourAnonAddress is anonymous, then redeemer retains privacy because no one knows which ticket was redeemed (not even the coordinator).

## Command options

see `stealth-gas help` and `stealth-gas <command> --help` for more details on each command.

Here are some example commands with more optional arguments passed:

```
stealth-gas new --chain-id 17000 --num 10 --output ~/new_10_tickets_holesky.json
```

you can also manually pass --pubkey 0xCoordinatorPubKey (but i don't recommend this unless you know exactly what/why you are doing it)

```
stealth-gas buy --chain-id 17000 --rpc-url https://youralchemyapiurl.io/key --account path/to/my/keystore/file --input ~/new_10_tickets_holesky.json
```

either pass --account or --key (for raw private key, not super safe to input on cli if key could be of high value, but fine on a purely test account)

you can also pass --gas-station-address 0xContractAddress if you want to manually pass the StealthGasStation contract address (again not recommended unless you know why tou are doing this)

```
stealth-gas finalize --chain-id 17000 --start-block 3213163 --rpc-url https://youralchemyapiurl.io/key --input ~/new_10_tickets_holesky.json --output ~/finalized_10_tickets_holesky.json
```

you can also pass custom --gas-station-address 0xContractAddress and or custom --pubkey 0xCoordinatorPubKey if desired

```
stealth-gas redeem --chain-id 17000 --url https://0000000000.org --input ~/finalized_10_tickets_holesky.json --spends '[{"amount": "9000000000000000", "receiver": "0xYourAnonAddress1"}, {"amount": "1000000000000000", "receiver": "0xYourAnonAddress2"}]'
```
