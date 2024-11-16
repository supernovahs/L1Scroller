# L1Scroller

A tooling support for Scroll's new **L1SLOAD** feature, enabling efficient off-chain and on-chain storage slot reading from L1 contracts. The repository includes:

1. **[L1Scroller](https://github.com/supernovahs/L1Scroller/blob/master/src/L1Scroller.sol)** - A smart contract deployed on the Scroll devnet with various read functions.
2. **[NFTL1Scroller](https://github.com/supernovahs/L1Scroller/blob/master/src/utils/NFTL1Scroller.sol)** - OpenZeppelin-compatible library for ERC721, providing helper functions for reading balance, owner, and token approval slots.
3. **[ERC20L1Scroller](https://github.com/supernovahs/L1Scroller/blob/master/src/utils/ERC20L1Scroller.sol)** - OpenZeppelin-compatible library for ERC20, offering functions for reading balance, allowance, total supply, name, and symbol slots.
4. **[Rust SDK](https://github.com/supernovahs/L1Scroller/tree/master/scroller-rs)** - A Rust SDK for using the functions off-chain, enabling seamless integration with Rust-based applications.

## L1Scroller Contract

The `L1Scroller` contract provides various functions to read different types of data from storage slots on L1 contracts.

### Functions

- `readSlot(address l1Contract, uint256 slot)` - Reads raw bytes from a specified storage slot.
- `readMultipleSlots(address l1Contract, uint256[] slots)` - Reads multiple storage slots and returns raw bytes data.
- `readUint(address l1Contract, uint256 slot)` - Reads a `uint256` value from the specified slot.
- `readUint160(address l1Contract, uint256 slot)` - Reads a `uint160` value from the specified slot.
- `readUint96(address l1Contract, uint256 slot)` - Reads a `uint96` value from the specified slot.
- `readUint48(address l1Contract, uint256 slot)` - Reads a `uint48` value from the specified slot.
- `readUint24(address l1Contract, uint256 slot)` - Reads a `uint24` value from the specified slot.
- `readUint8(address l1Contract, uint256 slot)` - Reads a `uint8` value from the specified slot.
- `readUint32(address l1Contract, uint256 slot)` - Reads a `uint32` value from the specified slot.
- `readUint64(address l1Contract, uint256 slot)` - Reads a `uint64` value from the specified slot.
- `readUint128(address l1Contract, uint256 slot)` - Reads a `uint128` value from the specified slot.
- `readAddress(address l1Contract, uint256 slot)` - Reads an `address` value from the specified slot.
- `bytesToUint(bytes data)` - Converts raw bytes to a `uint256` value.
- `readString(address l1Contract, uint256 slot)` - Reads a `string` value from the specified slot.

## NFTL1Scroller Library

The `NFTL1Scroller` library provides helper functions for reading storage slots related to ERC721 tokens, compatible with OpenZeppelin's ERC721 implementation.

### Functions

- `balances_slot(address account)` - Computes the storage slot for the balance of a specific owner.
- `owners_slot(uint256 tokenId)` - Computes the storage slot for the owner of a specific `tokenId`.
- `token_approvals_slot(uint256 tokenId)` - Computes the storage slot for the approved address of a specific `tokenId`.

## ERC20L1Scroller Library

The `ERC20L1Scroller` library offers helper functions for reading storage slots related to ERC20 tokens, compatible with OpenZeppelin's ERC20 implementation.

### Functions

- `balances_slot(address account)` - Computes the storage slot for the balance of a specific account.
- `allowances_slot(address account, address spender)` - Computes the storage slot for the allowance of a specific spender for a given account.
- `total_supply_slot()` - Returns the storage slot for the total supply of the ERC20 token.
- `name_slot()` - Returns the storage slot for the name of the ERC20 token.
- `symbol_slot()` - Returns the storage slot for the symbol of the ERC20 token.

## Rust SDK

The [Rust SDK](https://github.com/supernovahs/L1Scroller/tree/master/scroller-rs) provides off-chain access to the L1Scroller functions, allowing developers to easily integrate storage slot reading capabilities in Rust applications.

### Features

- **Reading Storage Slots**: Supports `readSlot`, `readMultipleSlots`, and other read functions off-chain.

## Getting Started

Clone the repository and explore the smart contracts and the Rust SDK:

```bash
git clone https://github.com/supernovahs/L1Scroller.git
```

## FEEDBACK

- eth_call is not working in L1SLOAD devnet due to unknown reasons (It works on Scroll Sepolia, Ethereum Sepolia). Hence testing for the Rust SDK are not available yet.

- Gas estimation often fails on forge `create` as well as on `remix`. It worked with following command
 ```bash
 forge create ./src/L1Scroller.sol:L1Scroller --rpc-url https://l1sload-rpc.scroll.io --private-key <KEY>  --gas-limit 1000000 --gas-price 551000000  --priority-gas-price 551000000
```

## Deployments

### Blockscout Link
L1Scroller - https://l1sload-blockscout.scroll.io/address/0xfA75fa50f36bb87669d0D4B8382BeC1C1C9570eC

### Contract Verification on Blockscout 
Seems the devnet is not suppported on blockscout. Attached screenshot:-
<img width="866" alt="Screenshot 2024-11-17 at 1 16 17 AM" src="https://github.com/user-attachments/assets/a77c7272-9a4a-4951-a071-15311792a0ac">

## Testing

- Run foundry tests 
```bash
forge test
```

- Run rust SDK tests 
```bash
cd scroller-rs
cargo test
```

## Made with ❤️ by supernovahs
- [github](https://github.com/supernovahs)
- [x](https://x.com/supernovahs444)







