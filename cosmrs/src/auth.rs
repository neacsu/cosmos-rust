//! Auth functionality.

use crate::crypto::PublicKey;
use crate::tx::{AccountNumber, SequenceNumber};
use crate::Result;
use crate::{proto, AccountId, ErrorReport};

/// BaseAccount defines a base account type. It contains all the necessary fields
/// for basic account functionality. Any custom account type should extend this
/// type for additional functionality (e.g. vesting).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BaseAccount {
    /// Bech32 [`AccountId`] of this account.
    pub address: AccountId,

    /// Optional [`PublicKey`] associated with this account.
    pub pubkey: Option<PublicKey>,

    /// `account_number` is the account number of the account in state
    pub account_number: AccountNumber,

    /// Sequence of the account, which describes the number of committed transactions signed by a
    /// given address.
    pub sequence: SequenceNumber,
}

impl TryFrom<proto::cosmos::auth::v1beta1::BaseAccount> for BaseAccount {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmos::auth::v1beta1::BaseAccount) -> Result<BaseAccount> {
        Ok(BaseAccount {
            address: proto.address.parse()?,
            pubkey: proto.pub_key.map(PublicKey::try_from).transpose()?,
            account_number: proto.account_number,
            sequence: proto.sequence,
        })
    }
}

impl From<BaseAccount> for proto::cosmos::auth::v1beta1::BaseAccount {
    fn from(account: BaseAccount) -> proto::cosmos::auth::v1beta1::BaseAccount {
        proto::cosmos::auth::v1beta1::BaseAccount {
            address: account.address.to_string(),
            pub_key: account.pubkey.map(Into::into),
            account_number: account.account_number,
            sequence: account.sequence,
        }
    }
}

/// ModuleAccount defines an account for modules that holds coins on a pool.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleAccount {
    /// [`BaseAccount`] specification of this module account.
    pub base_account: Option<BaseAccount>,

    /// Name of the module.
    pub name: String,

    /// Permissions associated with this module account.
    pub permissions: Vec<String>,
}

impl TryFrom<proto::cosmos::auth::v1beta1::ModuleAccount> for ModuleAccount {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmos::auth::v1beta1::ModuleAccount) -> Result<ModuleAccount> {
        Ok(ModuleAccount {
            base_account: proto.base_account.map(TryFrom::try_from).transpose()?,
            name: proto.name,
            permissions: proto.permissions,
        })
    }
}

impl From<ModuleAccount> for proto::cosmos::auth::v1beta1::ModuleAccount {
    fn from(account: ModuleAccount) -> proto::cosmos::auth::v1beta1::ModuleAccount {
        proto::cosmos::auth::v1beta1::ModuleAccount {
            base_account: account.base_account.map(Into::into),
            name: account.name,
            permissions: account.permissions,
        }
    }
}
