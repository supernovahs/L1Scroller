const { assert } = require("ethers");
const createL1ScrollerSdk = require("./l1ScrollerSdk");

// Initialize the SDK with your RPC URL
const sdk = createL1ScrollerSdk("https://l1sload-rpc.scroll.io");

async function testSdk() {
    try {
        const l1Contract = "0xA8E50c2607678747D9d8A24AC52234712bE41fD9";
        const uint_slot = 0;
        const address_slot = 1;
        const string_slot = 2;

        // Example calls
        const uintValue = await sdk.readUint(l1Contract, uint_slot);
        assert(uintValue ==2);
        console.log("Uint Value:", uintValue.toString());

        const addressValue = await sdk.readAddress(l1Contract, address_slot);
        assert(addressValue == "0x4838B106FCe9647Bdf1E7877BF73cE8B0BAD5f97");
        console.log("Address Value:", addressValue);

        const stringValue = await sdk.readString(l1Contract, string_slot);
        const cleanedString = stringValue.toString().replace(/\u0000/g, "");
        assert(cleanedString == "scroll");
    } catch (error) {
        console.error("Error:", error);
    }
}

testSdk();
