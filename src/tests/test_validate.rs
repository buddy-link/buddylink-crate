use std::thread::sleep;
use std::time::Duration;
use anchor_lang::Id;
use anchor_spl::token::Token;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::{Keypair, Signature, Signer};
use lazy_static::lazy_static;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_sdk::pubkey;
use solana_sdk::transaction::Transaction;
use buddy_link::instruction::{GeneralTransferRewardArgs, transfer_checked_global_only_reward, transfer_checked_global_reward, transfer_secure_local_reward, transfer_unchecked_local_shared_reward, TransferUncheckedLocalSharedRewardArgs, validate_referrer};

lazy_static! {
    static ref CLIENT: RpcClient = RpcClient::new_with_commitment(
        "http://localhost:8899",
        CommitmentConfig::confirmed(),
    );
}

//Taken from the amman configs (copied from devnet)
#[allow(dead_code)]
const MASTER_ORG: Pubkey = pubkey!("CRY1kbdXSDkK2fHP8aPMCH3dtwtWBpdc81tyt8XVVunH");
#[allow(dead_code)]
const ORGANIZATION: Pubkey = pubkey!("Vygga65LjTWs7kJR9Z7JbCpBpcuc7Tn8LkjXoxHomRQ");
const MINT: Pubkey = pubkey!("3Q6dz8cLd4BW1kyuGyUaS7qhTtFP7tGS55Y7fybCUfNy");
#[allow(dead_code)]
const REFERRER_AUTHORITY: Pubkey = pubkey!("DK1FtDDy2RkydDuhprUNKmsyVv8JQb5YDrUZe3GB8ZFc");
#[allow(dead_code)]
const REFERRER_GLOBAL_BUDDY: Pubkey = pubkey!("4jHbHkwjJoZgDBsx774LAmmqxPuGwk65SVdV6yr5Xjsm");
const REFERRER_TREASURY: Pubkey = pubkey!("AsY9QzsVwu6KX9N5Yy85M5jaMYitPYrAC7vuCY6A3YKf");
const REFERRER_ATA: Pubkey = pubkey!("C4yA9kJKohWhmGKAMGhJWRB827UdR6aVRUu82mGnmNwV");
const REFERRER_MEMBER: Pubkey = pubkey!("GZ3oVbxW1LY26LsbZKJEqbv7AXiJGsMtW9emm4wdexN9");
const REFEREE_AUTHORITY: Pubkey = pubkey!("HFnGHHTEKdggiHVFYEs1VAKKmjPvoD31HQsApkZqHqEx");
const REFEREE_GLOBAL_BUDDY: Pubkey = pubkey!("DLCAgJho2Fm3g2SEWHzfiuJdLMRhqUVDWtssqhCeCdYr");
const REFEREE_TREASURY: Pubkey = pubkey!("CMLckMKGfa5MTeovcfr9Rhgg31rFcAGS9ZKMXigaHMWJ");
const REFEREE_ATA: Pubkey = pubkey!("C2LZp5DNf6JsjEWoiQiS1jXQPopi3y4K2H7zN6App7mH");
const REFEREE_MEMBER: Pubkey = pubkey!("9xNqfpwRUEyqpURcgNZWFUrurrSRtdQTYpUQGbpJXWpp");

fn airdrop(admin: &Keypair) {
    CLIENT.request_airdrop(&admin.pubkey(), 2_000_000_000).unwrap();

    sleep(Duration::from_secs(1));
}

fn execute_txn(admin: &Keypair, instruction: Instruction) -> Signature {
    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&admin.pubkey()),
    );

    transaction.sign(&[admin], CLIENT.get_latest_blockhash().unwrap());

    let signature = CLIENT.send_and_confirm_transaction_with_spinner_and_config(
        &transaction,
        CommitmentConfig::confirmed(),
        RpcSendTransactionConfig {
            skip_preflight: true,
            preflight_commitment: None,
            encoding: None,
            max_retries: None,
            min_context_slot: None,
    }).unwrap();

    sleep(Duration::from_secs(1));

    signature
}

#[test]
fn test_validate_referrer() {
    let admin: Keypair = Keypair::new();

    airdrop(&admin);

    let instruction = validate_referrer(
        admin.pubkey(),
        REFEREE_AUTHORITY,
        Some(MINT),
        Some(REFERRER_ATA),
        Some(REFERRER_MEMBER),
        Some(REFERRER_TREASURY),
        Some(REFERRER_TREASURY),
        REFEREE_GLOBAL_BUDDY,
        REFEREE_GLOBAL_BUDDY,
        REFEREE_TREASURY,
        REFEREE_MEMBER
    );

    let signature = execute_txn(&admin, instruction);

    let result = CLIENT.get_signature_status(&signature).unwrap();

    assert_eq!(result.unwrap(), Ok(()));
}

#[test]
fn test_transfer_unchecked_local_shared_reward() {
    let admin: Keypair = Keypair::new();

    airdrop(&admin);

    let instruction = transfer_unchecked_local_shared_reward(
        admin.pubkey(),
        Some(solana_program::system_program::id()),
        None,
        None,
        None,
        &[REFERRER_TREASURY, REFERRER_MEMBER],
        &TransferUncheckedLocalSharedRewardArgs {
            total_amount: 10,
            shares_in_bps: vec![10_000],
            members_included: true,
        }
    );

    let signature = execute_txn(&admin, instruction);

    let result = CLIENT.get_signature_status(&signature).unwrap();

    assert_eq!(result.unwrap(), Ok(()));
}

#[test]
fn test_transfer_secure_local_reward() {
    let admin: Keypair = Keypair::new();

    airdrop(&admin);

    let instruction = transfer_secure_local_reward(
        REFEREE_AUTHORITY,
        MINT,
        Token::id(),
        REFEREE_ATA,
        REFERRER_ATA,
        REFERRER_MEMBER,
        REFERRER_TREASURY,
        REFERRER_TREASURY,
        REFEREE_GLOBAL_BUDDY,
        REFEREE_GLOBAL_BUDDY,
        REFEREE_TREASURY,
        REFEREE_MEMBER,
        &GeneralTransferRewardArgs {
            amount: 10,
        }
    );

    let signature = execute_txn(&admin, instruction);

    let result = CLIENT.get_signature_status(&signature).unwrap();

    assert_eq!(result.unwrap(), Ok(()));
}

#[test]
fn test_transfer_checked_global_reward() {
    let admin: Keypair = Keypair::new();

    airdrop(&admin);

    let instruction = transfer_checked_global_reward(
        admin.pubkey(),
        MINT,
        Token::id(),
        REFEREE_ATA,
        REFERRER_ATA,
        Some(REFERRER_MEMBER),
        REFERRER_TREASURY,
        REFERRER_TREASURY,
        REFEREE_MEMBER,
        Some(REFERRER_TREASURY),
        Some(REFERRER_ATA),
        &GeneralTransferRewardArgs {
            amount: 10,
        }
    );

    let signature = execute_txn(&admin, instruction);

    let result = CLIENT.get_signature_status(&signature).unwrap();

    assert_eq!(result.unwrap(), Ok(()));
}

#[test]
fn test_transfer_checked_global_only_reward() {
    let admin: Keypair = Keypair::new();

    airdrop(&admin);

    let instruction = transfer_checked_global_only_reward(
        admin.pubkey(),
        Some(solana_program::system_program::id()),
        None,
        None,
        None,
        None,
        REFERRER_TREASURY,
        REFERRER_TREASURY,
        REFEREE_GLOBAL_BUDDY,
        REFEREE_GLOBAL_BUDDY,
        &GeneralTransferRewardArgs {
            amount: 10,
        }
    );

    let signature = execute_txn(&admin, instruction);

    let result = CLIENT.get_signature_status(&signature).unwrap();

    assert_eq!(result.unwrap(), Ok(()));
}