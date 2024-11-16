const { ethers } = require("ethers");

// Define the ABI for the L1Scroller contract
const abi = [
    "function readUint(address l1_contract, uint256 slot) external view returns (uint256)",
    "function readUint160(address l1_contract, uint256 slot) external view returns (uint160)",
    "function readAddress(address l1_contract, uint256 slot) external view returns (address)",
    "function readUint96(address l1_contract, uint256 slot) external view returns (uint96)",
    "function readUint48(address l1_contract, uint256 slot) external view returns (uint48)",
    "function readUint24(address l1_contract, uint256 slot) external view returns (uint24)",
    "function readUint8(address l1_contract, uint256 slot) external view returns (uint8)",
    "function readUint32(address l1_contract, uint256 slot) external view returns (uint32)",
    "function readUint64(address l1_contract, uint256 slot) external view returns (uint64)",
    "function readUint128(address l1_contract, uint256 slot) external view returns (uint128)",
    "function readString(address l1_contract, uint256 slot) external view returns (string)"
];

// Define the contract address
const CONTRACT_ADDRESS = "0xfA75fa50f36bb87669d0D4B8382BeC1C1C9570eC";

/**
 * Initializes the L1Scroller SDK with a given provider.
 * @param {string} rpcUrl - The RPC URL to connect to.
 * @returns {Object} - SDK object with functions to call the contract.
 */
function createL1ScrollerSdk(rpcUrl) {
    const provider = new ethers.JsonRpcProvider(rpcUrl);
    const contract = new ethers.Contract(CONTRACT_ADDRESS, abi, provider);

    // Define SDK functions

    async function readUint(l1Contract, slot) {
        return await contract.readUint(l1Contract, slot);
    }

    async function readUint160(l1Contract, slot) {
        return await contract.readUint160(l1Contract, slot);
    }

    async function readAddress(l1Contract, slot) {
        return await contract.readAddress(l1Contract, slot);
    }

    async function readUint96(l1Contract, slot) {
        return await contract.readUint96(l1Contract, slot);
    }

    async function readUint48(l1Contract, slot) {
        return await contract.readUint48(l1Contract, slot);
    }

    async function readUint24(l1Contract, slot) {
        return await contract.readUint24(l1Contract, slot);
    }

    async function readUint8(l1Contract, slot) {
        return await contract.readUint8(l1Contract, slot);
    }

    async function readUint32(l1Contract, slot) {
        return await contract.readUint32(l1Contract, slot);
    }

    async function readUint64(l1Contract, slot) {
        return await contract.readUint64(l1Contract, slot);
    }

    async function readUint128(l1Contract, slot) {
        return await contract.readUint128(l1Contract, slot);
    }

    async function readString(l1Contract, slot) {
        return await contract.readString(l1Contract, slot);
    }

    // Export all functions as part of the SDK
    return {
        readUint,
        readUint160,
        readAddress,
        readUint96,
        readUint48,
        readUint24,
        readUint8,
        readUint32,
        readUint64,
        readUint128,
        readString
    };
}

module.exports = createL1ScrollerSdk;
