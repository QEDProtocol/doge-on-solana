<a name="readme-top"></a>
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]




<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/QEDProtocol/doge-on-solana">
  <img height="256" src="https://github.com/QEDProtocol/doge-on-solana/raw/main/static/doge-sol-ibc.png?raw=true">
  </a>

  <h3 align="center">Doge on Solana</h3>

  <p align="center">
A Solana program that enables trustless inter-blockchain communication between Dogecoin/Bitcoin and Solana by verifying full PoW/AuxPow consensus on chain.
  </p>
  <p align="center">Made with ❤️ by <a href="https://x.com/QEDProtocol" target="_blank">QED</a></p>
</div>

## Features
* Full verification of Dogecoin and Bitcoin consensus in a Solana program
* Supports AuxPow/Non-AuxPow blocks for Mainnet and Testnet
* Maintains a constant-memory merkle tree of all blocks processed + 32 latest block cache in memory for convienence 
* Supports re-orgs of up to 4 blocks
* Trustlessly read any Dogecoin transaction on Solana using a Merkle Proof
* Trustlessly read any Bitcoin transaction on Solana using a Merkle Proof
* Entire state is zerocopy, so other applications can read from the contract with no overhead by reading in the account
  * ```rust
    QEDDogeChainState::ref_from_bytes(&account.data[33..]).?.get_finalized_block_hash()
    ```
* Uses zero knowledge proofs for proving scrypt_1024_1_1_256
* very trustless, much ibc



## Usage
```bash
git clone https://github.com/QEDProtocol/doge-on-solana
cd doge-on-solana
pnpm install
pnpm build:programs

# this is our buffer writer for getting around the 1232 byte tx limit
cp ./external-programs/CzqeK66uHUYbauvaLJ3sfQd9JmiMqvvPvAudpZmhr6xF.so ./target/deploy/

# sets up a solana ibc program account and appends sends two doge blocks to it
pnpm clients:js:test
```

## How it works
Coming Soon~


## License
Copyright (C) 2025 Zero Knowledge Labs Limited, QED Protocol

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.

Additional terms under GNU AGPL version 3 section 7:

As permitted by section 7(b) of the GNU Affero General Public License, 
you must retain the following attribution notice in all copies or 
substantial portions of the software:

"This software was created by QED (https://qedprotocol.com)
with contributions from Carter Feldman (https://x.com/cmpeq)."



[contributors-shield]: https://img.shields.io/github/contributors/QEDProtocol/doge-on-solana.svg?style=for-the-badge
[contributors-url]: https://github.com/QEDProtocol/doge-on-solana/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/QEDProtocol/doge-on-solana.svg?style=for-the-badge
[forks-url]: https://github.com/QEDProtocol/doge-on-solana/network/members
[stars-shield]: https://img.shields.io/github/stars/QEDProtocol/doge-on-solana.svg?style=for-the-badge
[stars-url]: https://github.com/QEDProtocol/doge-on-solana/stargazers
[issues-shield]: https://img.shields.io/github/issues/QEDProtocol/doge-on-solana.svg?style=for-the-badge
[issues-url]: https://github.com/QEDProtocol/doge-on-solana/issues

