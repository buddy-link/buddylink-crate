use anchor_lang::{Key, ToAccountInfo};
use solana_program::account_info::AccountInfo;
use solana_program::hash::hash;
use solana_program::instruction::AccountMeta;
use solana_program::pubkey::Pubkey;
use crate::constants::BL_PROGRAM_ID;

pub fn get_key_or_none<T>(account: &Option<T>) -> Option<Pubkey>
    where
        T: Key,
{
    account.as_ref().map(|account| account.key())
}

pub fn get_account_info_or_default<'info, T>(
    account: &Option<T>,
    default_account_info: &AccountInfo<'info>,
) -> AccountInfo<'info>
    where
        T: ToAccountInfo<'info>,
{
    if let Some(account) = account {
        return account.to_account_info();
    }

    default_account_info.to_account_info()
}

pub fn get_account_meta_or_read_default(pubkey: &Option<Pubkey>) -> AccountMeta {
    if let Some(pubkey) = pubkey {
        return AccountMeta::new(*pubkey, false);
    }

    AccountMeta::new_readonly(BL_PROGRAM_ID, false)
}

pub fn get_instruction_name_data(instruction_name: &str) -> Vec<u8> {
    let mut instruction_data: Vec<u8> = vec![];

    instruction_data.extend_from_slice(&hash(format!("global:{}", instruction_name).as_bytes()).to_bytes()[..8]);

    instruction_data
}