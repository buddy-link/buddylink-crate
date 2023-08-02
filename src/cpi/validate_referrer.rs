use crate::constants::constants::BL_PROGRAM_ID;
use crate::instruction;
use crate::utils::{get_account_info_or_default, get_key_or_none};
use anchor_lang::prelude::*;
use anchor_lang::{Accounts, Key, ToAccountInfo};
use anchor_spl::token::Mint;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;

#[derive(Accounts)]
pub struct ValidateReferrer<'info> {
    /// CHECK: The buddylink program
    #[account(executable, address = BL_PROGRAM_ID)]
    pub buddy_link_program: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    #[account()]
    pub authority: SystemAccount<'info>,

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
    #[account()]
    pub referee_member: UncheckedAccount<'info>,

    /// CHECK: Referrer member (account of the referrer within your organization) (if you want to validate the referrer of the current referee).
    #[account()]
    pub referrer_member: Option<UncheckedAccount<'info>>,
    /// CHECK: Referrer treasury (treasury that owns the referrer member ) (if you want to validate the referrer of the current referee).
    #[account()]
    pub referrer_treasury: Option<UncheckedAccount<'info>>,
    /// CHECK: Referrer treasury for reward (treasury that is linked to the current mint, could be the same as above) (if you want to validate the referrer of the current referee).
    #[account()]
    pub referrer_treasury_for_reward: Option<UncheckedAccount<'info>>,
    /// CHECK: Token account linked to the mint (if you want to validate the referral tree with a specific mint)
    #[account()]
    pub referrer_token_account: Option<UncheckedAccount<'info>>,

    #[account()]
    pub mint: Option<Account<'info, Mint>>,
}

pub fn validate_referrer(ctx: Context<ValidateReferrer>) -> ProgramResult {
    let instruction = instruction::validate_referrer(
        ctx.accounts.payer.key(),
        ctx.accounts.authority.key(),
        get_key_or_none(&ctx.accounts.mint),
        get_key_or_none(&ctx.accounts.referrer_token_account),
        get_key_or_none(&ctx.accounts.referrer_member),
        get_key_or_none(&ctx.accounts.referrer_treasury),
        get_key_or_none(&ctx.accounts.referrer_treasury_for_reward),
        ctx.accounts.referee_buddy_profile.key(),
        ctx.accounts.referee_buddy.key(),
        ctx.accounts.referee_treasury.key(),
        ctx.accounts.referee_member.key(),
    );

    let default_account_info = ctx.accounts.buddy_link_program.to_account_info();

    let account_infos = [
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.referee_buddy_profile.to_account_info(),
        ctx.accounts.referee_buddy.to_account_info(),
        ctx.accounts.referee_treasury.to_account_info(),
        ctx.accounts.referee_member.to_account_info(),
        get_account_info_or_default(&ctx.accounts.referrer_member, &default_account_info),
        get_account_info_or_default(&ctx.accounts.referrer_treasury, &default_account_info),
        get_account_info_or_default(
            &ctx.accounts.referrer_treasury_for_reward,
            &default_account_info,
        ),
        get_account_info_or_default(&ctx.accounts.mint, &default_account_info),
        get_account_info_or_default(&ctx.accounts.referrer_token_account, &default_account_info),
    ];

    invoke(&instruction, &account_infos)
}
