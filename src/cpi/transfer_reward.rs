use crate::constants::constants::BL_PROGRAM_ID;
use crate::instruction;
use crate::instruction::{GeneralTransferRewardArgs, TransferUncheckedLocalSharedRewardArgs};
use crate::utils::{get_account_info_or_default, get_key_or_none};
use anchor_lang::prelude::*;
use anchor_lang::{Accounts, Key, ToAccountInfo};
use anchor_spl::token::{Mint, Token, TokenAccount};
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke_signed;

#[derive(Accounts)]
pub struct TransferRewardUncheckedMultiple<'info> {
    /// CHECK: The buddylink program
    #[account(executable, address = BL_PROGRAM_ID)]
    pub buddy_link_program: UncheckedAccount<'info>,

    /// Authority of the account sending the funds.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// System Program
    pub system_program: Option<Program<'info, System>>,

    /// Mint
    #[account()]
    pub mint: Option<Account<'info, Mint>>,
    /// Token program
    pub token_program: Option<Program<'info, Token>>,

    /// Account sending the funds.
    #[account(mut)]
    pub from_token_account: Option<Account<'info, TokenAccount>>,
    /*
    Remaining accounts with be the referrer treasuries paired with referrer_member for sol or token accounts for spl
     */
}

pub fn transfer_unchecked_local_shared_reward<'info>(
    ctx: Context<'_, '_, '_, 'info, TransferRewardUncheckedMultiple<'info>>,
    total_amount: u64,
    shares_in_bps: Vec<u16>,
    members_included: bool,
    transfer_signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let remaining_accounts_key: Vec<Pubkey> =
        ctx.remaining_accounts.iter().map(|x| x.key()).collect();

    let default_account_info = ctx.accounts.buddy_link_program.to_account_info();

    let instruction = instruction::transfer_unchecked_local_shared_reward(
        ctx.accounts.authority.key(),
        Some(
            get_account_info_or_default(&ctx.accounts.system_program, &default_account_info).key(),
        ),
        get_key_or_none(&ctx.accounts.mint),
        Some(get_account_info_or_default(&ctx.accounts.token_program, &default_account_info).key()),
        get_key_or_none(&ctx.accounts.from_token_account),
        &remaining_accounts_key,
        &TransferUncheckedLocalSharedRewardArgs {
            total_amount,
            shares_in_bps,
            members_included,
        },
    );

    let mut account_infos = vec![
        ctx.accounts.authority.to_account_info(),
        get_account_info_or_default(&ctx.accounts.system_program, &default_account_info),
        get_account_info_or_default(&ctx.accounts.mint, &default_account_info),
        get_account_info_or_default(&ctx.accounts.token_program, &default_account_info),
        get_account_info_or_default(&ctx.accounts.from_token_account, &default_account_info),
    ];

    account_infos.extend_from_slice(ctx.remaining_accounts);

    invoke_signed(&instruction, &account_infos, transfer_signer_seeds)
}

#[derive(Accounts)]
pub struct TransferSecureLocalReward<'info> {
    /// CHECK: The buddylink program
    #[account(executable, address = BL_PROGRAM_ID)]
    pub buddy_link_program: UncheckedAccount<'info>,

    /// Authority of the account sending the funds.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// Mint
    #[account()]
    pub mint: Account<'info, Mint>,
    /// Token program
    pub token_program: Program<'info, Token>,

    /// Account sending the funds.
    #[account(mut)]
    pub from_token_account: Account<'info, TokenAccount>,
    /// Account receiving the funds (buddy link owned).
    #[account(mut)]
    pub referrer_token_account: Account<'info, TokenAccount>,

    /// CHECK: Referrer member (account of the referrer within your organization).
    #[account(mut)]
    pub referrer_member: UncheckedAccount<'info>,
    /// CHECK: Referrer treasury (treasury that owns the referrer member ).
    #[account(mut)]
    pub referrer_treasury: UncheckedAccount<'info>,
    /// CHECK: Referrer treasury for reward (treasury that is linked to the current mint, could be the same as above).
    #[account(mut)]
    pub referrer_treasury_for_reward: UncheckedAccount<'info>,

    /// CHECK: Buddy Link Profile of the referee.
    #[account()]
    pub referee_buddy_profile: UncheckedAccount<'info>,
    /// CHECK: Buddy Link Paid buddy of the referee (could be the same as above).
    #[account()]
    pub referee_buddy: UncheckedAccount<'info>,
    /// CHECK: Referee treasury (is owned by above).
    #[account(mut)]
    pub referee_treasury: UncheckedAccount<'info>,
    /// CHECK: Referee member (account of the referee within your organization).
    #[account(mut)]
    pub referee_member: UncheckedAccount<'info>,
}

pub fn transfer_secure_local_reward(
    ctx: Context<TransferSecureLocalReward>,
    amount: u64,
    transfer_signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let instruction = instruction::transfer_secure_local_reward(
        ctx.accounts.authority.key(),
        ctx.accounts.mint.key(),
        ctx.accounts.token_program.key(),
        ctx.accounts.from_token_account.key(),
        ctx.accounts.referrer_token_account.key(),
        ctx.accounts.referrer_member.key(),
        ctx.accounts.referrer_treasury.key(),
        ctx.accounts.referrer_treasury_for_reward.key(),
        ctx.accounts.referee_buddy_profile.key(),
        ctx.accounts.referee_buddy.key(),
        ctx.accounts.referee_treasury.key(),
        ctx.accounts.referee_member.key(),
        &GeneralTransferRewardArgs { amount },
    );

    invoke_signed(
        &instruction,
        &[
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.from_token_account.to_account_info(),
            ctx.accounts.referrer_member.to_account_info(),
            ctx.accounts.referrer_treasury.to_account_info(),
            ctx.accounts.referrer_treasury_for_reward.to_account_info(),
            ctx.accounts.referee_buddy_profile.to_account_info(),
            ctx.accounts.referee_buddy.to_account_info(),
            ctx.accounts.referee_treasury.to_account_info(),
            ctx.accounts.referee_member.to_account_info(),
            ctx.accounts.referrer_token_account.to_account_info(),
        ],
        transfer_signer_seeds,
    )
}

#[derive(Accounts)]
pub struct TransferCheckedGlobalReward<'info> {
    /// CHECK: The buddylink program
    #[account(executable, address = BL_PROGRAM_ID)]
    pub buddy_link_program: UncheckedAccount<'info>,

    /// Authority of the account sending the funds.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// Mint
    #[account()]
    pub mint: Account<'info, Mint>,
    /// Token program
    pub token_program: Program<'info, Token>,

    /// Account sending the funds.
    #[account(mut)]
    pub from_token_account: Account<'info, TokenAccount>,
    /// Account receiving the funds (buddy link owned).
    #[account(mut)]
    pub referrer_token_account: Account<'info, TokenAccount>,

    /// CHECK: Referrer member (account of the referrer within your organization) (None if don't want on-chain analytics).
    #[account(mut)]
    pub referrer_member: Option<UncheckedAccount<'info>>,
    /// CHECK: Referrer treasury (treasury that owns the referrer member ).
    #[account(mut)]
    pub referrer_treasury: UncheckedAccount<'info>,
    /// CHECK: Referrer treasury for reward (treasury that is linked to the current mint, could be the same as above).
    #[account(mut)]
    pub referrer_treasury_for_reward: UncheckedAccount<'info>,

    /// CHECK: Referee member (account of the referee within your organization).
    #[account(mut)]
    pub referee_member: UncheckedAccount<'info>,

    /// CHECK: Global referrer treasury (treasury of the global referrer of current referee) (None if user doesn't have global referrer).
    #[account(mut)]
    pub buddy_global_referrer_treasury: Option<UncheckedAccount<'info>>,
    /// CHECK: Global referrer treasury for reward (treasury of the global referrer of current referee that is linked to the current mint, could be same as above) (None if user doesn't have global referrer).
    #[account(mut)]
    pub buddy_global_referrer_treasury_for_reward: Option<UncheckedAccount<'info>>,
}

pub fn transfer_checked_global_reward(
    ctx: Context<TransferCheckedGlobalReward>,
    amount: u64,
    transfer_signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let instruction = instruction::transfer_checked_global_reward(
        ctx.accounts.authority.key(),
        ctx.accounts.mint.key(),
        ctx.accounts.token_program.key(),
        ctx.accounts.from_token_account.key(),
        ctx.accounts.referrer_token_account.key(),
        get_key_or_none(&ctx.accounts.referrer_member),
        ctx.accounts.referrer_treasury.key(),
        ctx.accounts.referrer_treasury_for_reward.key(),
        ctx.accounts.referee_member.key(),
        get_key_or_none(&ctx.accounts.buddy_global_referrer_treasury),
        get_key_or_none(&ctx.accounts.buddy_global_referrer_treasury_for_reward),
        &GeneralTransferRewardArgs { amount },
    );

    let default_account_info = ctx.accounts.buddy_link_program.to_account_info();

    invoke_signed(
        &instruction,
        &[
            ctx.accounts.authority.to_account_info(),
            get_account_info_or_default(
                &ctx.accounts.buddy_global_referrer_treasury,
                &default_account_info,
            ),
            get_account_info_or_default(
                &ctx.accounts.buddy_global_referrer_treasury_for_reward,
                &default_account_info,
            ),
            ctx.accounts.referee_member.to_account_info(),
            ctx.accounts.referrer_treasury.to_account_info(),
            ctx.accounts.referrer_treasury_for_reward.to_account_info(),
            ctx.accounts.referee_member.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.from_token_account.to_account_info(),
            ctx.accounts.referrer_token_account.to_account_info(),
        ],
        transfer_signer_seeds,
    )
}

#[derive(Accounts)]
pub struct TransferCheckedGlobalOnlyReward<'info> {
    /// CHECK: The buddylink program
    #[account(executable, address = BL_PROGRAM_ID)]
    pub buddy_link_program: UncheckedAccount<'info>,

    /// Authority of the account sending the funds.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// System Program - Only used if sending SOL, None if sending SPL.
    pub system_program: Option<Program<'info, System>>,

    /// Mint, None if sending SOL.
    #[account()]
    pub mint: Option<Account<'info, Mint>>,
    /// Token program, None if sending SOL.
    pub token_program: Option<Program<'info, Token>>,

    /// From token account - None if sending SOL (will send from authority), else is Token Account
    #[account(mut)]
    pub from_token_account: Option<Account<'info, TokenAccount>>,
    /// Referrer token account - None if receiving SOL (will send to treasury), else is Token Account
    #[account(mut)]
    pub referrer_token_account: Option<Account<'info, TokenAccount>>,

    /// CHECK: Global referrer treasury (treasury of the global referrer of current referee).
    #[account(mut)]
    pub global_referrer_treasury: UncheckedAccount<'info>,
    /// CHECK: Global referrer treasury for reward (treasury of the global referrer of current referee that is linked to the current mint, could be same as ).
    #[account(mut)]
    pub global_referrer_treasury_for_reward: UncheckedAccount<'info>,

    /// CHECK: Buddy Link Profile of the referee.
    #[account()]
    pub referee_buddy_profile: UncheckedAccount<'info>,
    /// CHECK: Buddy Link Paid buddy of the referee (could be the same as above).
    #[account()]
    pub referee_buddy: UncheckedAccount<'info>,
}

pub fn transfer_checked_global_only_reward(
    ctx: Context<TransferCheckedGlobalOnlyReward>,
    amount: u64,
    transfer_signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let default_account_info = ctx.accounts.buddy_link_program.to_account_info();

    let instruction = instruction::transfer_checked_global_only_reward(
        ctx.accounts.authority.key(),
        Some(
            get_account_info_or_default(&ctx.accounts.system_program, &default_account_info).key(),
        ),
        get_key_or_none(&ctx.accounts.mint),
        Some(get_account_info_or_default(&ctx.accounts.token_program, &default_account_info).key()),
        get_key_or_none(&ctx.accounts.from_token_account),
        get_key_or_none(&ctx.accounts.referrer_token_account),
        ctx.accounts.global_referrer_treasury.key(),
        ctx.accounts.global_referrer_treasury_for_reward.key(),
        ctx.accounts.referee_buddy_profile.key(),
        ctx.accounts.referee_buddy.key(),
        &GeneralTransferRewardArgs { amount },
    );

    invoke_signed(
        &instruction,
        &[
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.global_referrer_treasury.to_account_info(),
            ctx.accounts
                .global_referrer_treasury_for_reward
                .to_account_info(),
            ctx.accounts.referee_buddy_profile.to_account_info(),
            ctx.accounts.referee_buddy.to_account_info(),
            get_account_info_or_default(&ctx.accounts.system_program, &default_account_info),
            get_account_info_or_default(&ctx.accounts.mint, &default_account_info),
            get_account_info_or_default(&ctx.accounts.token_program, &default_account_info),
            get_account_info_or_default(
                &ctx.accounts.referrer_token_account,
                &default_account_info,
            ),
            get_account_info_or_default(&ctx.accounts.from_token_account, &default_account_info),
        ],
        transfer_signer_seeds,
    )
}
