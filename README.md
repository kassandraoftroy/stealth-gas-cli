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

step 1: generate 10 unsigned tickets

```bash
stealth-gas new -n 10
```

step 2: call buyGasTickets onchain to take your 10 locally created unsigned tickets and buy them from the coordinator

```bash
stealth-gas buy -i ~/.stealthereum/unsigned_tickets_17000.json -k 0xYourPrivateKey
```

step 3: after buying gas tickets and waiting (~15 min must wait usually, but up to ~1 hour in worse conditions) finalize your tickets after cooridinator returned signed tickets (by calling sendGasTickets onchain)

```bash
stealth-gas finalize --start-block 3048901 -i ~/.stealthereum/unsigned_tickets_17000.json
```

step 4: user can now send a SpendRequest to the coordinator server and redeem the 10 signed tickets (or any number of tickets depending on how many SignedTickets are in the input JSON file of finalized tickets)

```bash
stealth-gas redeem -s '[{"amount": "9900000000000000", "receiver": "0xYourAnonAddress"}]' -i ~/.stealthereum/finalized_tickets_17000.json
```

here we redeem 10 signed tickets worth 0.01 ETH in total. We send 0.0099 ETH to 0xYourAnonAddress. (Since there is leftover the coordinator will take it and transfer herself 0.0001 ETH)

Since 0xYourAnonAddress is anonymous, then redeemer retains privacy because no one knows which ticket was redeemed (not even the coordinator).
