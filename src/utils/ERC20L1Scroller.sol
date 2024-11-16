// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/// @title ERC20L1Scroller - Utility library for reading OpenZeppelin-compatible ERC20 storage slots on Scroll
/// @notice This library provides functions to compute storage slots for balances and allowances
/// based on the OpenZeppelin ERC20 storage layout. It is designed to be used with the L1SLOAD precompiled contract.
/// https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/ERC20.sol
library ERC20L1Scroller {
    /// @dev Slot index for mapping of `account` to balance (OpenZeppelin storage layout).
    uint256 constant BALANCES_OPENZEPPELIN = 0;

    /// @dev Slot index for mapping of `account` to another address's allowance (OpenZeppelin storage layout).
    uint256 constant ALLOWANCES_OPENZEPPELIN = 1;

    /// @dev Slot index for the total supply of the ERC20 token (OpenZeppelin storage layout).
    uint256 constant TOTAL_SUPPLY_OPENZEPPELIN = 2;

    /// @dev Slot index for the `name` of the ERC20 token (OpenZeppelin storage layout).
    uint256 constant NAME_SLOT_OPENZEPPELIN = 3;

    /// @dev Slot index for the `symbol` of the ERC20 token (OpenZeppelin storage layout).
    uint256 constant SYMBOL_SLOT_OPENZEPPELIN = 4;

    /// @notice Computes the storage slot for the balance of a specific account.
    /// @param account The address of the ERC20 token holder.
    /// @return The storage slot for the balance of the given account.
    /// @dev The slot is computed as `keccak256(account + BALANCES_OPENZEPPELIN)`.
    /// This is based on the OpenZeppelin storage layout for ERC20 contracts.
    function balances_slot(address account) public pure returns (uint256) {
        return uint256(keccak256(abi.encodePacked(uint256(uint160(account)), BALANCES_OPENZEPPELIN)));
    }

    /// @notice Computes the storage slot for the allowance of a specific spender for a given account.
    /// @param account The address of the ERC20 token holder (the owner).
    /// @param spender The address of the spender who has been approved.
    /// @return The storage slot for the allowance of the `spender` for the given `account`.
    /// @dev The slot is computed as `keccak256(spender + keccak256(account + ALLOWANCES_OPENZEPPELIN))`.
    /// This follows the OpenZeppelin storage layout for ERC20 allowances.
    function allowances_slot(address account, address spender) public pure returns (uint256) {
        return
            uint256(keccak256(abi.encodePacked(spender, keccak256(abi.encodePacked(account, ALLOWANCES_OPENZEPPELIN)))));
    }

    /// @notice Computes the storage slot for the total supply of the ERC20 token.
    /// @return The storage slot for the total supply of the token.
    /// @dev The total supply is stored directly at the slot index `TOTAL_SUPPLY_OPENZEPPELIN`.
    function total_supply_slot() public pure returns (uint256) {
        return TOTAL_SUPPLY_OPENZEPPELIN;
    }

    /// @notice Computes the storage slot for the `name` of the ERC20 token.
    /// @return The storage slot for the `name` of the token.
    /// @dev The `name` is stored directly at the slot index `NAME_SLOT_OPENZEPPELIN`.
    function name_slot() public pure returns (uint256) {
        return NAME_SLOT_OPENZEPPELIN;
    }

    /// @notice Computes the storage slot for the `symbol` of the ERC20 token.
    /// @return The storage slot for the `symbol` of the token.
    /// @dev The `symbol` is stored directly at the slot index `SYMBOL_SLOT_OPENZEPPELIN`.
    function symbol_slot() public pure returns (uint256) {
        return SYMBOL_SLOT_OPENZEPPELIN;
    }
}
