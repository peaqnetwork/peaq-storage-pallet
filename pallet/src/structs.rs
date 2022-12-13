use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::RuntimeDebug;
use sp_std::vec::Vec;

/// Attributes of a User.
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, RuntimeDebug, TypeInfo,
)]
pub struct UserAttribute {
    pub profile: Vec<u8>,
    pub sessions: Vec<u8>,
}
