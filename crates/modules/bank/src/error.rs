use cosmrs::AccountId;

pub use displaydoc::Display;

#[derive(Debug, Display)]
pub enum Error {
    /// failed to decode message
    MsgDecodeFailure,
    /// failed to validate message: `{reason}`
    MsgValidationFailure { reason: String },
    /// account `{account}` doesn't exist
    NonExistentAccount { account: AccountId },
    /// insufficient funds in sender account
    InsufficientSourceFunds,
    /// receiver account funds overflow
    DestFundOverflow,
    /// Store error: `{reason}`
    Store { reason: String },
}
