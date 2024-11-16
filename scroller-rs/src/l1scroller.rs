use std::{str::FromStr, sync::Arc};

use alloy::{
    network::Ethereum,
    primitives::{
        aliases::{U24, U48, U96},
        Address, Bytes, U128, U160, U256, U32, U64,
    },
    providers::{
        fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
        Identity, ProviderBuilder, RootProvider,
    },
    transports::http::{reqwest::Url, Client, Http},
};

use crate::{
    bindings::l1scroller::L1Scroller::{self},
    error::L1ScrollerError,
};

pub type ProviderType = RootProvider<Http<Client>>;
pub type L1ScrollerContracttype = L1Scroller::L1ScrollerInstance<
    Http<Client>,
    FillProvider<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        RootProvider<Http<Client>>,
        Http<Client>,
        Ethereum,
    >,
>;

pub struct Scroller<C> {
    scroller_contract: Arc<C>,
}

impl Scroller<L1ScrollerContracttype> {
    pub fn new(rpc_url: &str, scroller_addr: Address) -> eyre::Result<Self> {
        let url = Url::from_str(rpc_url)?;
        let pr = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_http(url);
        let scroller_contract_instance = Arc::new(L1Scroller::new(scroller_addr, pr));
        Ok(Self {
            scroller_contract: scroller_contract_instance,
        })
    }

    pub async fn read_slot(
        &self,
        l1_contract_addr: Address,
        slot: U256,
    ) -> eyre::Result<Bytes, L1ScrollerError> {
        Ok(self
            .scroller_contract
            .readSlot(l1_contract_addr, slot)
            .call()
            .await
            .map_err(|_| L1ScrollerError::ReadSlotError)?
            ._0)
    }

    pub async fn read_multiple_slots(
        &self,
        l1_contract_addr: Address,
        slot: Vec<U256>,
    ) -> eyre::Result<Bytes, L1ScrollerError> {
        Ok(self
            .scroller_contract
            .readMultipleSlots(l1_contract_addr, slot)
            .call()
            .await
            .map_err(|_| L1ScrollerError::ReadMultipleSlotError)?
            ._0)
    }

    pub async fn read_uint(
        &self,
        l1_contract_addr: Address,
        slot: U256,
    ) -> eyre::Result<U256, L1ScrollerError> {
        self.scroller_contract
            .readUint(l1_contract_addr, slot)
            .call()
            .await
            .unwrap();
        Ok(self
            .scroller_contract
            .readUint(l1_contract_addr, slot)
            .call()
            .await
            .map_err(|_| L1ScrollerError::ReadUintError)?
            ._0)
    }

    pub async fn read_uint160(
        &self,
        l1_contract_addr: Address,
        slot: U256,
    ) -> eyre::Result<U160, L1ScrollerError> {
        Ok(self
            .scroller_contract
            .readUint160(l1_contract_addr, slot)
            .call()
            .await
            .map_err(|_| L1ScrollerError::ReadU160Error)?
            ._0)
    }

    pub async fn read_address(
        &self,
        l1_contract_addr: Address,
        slot: U256,
    ) -> eyre::Result<Address, L1ScrollerError> {
        Ok(self
            .scroller_contract
            .readAddress(l1_contract_addr, slot)
            .call()
            .await
            .map_err(|_| L1ScrollerError::ReadAddressError)?
            ._0)
    }

    pub async fn read_uint96(
        &self,
        l1_contract_addr: Address,
        slot: U256,
    ) -> eyre::Result<U96, L1ScrollerError> {
        Ok(self
            .scroller_contract
            .readUint96(l1_contract_addr, slot)
            .call()
            .await
            .map_err(|_| L1ScrollerError::ReadU96Error)?
            ._0)
    }

    pub async fn read_uint48(
        &self,
        l1_contract_addr: Address,
        slot: U256,
    ) -> eyre::Result<U48, L1ScrollerError> {
        Ok(self
            .scroller_contract
            .readUint48(l1_contract_addr, slot)
            .call()
            .await
            .map_err(|_| L1ScrollerError::ReadU48Error)?
            ._0)
    }

    pub async fn read_uint24(
        &self,
        l1_contract_addr: Address,
        slot: U256,
    ) -> eyre::Result<U24, L1ScrollerError> {
        Ok(self
            .scroller_contract
            .readUint24(l1_contract_addr, slot)
            .call()
            .await
            .map_err(|_| L1ScrollerError::ReadU24Error)?
            ._0)
    }

    pub async fn read_uint8(
        &self,
        l1_contract_addr: Address,
        slot: U256,
    ) -> eyre::Result<u8, L1ScrollerError> {
        Ok(self
            .scroller_contract
            .readUint8(l1_contract_addr, slot)
            .call()
            .await
            .map_err(|_| L1ScrollerError::ReadU8Error)?
            ._0)
    }

    pub async fn read_uint32(
        &self,
        l1_contract_addr: Address,
        slot: U256,
    ) -> eyre::Result<U32, L1ScrollerError> {
        Ok(U32::from(
            self.scroller_contract
                .readUint32(l1_contract_addr, slot)
                .call()
                .await
                .map_err(|_| L1ScrollerError::ReadU32Error)?
                ._0,
        ))
    }

    pub async fn read_uint64(
        &self,
        l1_contract_addr: Address,
        slot: U256,
    ) -> eyre::Result<U64, L1ScrollerError> {
        Ok(U64::from(
            self.scroller_contract
                .readUint64(l1_contract_addr, slot)
                .call()
                .await
                .map_err(|_| L1ScrollerError::ReadU64Error)?
                ._0,
        ))
    }

    pub async fn read_uint128(
        &self,
        l1_contract_addr: Address,
        slot: U256,
    ) -> eyre::Result<U128, L1ScrollerError> {
        Ok(U128::from(
            self.scroller_contract
                .readUint128(l1_contract_addr, slot)
                .call()
                .await
                .map_err(|_| L1ScrollerError::ReadU64Error)?
                ._0,
        ))
    }

    pub async fn read_string(
        &self,
        l1_contract_addr: Address,
        slot: U256,
    ) -> eyre::Result<String, L1ScrollerError> {
        self.scroller_contract
            .readString(l1_contract_addr, slot)
            .call()
            .await
            .unwrap();

        Ok(self
            .scroller_contract
            .readString(l1_contract_addr, slot)
            .call()
            .await
            .map_err(|_| L1ScrollerError::ReadStringError)?
            ._0)
    }
}

#[cfg(test)]
mod tests {

    use alloy::{primitives::address, providers::Provider, rpc::client::RpcClient};


    use super::*;

    #[test]
    fn test_new() {
        let rpc_url = "https://l1sload-rpc.scroll.io";
        let scroller_addr: Address = Address::ZERO;
        let s = Scroller::new(rpc_url, scroller_addr).unwrap();
    }

    #[tokio::test]
    async fn test_read_string() {
        let rpc_url = "https://l1sload-rpc.scroll.io";
        let url = Url::from_str(&rpc_url).unwrap();
        let rpc_client = RpcClient::new_http(url);
        let pr = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_client(rpc_client);
        // RPC calls work ,but eth_call does not work
        let block = pr.get_max_priority_fee_per_gas().await.unwrap();
        let scroller_addr: Address =
            Address::from_str("0x59c6C9958b9c3603D0B753d90f370704e64D9311").unwrap();
        let scroller = Scroller::new(rpc_url, scroller_addr).unwrap();

        // eth_call does not work on the L1SLOAD devnet as discussed with scroll team
        // let s = scroller
        //     .read_slot(
        //         address!("9487f81d024290F2919b912f7987d030482ed344"),
        //         "0".parse().unwrap(),
        //     )
        //     .await
        //     .unwrap();
    }
}
