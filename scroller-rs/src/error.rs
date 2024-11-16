use thiserror::Error;

#[derive(Debug, Error)]
pub enum L1ScrollerError {
    #[error("Failed to read slot")]
    ReadSlotError,

    #[error("Failed to read multiple slot")]
    ReadMultipleSlotError,

    #[error("Failed to read U256")]
    ReadUintError,

    #[error("Failed to read U160")]
    ReadU160Error,

    #[error("Failed to read Address")]
    ReadAddressError,

    #[error("Failed to read U96")]
    ReadU96Error,

    #[error("Failed to read U48")]
    ReadU48Error,

    #[error("Failed to read U24")]
    ReadU24Error,

    #[error("Failed to read U8")]
    ReadU8Error,

    #[error("Failed to read U32")]
    ReadU32Error,

    #[error("Failed to read U64")]
    ReadU64Error,

    #[error("Failed to read String")]
    ReadStringError,
}
