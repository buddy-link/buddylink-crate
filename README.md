## Here is an example of how you would call the SDK for SPL

**Here the transfer is made by the signer directly, if your "from account" is owned by a PDA, then transfer seeds would
need to be provided.**

```rust
let cpi_context = CpiContext::new(
    buddy_link_program.to_account_info(),
    buddy_link::cpi::TransferCheckedGlobalOnlyReward {
        buddy_link_program: buddy_link_program.to_account_info(),
        authority: authority.to_account_info(),
        system_program: None,
        mint:  Some(mint.to_account_info()),
        token_program: Some(token_program.to_account_info()),
        from_token_account: Some(user_token_account.to_account_info()),
        referrer_token_account: Some(remaining_accounts[0].to_account_info()),
        global_referrer_treasury: referrer_treasury.to_account_info(),
        global_referrer_treasury_for_reward: referrer_treasury_for_reward.to_account_info(),
        referee_buddy_profile: buddy_profile.to_account_info(),
        referee_buddy: buddy.to_account_info(),
    },
);

buddy_link::cpi::transfer_checked_global_only_reward(
    cpi_context,
    amount_referral,
    & [],
);
```

## Here is an example of how you would call the SDK for SOL

**Here the transfer is made by the signer directly, if your "from account" is owned by a PDA, then transfer seeds would
need to be provided.**

```rust
let cpi_context = CpiContext::new(
    buddy_link_program.to_account_info(),
    buddy_link::cpi::TransferCheckedGlobalOnlyReward {
        buddy_link_program: buddy_link_program.to_account_info(),
        authority: authority.to_account_info(),
        system_program: Some(system_program.to_account_info()),
        mint: None,
        token_program: None,
        from_token_account: None,
        referrer_token_account: None,
        global_referrer_treasury: referrer_treasury.to_account_info(),
        global_referrer_treasury_for_reward: referrer_treasury_for_reward.to_account_info(),
        referee_buddy_profile: buddy_profile.to_account_info(),
        referee_buddy: buddy.to_account_info(),
    },
);

buddy_link::cpi::transfer_checked_global_only_reward(
    cpi_context,
    amount_referral,
    &[],
)
```

## How to test

1. yarn install
2. amman start
3. cargo test
