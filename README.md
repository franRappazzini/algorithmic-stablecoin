# Solana Algorithmic Stablecoin

This repository contains a Solana program for creating a stablecoin using the Anchor framework. The stablecoin is designed to maintain a stable value against a specific asset or currency.

## Features

- **Anchor Framework**: The stablecoin program is built using the Anchor framework, which provides a convenient and secure way to develop Solana programs.

## Smart Contract

The smart contract for the stablecoin is implemented in Rust and utilizes various Solana libraries and dependencies. Some of the key libraries used include:

- `anchor-lang`: A library for developing Solana programs, version 0.30.1 with the "init-if-needed" feature.
- `anchor-spl`: A library for build and managment of SPL tokens on Solana, version 0.30.1.
- `pyth-solana-receiver-sdk`: An Oracle for receiving data on Solana, version 0.6.1.
- `solana-program`: A library for Solana program development, version 1.18.17.

## Testing

To ensure the stability and reliability of the stablecoin program, comprehensive testing is performed. The testing process includes the use of various libraries and tools, such as:

- `@coral-xyz/anchor`: A library for developing Solana programs, version 0.30.1.
- `@pythnetwork/hermes-client`: A client for interacting with the Pyth Oracle on Solana, version 2.0.0.
- `@pythnetwork/pyth-solana-receiver`: A library for receiving data from the Pyth Oracle on Solana, version 0.10.1.
- `@solana/web3.js`: A library for interacting with Solana blockchain, version 1.98.2.

## Getting Started

To get started with the stablecoin program, follow these steps:

1. Clone this repository.

```
git clone https://github.com/your-repository.git
```

2. Install the required dependencies.

```
cd your-repository
npm install
```

3. Build the stablecoin program.

```
anchor build
```

4. Deploy the program to a Solana network.

```
anchor deploy
```

5. Test the program using the provided unit tests.

```
anchor test
```

For detailed instructions, please refer to the [Installation Guide](./docs/installation.md) and [Usage Guide](./docs/usage.md).

## Contributing

Contributions are welcome! If you would like to contribute to this project, please follow the guidelines outlined in the [Contribution Guide](./CONTRIBUTING.md).

## License

This project is licensed under the [MIT License](./LICENSE).
