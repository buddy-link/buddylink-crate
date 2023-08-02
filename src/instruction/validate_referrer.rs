use crate::constants::constants::BL_PROGRAM_ID;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;

///# Validate Referrer
///
/// Validates that the referee, referrer, and all other accounts are valid within BuddyLink. Will error out if they are not valid.
///
/// 1. `[writable, signer]` Payer for the transaction.
/// 2. `[]` Owner of the referee accounts
/// 3. `[optional]` Mint (if you want to validate the referral tree with a specific mint)
/// 4. `[optional]` Token account linked to the mint (if you want to validate the referral tree with a specific mint)
/// 5. `[optional]` Referrer member (account of the referrer within your organization) (if you want to validate the referrer of the current referee).
/// 6. `[optional]` Referrer treasury (treasury that owns the referrer member #5) (if you want to validate the referrer of the current referee).
/// 7. `[optional]` Referrer treasury for reward (treasury that is linked to the current mint, could be the same as #6) (if you want to validate the referrer of the current referee).
/// 8. `[]` Buddy Link Profile of the referee.
/// 9. `[]` Buddy Link Paid buddy of the referee (could be the same as #8).
/// 10. `[writable]` Referee treasury (is owned by #9).
/// 11. `[]` Referee member (account of the referee within your organization).
#[allow(clippy::too_many_arguments)]
pub fn validate_referrer(
    payer: Pubkey,
    authority: Pubkey,
    mint: Option<Pubkey>,
    referrer_token_account: Option<Pubkey>,
    referrer_member: Option<Pubkey>,
    referrer_treasury: Option<Pubkey>,
    referrer_treasury_for_reward: Option<Pubkey>,
    referee_buddy_profile: Pubkey,
    referee_buddy: Pubkey,
    referee_treasury: Pubkey,
    referee_member: Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(payer, true),
        AccountMeta::new_readonly(authority, false),
        AccountMeta::new_readonly(referee_buddy_profile, false),
        AccountMeta::new_readonly(referee_buddy, false),
        AccountMeta::new(referee_treasury, false),
        AccountMeta::new_readonly(referee_member, false),
        AccountMeta::new_readonly(referrer_member.unwrap_or(BL_PROGRAM_ID), false),
        AccountMeta::new_readonly(referrer_treasury.unwrap_or(BL_PROGRAM_ID), false),
        AccountMeta::new_readonly(referrer_treasury_for_reward.unwrap_or(BL_PROGRAM_ID), false),
        AccountMeta::new_readonly(mint.unwrap_or(BL_PROGRAM_ID), false),
        AccountMeta::new_readonly(referrer_token_account.unwrap_or(BL_PROGRAM_ID), false),
    ];

    Instruction {
        program_id: BL_PROGRAM_ID,
        accounts,
        data: vec![],
    }
}
