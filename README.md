# L1Scroller

A tooling support for Scroll's new L1SLOAD feature. 
1) [L1Scroller](https://github.com/supernovahs/L1Scroller/blob/master/src/L1Scroller.sol) - Contract deployed on the scroll devnet with functions like `readSlot`, `readMultipleSlots`, `readUint`, `readUint160`,`readAddress`,`readUint96`,`readUint48`,`readUint24`, `readUint8`,`readUint32`,`readUint64`,`readUint128`,`bytesToUint`,`readString`.

2) [NFTL1Scroller](https://github.com/supernovahs/L1Scroller/blob/master/src/utils/NFTL1Scroller.sol) - Openzeppelin compatible library for ERC721, containing helper functions to get `balances_slot`, `owners_slot`,`token_approvals_slot`.

3) [ERC20Scroller](https://github.com/supernovahs/L1Scroller/blob/master/src/utils/ERC20L1Scroller.sol) -Openzeppelin compatible library for ERC20, containing helper functions to get `balances_slot`,`allowances_slot`,`total_supply_slot`,`name_slot`, `symbol_slot`.

4) Rust [SDK](https://github.com/supernovahs/L1Scroller/tree/master/scroller-rs) to use the functions offchain. 


## Feedback

1) eth_call is not working in L1SLOAD devnet due to unknown reasons (It works on Scroll Sepolia, Ethereum Sepolia). Hence testing for the Rust SDK are not available yet.


## Deployments








## Made with ❤️ by supernovahs
- [github](https://github.com/supernovahs)
- [x](https://x.com/supernovahs444)




