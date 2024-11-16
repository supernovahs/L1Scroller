// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/// @title NFTL1Scroller - Utility library for reading  OpenZeppelin-compatible ERC721 storage slots on Scroll.
/// @notice This library provides functions to compute storage slots for balance, owner, and token approvals
/// based on the OpenZeppelin ERC721 storage layout. It is designed to be used with the L1SLOAD precompiled contract.
/// https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC721/ERC721.sol
library NFTL1Scroller {
    /// @dev Slot index for the `name` of the ERC721 token (OpenZeppelin storage layout).
    uint256 constant NAME_SLOT_OPENZEPPELIN = 0;

    /// @dev Slot index for the `symbol` of the ERC721 token (OpenZeppelin storage layout).
    uint256 constant SYMBOL_SLOT_OPENZEPPELIN = 1;

    /// @dev Slot index for mapping of `tokenId` to owner address (OpenZeppelin storage layout).
    uint256 constant ID_TO_OWNER_SLOT_OPENZEPPELIN = 2;

    /// @dev Slot index for mapping of `owner` address to balance (OpenZeppelin storage layout).
    uint256 constant BALANCE_OF_OWNER_SLOT_OPENZEPPELIN = 3;

    /// @dev Slot index for mapping of `tokenId` to approved address (OpenZeppelin storage layout).
    uint256 constant TOKEN_APPROVALS = 4;

    /// @notice Computes the storage slot for the balance of a specific owner.
    /// @param account The address of the ERC721 token owner.
    /// @return The storage slot for the balance of the given owner address.
    /// @dev The slot is computed as `keccak256(account + BALANCE_OF_OWNER_SLOT_OPENZEPPELIN)`.
    function balances_slot(address account) public pure returns (uint256) {
        return uint256(keccak256(abi.encodePacked(uint256(uint160(account)), BALANCE_OF_OWNER_SLOT_OPENZEPPELIN)));
    }

    /// @notice Computes the storage slot for the owner of a specific `tokenId`.
    /// @param tokenId The ID of the ERC721 token.
    /// @return The storage slot for the owner of the given `tokenId`.
    /// @dev The slot is computed as `keccak256(tokenId + ID_TO_OWNER_SLOT_OPENZEPPELIN)`.
    function owners_slot(uint256 tokenId) public pure returns (uint256) {
        return uint256(keccak256(abi.encodePacked(tokenId, ID_TO_OWNER_SLOT_OPENZEPPELIN)));
    }

    /// @notice Computes the storage slot for the approved address of a specific `tokenId`.
    /// @param tokenId The ID of the ERC721 token.
    /// @return The storage slot for the approved address of the given `tokenId`.
    /// @dev The slot is computed as `keccak256(tokenId + TOKEN_APPROVALS)`.
    function token_approvals_slot(uint256 tokenId) public pure returns (uint256) {
        return uint256(keccak256(abi.encodePacked(tokenId, TOKEN_APPROVALS)));
    }
}
