// These are the concrete types we will use in our simple state machine.
// Modules are configured for these types directly, and they satisfy all of our
// trait requirements.
pub type AccountId = String;
pub type Balance = u128;
pub type BlockNumber = u32;
pub type Nonce = u32;
pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
pub type Header = crate::support::Header<BlockNumber>;
pub type Block = crate::support::Block<Header, Extrinsic>;
pub type Content = &'static str;