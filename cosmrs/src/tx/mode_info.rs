//! Mode info.

use super::SignMode;
use crate::{proto, Error, ErrorReport, Result};

/// [`ModeInfo`] describes the signing mode of a single or nested multisig signer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModeInfo {
    /// Single represents a single signer.
    Single(Single),
}

impl ModeInfo {
    /// Create [`ModeInfo`] for a single signer using the given mode.
    pub fn single(sign_mode: SignMode) -> ModeInfo {
        ModeInfo::Single(sign_mode.into())
    }
}

impl TryFrom<proto::cosmos::tx::v1beta1::ModeInfo> for ModeInfo {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmos::tx::v1beta1::ModeInfo) -> Result<ModeInfo> {
        match proto.sum {
            Some(proto::cosmos::tx::v1beta1::mode_info::Sum::Single(single)) => {
                Ok(ModeInfo::Single(single.into()))
            }
            None => Err(Error::MissingField { name: "mode_info" }.into()),
        }
    }
}

impl From<ModeInfo> for proto::cosmos::tx::v1beta1::ModeInfo {
    fn from(mode_info: ModeInfo) -> proto::cosmos::tx::v1beta1::ModeInfo {
        proto::cosmos::tx::v1beta1::ModeInfo {
            sum: Some(match mode_info {
                ModeInfo::Single(single) => {
                    proto::cosmos::tx::v1beta1::mode_info::Sum::Single(single.into())
                }
            }),
        }
    }
}

impl From<Single> for ModeInfo {
    fn from(single: Single) -> ModeInfo {
        ModeInfo::Single(single)
    }
}

/// [`Single`] is the mode info for a single signer.
///
/// It is structured as a message to allow for additional fields such as locale for
/// `SIGN_MODE_TEXTUAL` in the future.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Single {
    /// Signing mode of the single signer.
    pub mode: SignMode,
}

impl From<proto::cosmos::tx::v1beta1::mode_info::Single> for Single {
    fn from(proto: proto::cosmos::tx::v1beta1::mode_info::Single) -> Single {
        Single { mode: proto.mode() }
    }
}

impl From<Single> for proto::cosmos::tx::v1beta1::mode_info::Single {
    fn from(single: Single) -> proto::cosmos::tx::v1beta1::mode_info::Single {
        proto::cosmos::tx::v1beta1::mode_info::Single {
            mode: single.mode as _,
        }
    }
}

impl From<SignMode> for Single {
    fn from(mode: SignMode) -> Single {
        Single { mode }
    }
}

impl From<Single> for SignMode {
    fn from(single: Single) -> SignMode {
        single.mode
    }
}
