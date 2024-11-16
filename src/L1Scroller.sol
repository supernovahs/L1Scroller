// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/// @title L1Scroller - Utility contract to read storage slots from L1 contracts.
/// @notice This contract provides functions to read various integer types and strings
/// from storage slots of another contract on the L1 layer using a precompiled contract.
contract L1Scroller {
    /// @dev Address of the precompiled contract for reading storage slots.
    address constant L1_SLOAD = 0x0000000000000000000000000000000000000101;

    address constant L1_BLOCKS = 0x5300000000000000000000000000000000000001;

    /// @notice Reads the raw bytes from a specified storage slot of an L1 contract.
    /// @param l1_contract The address of the L1 contract.
    /// @param slot The storage slot index to read from.
    /// @return result The raw bytes stored at the specified slot.
    function readSlot(address l1_contract, uint256 slot) public view returns (bytes memory) {
        bytes memory input = abi.encodePacked(l1_contract, slot);
        (bool success, bytes memory result) = L1_SLOAD.staticcall(input);
        require(success, "Failed to read slot");
        return result;
    }

    /// @notice Reads the raw bytes from multiple slots of an L1 contract.
    /// @param l1_contract The address of the L1 contract.
    /// @param slot The storage slotsto read from.
    /// @return result The raw bytes
    function readMultipleSlots(address l1_contract, uint256[] memory slot) public view returns (bytes memory) {
        bytes memory input = abi.encodePacked(l1_contract, slot);
        (bool success, bytes memory result) = L1_SLOAD.staticcall(input);
        require(success, "Failed to read slot");
        return result;
    }

    /// @notice Reads a `uint256` value from a specified storage slot.
    /// @param l1_contract The address of the L1 contract.
    /// @param slot The storage slot index to read from.
    /// @return The `uint256` value stored at the specified slot.
    function readUint(address l1_contract, uint256 slot) public view returns (uint256) {
        bytes memory result = readSlot(l1_contract, slot);
        return bytesToUint(result);
    }

    /// @notice Reads a `uint160` value from a specified storage slot.
    function readUint160(address l1_contract, uint256 slot) public view returns (uint160) {
        bytes memory result = readSlot(l1_contract, slot);
        return uint160(bytesToUint(result));
    }

    /// @notice Reads a `address` value from a specified storage slot.
    function readAddress(address l1_contract, uint256 slot) public view returns (address) {
        bytes memory result = readSlot(l1_contract, slot);
        return address(uint160(bytesToUint(result)));
    }

    /// @notice Reads a `uint96` value from a specified storage slot.
    function readUint96(address l1_contract, uint256 slot) public view returns (uint96) {
        bytes memory result = readSlot(l1_contract, slot);
        return uint96(bytesToUint(result));
    }

    /// @notice Reads a `uint48` value from a specified storage slot.
    function readUint48(address l1_contract, uint256 slot) public view returns (uint48) {
        bytes memory result = readSlot(l1_contract, slot);
        return uint48(bytesToUint(result));
    }

    /// @notice Reads a `uint24` value from a specified storage slot.
    function readUint24(address l1_contract, uint256 slot) public view returns (uint24) {
        bytes memory result = readSlot(l1_contract, slot);
        return uint24(bytesToUint(result));
    }

    /// @notice Reads a `uint8` value from a specified storage slot.
    function readUint8(address l1_contract, uint256 slot) public view returns (uint8) {
        bytes memory result = readSlot(l1_contract, slot);
        return uint8(bytesToUint(result));
    }

    /// @notice Reads a `uint32` value from a specified storage slot.
    function readUint32(address l1_contract, uint256 slot) public view returns (uint32) {
        bytes memory result = readSlot(l1_contract, slot);
        return uint32(bytesToUint(result));
    }

    /// @notice Reads a `uint64` value from a specified storage slot.
    function readUint64(address l1_contract, uint256 slot) public view returns (uint64) {
        bytes memory result = readSlot(l1_contract, slot);
        return uint64(bytesToUint(result));
    }

    /// @notice Reads a `uint128` value from a specified storage slot.
    function readUint128(address l1_contract, uint256 slot) public view returns (uint128) {
        bytes memory result = readSlot(l1_contract, slot);
        return uint128(bytesToUint(result));
    }

    /// @notice Converts a `bytes` array to a `uint256` value using ABI decoding.
    /// @param b The `bytes` array to convert.
    /// @return The decoded `uint256` value.
    function bytesToUint(bytes memory b) public pure returns (uint256) {
        return abi.decode(b, (uint256));
    }

    /// @notice Reads a `string` value from a specified storage slot.
    /// @param l1_contract The address of the L1 contract.
    /// @param slot The storage slot index to read from.
    /// @return The decoded `string` value.
    function readString(address l1_contract, uint256 slot) public view returns (string memory) {
        bytes memory result = readSlot(l1_contract, slot);
        return string(result);
    }
}
