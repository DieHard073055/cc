# Cryptocurrency Unit Converter CLI

A command-line tool for converting between different cryptocurrency units (Bitcoin, Ethereum, and Solana).

## Overview

This Rust-based CLI application provides a simple way to convert between various cryptocurrency units. Whether you need to convert BTC to Satoshis, ETH to Gwei, or SOL to Lamports, this tool makes it quick and easy.

## Features

- Support for Bitcoin, Ethereum, and Solana cryptocurrencies
- Conversion between multiple denomination units
- Simple command-line interface
- Precise conversion with proper decimal handling

## Installation

### Prerequisites

- Rust and Cargo (Install from [rustup.rs](https://rustup.rs))

### Building from Source


```bash


# Clone the repository
git clone https://github.com/DieHard073055/cc.git
cd cc

# Build the project
cargo build --release

# The binary will be available at ./target/release/cc


```



## Usage


```bash


cc <AMOUNT> <FROM_UNIT> to <TO_UNIT>


```



For help:


```bash


cc help


```



## Supported Units

### Bitcoin (BTC)
- btc - Bitcoin
- mbtc - Milli-Bitcoin (1/1,000 BTC)
- bit - Micro-Bitcoin (1/1,000,000 BTC)
- sats or satoshi - Satoshi (1/100,000,000 BTC)

### Ethereum (ETH)
- eth - Ether
- gwei - Giga-wei (1/1,000,000,000 ETH)
- wei - Wei (1/1,000,000,000,000,000,000 ETH)
- fin - Fin

### Solana (SOL)
- sol - SOL
- lam or lamports - Lamports (1/1,000,000,000 SOL)

## Examples

Convert 1 Bitcoin to Satoshis:


```bash


cc 1 btc to sats
# Output: 1 btc = 100000000 sats


```



Convert 5000 Gwei to ETH:


```bash


cc 5000 gwei to eth
# Output: 5000 gwei = 0.000005 eth


```



Convert 0.5 SOL to Lamports:


```bash


cc 0.5 sol to lam
# Output: 0.5 sol = 500000000 lam


```



## Error Handling

The converter will display helpful error messages for:
- Invalid units
- Invalid amount formats
- Incompatible unit conversions
- Missing arguments

## Project Structure


```


cc/
├── src/
│   ├── main.rs        # Application entry point
│   ├── lib.rs         # Module exports
│   ├── cli.rs         # Command-line interface logic
│   ├── conversion.rs  # Conversion algorithms
│   └── currency.rs    # Currency definitions and unit handling
├── Cargo.toml         # Project dependencies and metadata
└── README.md          # This file


```



## Dependencies

- thiserror - For error handling

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (git checkout -b feature/amazing-feature)
3. Commit your changes (git commit -m 'Add some amazing feature')
4. Push to the branch (git push origin feature/amazing-feature)
5. Open a Pull Request
