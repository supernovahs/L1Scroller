/**

Generated by the following Solidity interface...
```solidity
interface NFTL1Scroller {
    function balances_slot(address account) external pure returns (uint256);
    function owners_slot(uint256 tokenId) external pure returns (uint256);
    function token_approvals_slot(uint256 tokenId) external pure returns (uint256);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "balances_slot",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "owners_slot",
    "inputs": [
      {
        "name": "tokenId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "token_approvals_slot",
    "inputs": [
      {
        "name": "tokenId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "pure"
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod NFTL1Scroller {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x61019e610039600b82828239805160001a607314602c57634e487b7160e01b600052600060045260246000fd5b30600052607381538281f3fe730000000000000000000000000000000000000000301460806040526004361061004b5760003560e01c8063044d0f3d1461005057806354595881146100755780636b8f081b14610088575b600080fd5b61006361005e36600461011f565b61009b565b60405190815260200160405180910390f35b61006361008336600461011f565b6100d8565b610063610096366004610138565b6100f7565b60008160026040516020016100ba929190918252602082015260400190565b60408051601f19818403018152919052805160209091012092915050565b60008160046040516020016100ba929190918252602082015260400190565b6000816001600160a01b031660036040516020016100ba929190918252602082015260400190565b60006020828403121561013157600080fd5b5035919050565b60006020828403121561014a57600080fd5b81356001600160a01b038116811461016157600080fd5b939250505056fea2646970667358221220c14b13d633358fa19d8c0e7d5d29525949ef5913bf5ce7240ac1e68cdc5bc8a564736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\x9Ea\09`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14`,WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0KW`\x005`\xE0\x1C\x80c\x04M\x0F=\x14a\0PW\x80cTYX\x81\x14a\0uW\x80ck\x8F\x08\x1B\x14a\0\x88W[`\0\x80\xFD[a\0ca\0^6`\x04a\x01\x1FV[a\0\x9BV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0ca\0\x836`\x04a\x01\x1FV[a\0\xD8V[a\0ca\0\x966`\x04a\x018V[a\0\xF7V[`\0\x81`\x02`@Q` \x01a\0\xBA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x92\x91PPV[`\0\x81`\x04`@Q` \x01a\0\xBA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16`\x03`@Q` \x01a\0\xBA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x011W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x01JW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01aW`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xC1K\x13\xD635\x8F\xA1\x9D\x8C\x0E}])RYI\xEFY\x13\xBF\\\xE7$\n\xC1\xE6\x8C\xDC[\xC8\xA5dsolcC\0\x08\x1A\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x730000000000000000000000000000000000000000301460806040526004361061004b5760003560e01c8063044d0f3d1461005057806354595881146100755780636b8f081b14610088575b600080fd5b61006361005e36600461011f565b61009b565b60405190815260200160405180910390f35b61006361008336600461011f565b6100d8565b610063610096366004610138565b6100f7565b60008160026040516020016100ba929190918252602082015260400190565b60408051601f19818403018152919052805160209091012092915050565b60008160046040516020016100ba929190918252602082015260400190565b6000816001600160a01b031660036040516020016100ba929190918252602082015260400190565b60006020828403121561013157600080fd5b5035919050565b60006020828403121561014a57600080fd5b81356001600160a01b038116811461016157600080fd5b939250505056fea2646970667358221220c14b13d633358fa19d8c0e7d5d29525949ef5913bf5ce7240ac1e68cdc5bc8a564736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0KW`\x005`\xE0\x1C\x80c\x04M\x0F=\x14a\0PW\x80cTYX\x81\x14a\0uW\x80ck\x8F\x08\x1B\x14a\0\x88W[`\0\x80\xFD[a\0ca\0^6`\x04a\x01\x1FV[a\0\x9BV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0ca\0\x836`\x04a\x01\x1FV[a\0\xD8V[a\0ca\0\x966`\x04a\x018V[a\0\xF7V[`\0\x81`\x02`@Q` \x01a\0\xBA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x92\x91PPV[`\0\x81`\x04`@Q` \x01a\0\xBA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16`\x03`@Q` \x01a\0\xBA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x011W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x01JW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01aW`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xC1K\x13\xD635\x8F\xA1\x9D\x8C\x0E}])RYI\xEFY\x13\xBF\\\xE7$\n\xC1\xE6\x8C\xDC[\xC8\xA5dsolcC\0\x08\x1A\x003",
    );
    /**Function with signature `balances_slot(address)` and selector `0x6b8f081b`.
    ```solidity
    function balances_slot(address account) external pure returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct balances_slotCall {
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`balances_slot(address)`](balances_slotCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct balances_slotReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<balances_slotCall> for UnderlyingRustTuple<'_> {
                fn from(value: balances_slotCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for balances_slotCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<balances_slotReturn> for UnderlyingRustTuple<'_> {
                fn from(value: balances_slotReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for balances_slotReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for balances_slotCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = balances_slotReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "balances_slot(address)";
            const SELECTOR: [u8; 4] = [107u8, 143u8, 8u8, 27u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `owners_slot(uint256)` and selector `0x044d0f3d`.
    ```solidity
    function owners_slot(uint256 tokenId) external pure returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct owners_slotCall {
        pub tokenId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`owners_slot(uint256)`](owners_slotCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct owners_slotReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<owners_slotCall> for UnderlyingRustTuple<'_> {
                fn from(value: owners_slotCall) -> Self {
                    (value.tokenId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for owners_slotCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { tokenId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<owners_slotReturn> for UnderlyingRustTuple<'_> {
                fn from(value: owners_slotReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for owners_slotReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for owners_slotCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = owners_slotReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owners_slot(uint256)";
            const SELECTOR: [u8; 4] = [4u8, 77u8, 15u8, 61u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.tokenId,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `token_approvals_slot(uint256)` and selector `0x54595881`.
    ```solidity
    function token_approvals_slot(uint256 tokenId) external pure returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct token_approvals_slotCall {
        pub tokenId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`token_approvals_slot(uint256)`](token_approvals_slotCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct token_approvals_slotReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<token_approvals_slotCall> for UnderlyingRustTuple<'_> {
                fn from(value: token_approvals_slotCall) -> Self {
                    (value.tokenId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for token_approvals_slotCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { tokenId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<token_approvals_slotReturn> for UnderlyingRustTuple<'_> {
                fn from(value: token_approvals_slotReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for token_approvals_slotReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for token_approvals_slotCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = token_approvals_slotReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "token_approvals_slot(uint256)";
            const SELECTOR: [u8; 4] = [84u8, 89u8, 88u8, 129u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.tokenId,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    ///Container for all the [`NFTL1Scroller`](self) function calls.
    pub enum NFTL1ScrollerCalls {
        balances_slot(balances_slotCall),
        owners_slot(owners_slotCall),
        token_approvals_slot(token_approvals_slotCall),
    }
    #[automatically_derived]
    impl NFTL1ScrollerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [4u8, 77u8, 15u8, 61u8],
            [84u8, 89u8, 88u8, 129u8],
            [107u8, 143u8, 8u8, 27u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for NFTL1ScrollerCalls {
        const NAME: &'static str = "NFTL1ScrollerCalls";
        const MIN_DATA_LENGTH: usize = 32usize;
        const COUNT: usize = 3usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::balances_slot(_) => <balances_slotCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::owners_slot(_) => <owners_slotCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::token_approvals_slot(_) => {
                    <token_approvals_slotCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            )
                -> alloy_sol_types::Result<NFTL1ScrollerCalls>] = &[
                {
                    fn owners_slot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTL1ScrollerCalls> {
                        <owners_slotCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTL1ScrollerCalls::owners_slot)
                    }
                    owners_slot
                },
                {
                    fn token_approvals_slot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTL1ScrollerCalls> {
                        <token_approvals_slotCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTL1ScrollerCalls::token_approvals_slot)
                    }
                    token_approvals_slot
                },
                {
                    fn balances_slot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTL1ScrollerCalls> {
                        <balances_slotCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTL1ScrollerCalls::balances_slot)
                    }
                    balances_slot
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::balances_slot(inner) => {
                    <balances_slotCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::owners_slot(inner) => {
                    <owners_slotCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::token_approvals_slot(inner) => {
                    <token_approvals_slotCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::balances_slot(inner) => {
                    <balances_slotCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::owners_slot(inner) => {
                    <owners_slotCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::token_approvals_slot(inner) => {
                    <token_approvals_slotCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`NFTL1Scroller`](self) contract instance.

    See the [wrapper's documentation](`NFTL1ScrollerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> NFTL1ScrollerInstance<T, P, N> {
        NFTL1ScrollerInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

    Returns a new instance of the contract, if the deployment was successful.

    For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<NFTL1ScrollerInstance<T, P, N>>>
    {
        NFTL1ScrollerInstance::<T, P, N>::deploy(provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
    and constructor arguments, if any.

    This is a simple wrapper around creating a `RawCallBuilder` with the data set to
    the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        NFTL1ScrollerInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`NFTL1Scroller`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`NFTL1Scroller`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct NFTL1ScrollerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for NFTL1ScrollerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("NFTL1ScrollerInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > NFTL1ScrollerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`NFTL1Scroller`](self) contract instance.

        See the [wrapper's documentation](`NFTL1ScrollerInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

        Returns a new instance of the contract, if the deployment was successful.

        For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(provider: P) -> alloy_contract::Result<NFTL1ScrollerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
        and constructor arguments, if any.

        This is a simple wrapper around creating a `RawCallBuilder` with the data set to
        the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> NFTL1ScrollerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> NFTL1ScrollerInstance<T, P, N> {
            NFTL1ScrollerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > NFTL1ScrollerInstance<T, P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`balances_slot`] function.
        pub fn balances_slot(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, balances_slotCall, N> {
            self.call_builder(&balances_slotCall { account })
        }
        ///Creates a new call builder for the [`owners_slot`] function.
        pub fn owners_slot(
            &self,
            tokenId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, owners_slotCall, N> {
            self.call_builder(&owners_slotCall { tokenId })
        }
        ///Creates a new call builder for the [`token_approvals_slot`] function.
        pub fn token_approvals_slot(
            &self,
            tokenId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, token_approvals_slotCall, N> {
            self.call_builder(&token_approvals_slotCall { tokenId })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > NFTL1ScrollerInstance<T, P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
