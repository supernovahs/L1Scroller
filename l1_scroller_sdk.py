from web3 import Web3

# ABI for the L1Scroller contract
ABI = [
    {
        "inputs": [
            {"internalType": "address", "name": "l1_contract", "type": "address"},
            {"internalType": "uint256", "name": "slot", "type": "uint256"}
        ],
        "name": "readUint",
        "outputs": [{"internalType": "uint256", "name": "", "type": "uint256"}],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [
            {"internalType": "address", "name": "l1_contract", "type": "address"},
            {"internalType": "uint256", "name": "slot", "type": "uint256"}
        ],
        "name": "readUint160",
        "outputs": [{"internalType": "uint160", "name": "", "type": "uint160"}],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [
            {"internalType": "address", "name": "l1_contract", "type": "address"},
            {"internalType": "uint256", "name": "slot", "type": "uint256"}
        ],
        "name": "readAddress",
        "outputs": [{"internalType": "address", "name": "", "type": "address"}],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [
            {"internalType": "address", "name": "l1_contract", "type": "address"},
            {"internalType": "uint256", "name": "slot", "type": "uint256"}
        ],
        "name": "readString",
        "outputs": [{"internalType": "string", "name": "", "type": "string"}],
        "stateMutability": "view",
        "type": "function"
    }
]

# Contract address
CONTRACT_ADDRESS = "0xfA75fa50f36bb87669d0D4B8382BeC1C1C9570eC"

class L1ScrollerSdk:
    def __init__(self, rpc_url):
        self.web3 = Web3(Web3.HTTPProvider(rpc_url))
        if not self.web3.is_connected():
            raise ConnectionError("Failed to connect to the RPC URL.")
        self.contract = self.web3.eth.contract(address=CONTRACT_ADDRESS, abi=ABI)

    def read_uint(self, l1_contract, slot):
        return self.contract.functions.readUint(l1_contract, slot).call()

    def read_uint160(self, l1_contract, slot):
        return self.contract.functions.readUint160(l1_contract, slot).call()

    def read_address(self, l1_contract, slot):
        return self.contract.functions.readAddress(l1_contract, slot).call()

    def read_string(self, l1_contract, slot):
        result = self.contract.functions.readString(l1_contract, slot).call()
        # Remove null characters if present
        return result.replace('\x00', '')

# Example usage
if __name__ == "__main__":
    rpc_url = "https://l1sload-rpc.scroll.io"
    l1_contract_address = "0xA8E50c2607678747D9d8A24AC52234712bE41fD9"
    uint_slot = 0
    address_slot = 1
    string_slot = 2

    sdk = L1ScrollerSdk(rpc_url)

    try:
        uint_value = sdk.read_uint(l1_contract_address, uint_slot)
        print("Uint Value:", uint_value)

        address_value = sdk.read_address(l1_contract_address, address_slot)
        print("Address Value:", address_value)

        string_value = sdk.read_string(l1_contract_address, string_slot)
        print("String Value:", string_value)

    except Exception as e:
        print("Error:", e)
