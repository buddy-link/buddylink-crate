use anchor_lang::{Key, ToAccountInfo};
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;

pub fn get_key_or_none<T>(account: &Option<T>) -> Option<Pubkey>
where
    T: Key,
{
    if let Some(account) = account {
        Some(account.key())
    } else {
        None
    }
}

pub fn get_account_info_or_default<'info, T>(
    account: &Option<T>,
    default_account_info: &AccountInfo<'info>,
) -> AccountInfo<'info>
where
    T: ToAccountInfo<'info>,
{
    if let Some(account) = account {
        account.to_account_info()
    } else {
        default_account_info.to_account_info()
    }
}
