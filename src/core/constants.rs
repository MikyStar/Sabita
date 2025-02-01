pub const LENGTH_DIMENSION: u8 = 9;
pub const MAX_NB_VALUES: u8 = LENGTH_DIMENSION.pow(2);

pub const TO_BE_SOLVED: u8 = 0;

/// The minimum number of element to be present in order to ensure a single solution
/// @see https://www.reddit.com/r/math/comments/r931e3/comment/hn9h2v6/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
pub const MINIMUM_PROVIDED: u8 = 17;

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
