use solana_program::pubkey;
use solana_program::pubkey::Pubkey;

#[cfg(not(any(feature = "mainnet", feature = "devnet")))]
pub const BL_PROGRAM_ID: Pubkey = pubkey!("9zE4EQ5tJbEeMYwtS2w8KrSHTtTW4UPqwfbBSEkUrNCA");
// pub const BL_PROGRAM_ID: Pubkey = pubkey!("2iF3HaLpk6vuUXxGK3uDsxWoP5htC7NWZNctAevxZewY");

#[cfg(feature = "devnet")]
pub const BL_PROGRAM_ID: Pubkey = pubkey!("9zE4EQ5tJbEeMYwtS2w8KrSHTtTW4UPqwfbBSEkUrNCA");
// pub const BL_PROGRAM_ID: Pubkey = pubkey!("2iF3HaLpk6vuUXxGK3uDsxWoP5htC7NWZNctAevxZewY");

#[cfg(feature = "mainnet")]
pub const BL_PROGRAM_ID: Pubkey = pubkey!("BUDDYtQp7Di1xfojiCSVDksiYLQx511DPdj2nbtG9Yu5");
