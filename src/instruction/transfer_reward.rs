use crate::constants::BL_PROGRAM_ID;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use crate::utils::{get_account_meta_or_read_default, get_instruction_name_data};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct GeneralTransferRewardArgs {
    /// The amount of tokens to be transferred
    pub amount: u64,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct TransferUncheckedLocalSharedRewardArgs {
    /// The amount of tokens to be split and transferred
    pub total_amount: u64,
    /// How the token will be split in bps
    pub shares_in_bps: Vec<u16>,
    /// If the referrer members are included in the remaining accounts.
    /// Is needed if you want to use the on-chain analytics system
    pub members_included: bool,
}

///# Transfer Unchecked Local Shared Reward (SPL & SOL)
///
/// Transfer referral rewards to multiple user based on their share in bps
///
/// UNCHECKED: does not validate any accounts
/// LOCAL: does not use the global referral system of Buddylink
///
/// 1. `[writable, signer]` Authority of the account sending the funds.
/// 2. `[optional]` Only used if sending SOL, None if sending SPL.
/// 3. `[optional]` Mint, None if sending SOL.
/// 4. `[optional]` Token program, None if sending SOL.
/// 5. `[writable, optional]` None if sending SOL (will send from authority), else is Token Account
/// 7. `[writable]` Combination of referrer treasury / token account AND referrer member (if you want analytics on-chain).
/// 8. Transfer arguments
pub fn transfer_unchecked_local_shared_reward(
    authority: Pubkey,
    system_program: Option<Pubkey>,
    mint: Option<Pubkey>,
    token_program: Option<Pubkey>,
    from_account: Option<Pubkey>,
    remaining_accounts: &[Pubkey],
    transfer_args: &TransferUncheckedLocalSharedRewardArgs,
) -> Instruction {
    let mut instruction_data = get_instruction_name_data("transfer_reward_unchecked_multiple");
    instruction_data.extend_from_slice(&transfer_args.try_to_vec().unwrap());

    let mut accounts = vec![
        AccountMeta::new(authority, true),
        AccountMeta::new_readonly(system_program.unwrap_or(BL_PROGRAM_ID), false),
        AccountMeta::new_readonly(mint.unwrap_or(BL_PROGRAM_ID), false),
        AccountMeta::new_readonly(token_program.unwrap_or(BL_PROGRAM_ID), false),
        get_account_meta_or_read_default(&from_account),
    ];

    accounts.extend_from_slice(
        &remaining_accounts
            .iter()
            .map(|account| AccountMeta {
                pubkey: *account,
                is_signer: false,
                is_writable: true,
            })
            .collect::<Vec<AccountMeta>>(),
    );

    Instruction {
        program_id: BL_PROGRAM_ID,
        accounts,
        data: instruction_data,
    }
}

///# Transfer Secure Local Reward (SPL)
///
/// Transfer referral rewards to a single user, where validation is done.
///
/// SECURE: validation is done on the seeds, the owners of the accounts, the referral tree, the buddy doing the action is the same of the one in your system.
/// LOCAL: does not use the global referral system of Buddylink
///
/// 1. `[writable, signer]` Authority of the account sending the funds.
/// 2. `[]` Mint
/// 3. `[]` Token program
/// 4. `[writable]` Account sending the funds.
/// 5. `[writable]` Account receiving the funds (buddy link owned).
/// 6. `[writable]` Referrer member (account of the referrer within your organization).
/// 7. `[writable]` Referrer treasury (treasury that owns the referrer member #6).
/// 8. `[writable]` Referrer treasury for reward (treasury that is linked to the current mint, could be the same as #7).
/// 9. `[writable]` Buddy Link Profile of the referee.
/// 10. `[writable]` Buddy Link Paid buddy of the referee (could be the same as #9).
/// 11. `[writable]` Referee treasury (is owned by #10).
/// 12. `[writable]` Referee member (account of the referee within your organization).
/// 13. Transfer arguments
#[allow(clippy::too_many_arguments)]
pub fn transfer_secure_local_reward(
    authority: Pubkey,
    mint: Pubkey,
    token_program: Pubkey,
    from_token_account: Pubkey,
    referrer_token_account: Pubkey,
    referrer_member: Pubkey,
    referrer_treasury: Pubkey,
    referrer_treasury_for_reward: Pubkey,
    referee_buddy_profile: Pubkey,
    referee_buddy: Pubkey,
    referee_treasury: Pubkey,
    referee_member: Pubkey,
    transfer_args: &GeneralTransferRewardArgs,
) -> Instruction {
    let mut instruction_data = get_instruction_name_data("transfer_reward_secure_no_global");
    instruction_data.extend_from_slice(&transfer_args.try_to_vec().unwrap());

    Instruction {
        program_id: BL_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new(from_token_account, false),
            AccountMeta::new(referrer_member, false),
            AccountMeta::new(referrer_treasury, false),
            AccountMeta::new(referrer_treasury_for_reward, false),
            AccountMeta::new(referee_buddy_profile, false),
            AccountMeta::new(referee_buddy, false),
            AccountMeta::new(referee_treasury, false),
            AccountMeta::new(referee_member, false),
            AccountMeta::new(referrer_token_account, false),
        ],
        data: instruction_data,
    }
}

///# Transfer Checked Global Reward (SPL)
///
/// Transfer referral rewards to a single user, where validation is done.
///
/// CHECKED: validation is done on the seeds, the owners of the accounts, the referral tree.
/// GLOBAL: option to also use the global referral system of buddylink
///
/// 1. `[writable, signer]` Authority of the account sending the funds.
/// 2. `[]` Mint
/// 3. `[]` Token program
/// 4. `[writable]` Account sending the funds.
/// 5. `[writable]` Account receiving the funds (buddy link owned).
/// 6. `[writable, optional]` Referrer member (account of the referrer within your organization) (None if don't want on-chain analytics).
/// 7. `[writable]` Referrer treasury (treasury that owns the referrer member #6).
/// 8. `[writable]` Referrer treasury for reward (treasury that is linked to the current mint, could be the same as #7).
/// 9. `[writable]` Referee member (account of the referee within your organization).
/// 10. `[writable]` Global referrer treasury (treasury of the global referrer of current referee) (None if user doesn't have global referrer).
/// 11. `[writable]` Global referrer treasury for reward (treasury of the global referrer of current referee that is linked to the current mint, could be same as #10) (None if user doesn't have global referrer).
/// 12. Transfer arguments
#[allow(clippy::too_many_arguments)]
pub fn transfer_checked_global_reward(
    authority: Pubkey,
    mint: Pubkey,
    token_program: Pubkey,
    from_token_account: Pubkey,
    referrer_token_account: Pubkey,
    referrer_member: Option<Pubkey>,
    referrer_treasury: Pubkey,
    referrer_treasury_for_reward: Pubkey,
    referee_member: Pubkey,
    buddy_global_referrer_treasury: Option<Pubkey>,
    buddy_global_referrer_token_account: Option<Pubkey>,
    transfer_args: &GeneralTransferRewardArgs,
) -> Instruction {
    let mut instruction_data = get_instruction_name_data("transfer_reward_spl");
    instruction_data.extend_from_slice(&transfer_args.try_to_vec().unwrap());

    Instruction {
        program_id: BL_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(authority, true),
            get_account_meta_or_read_default(&buddy_global_referrer_treasury),
            get_account_meta_or_read_default(&buddy_global_referrer_token_account),
            get_account_meta_or_read_default(&referrer_member),
            AccountMeta::new(referrer_treasury, false),
            AccountMeta::new(referrer_treasury_for_reward, false),
            AccountMeta::new(referee_member, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new(from_token_account, false),
            AccountMeta::new(referrer_token_account, false),
        ],
        data: instruction_data,
    }
}

///# Transfer Checked Global Only Reward (SPL & SOL)
///
/// Transfer referral rewards within the global referral buddy link system.
/// If you're an organization, you probably won't use this.
///
/// CHECKED: validation is done on the seeds, the owners of the accounts, the referral tree.
/// GLOBAL ONLY: using only the global referral system of buddylink
///
/// 1. `[writable, signer]` Authority of the account sending the funds.
/// 2. `[optional]` Only used if sending SOL, None if sending SPL.
/// 3. `[optional]` Mint, None if sending SOL.
/// 4. `[optional]` Token program, None if sending SOL.
/// 5. `[writable, optional]` None if sending SOL (will send from authority), else is Token Account
/// 6. `[writable, optional]` None if receiving SOL (will send to treasury), else is Token Account
/// 7. `[writable]` Global referrer treasury (treasury of the global referrer of current referee).
/// 8. `[writable]` Global referrer treasury for reward (treasury of the global referrer of current referee that is linked to the current mint, could be same as #10).
/// 9. `[writable]` Buddy Link Profile of the referee.
/// 10. `[writable]` Buddy Link Paid buddy of the referee (could be the same as #9).
/// 11. Transfer arguments
#[allow(clippy::too_many_arguments)]
pub fn transfer_checked_global_only_reward(
    authority: Pubkey,
    system_program: Option<Pubkey>,
    mint: Option<Pubkey>,
    token_program: Option<Pubkey>,
    from_token_account: Option<Pubkey>,
    referrer_token_account: Option<Pubkey>,
    buddy_global_referrer_treasury: Pubkey,
    buddy_global_referrer_treasury_for_reward: Pubkey,
    referee_buddy_profile: Pubkey,
    referee_buddy: Pubkey,
    transfer_args: &GeneralTransferRewardArgs,
) -> Instruction {
    let mut instruction_data = get_instruction_name_data("transfer_reward_global");
    instruction_data.extend_from_slice(&transfer_args.try_to_vec().unwrap());

    Instruction {
        program_id: BL_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(buddy_global_referrer_treasury, false),
            AccountMeta::new(buddy_global_referrer_treasury_for_reward, false),
            AccountMeta::new_readonly(referee_buddy_profile, false),
            AccountMeta::new_readonly(referee_buddy, false),
            AccountMeta::new_readonly(system_program.unwrap_or(BL_PROGRAM_ID), false),
            AccountMeta::new_readonly(mint.unwrap_or(BL_PROGRAM_ID), false),
            AccountMeta::new_readonly(token_program.unwrap_or(BL_PROGRAM_ID), false),
            get_account_meta_or_read_default(&referrer_token_account),
            get_account_meta_or_read_default(&from_token_account),
        ],
        data: instruction_data,
    }
}
