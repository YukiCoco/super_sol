use anyhow::Result;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::Keypair,
};
use spl_associated_token_account::get_associated_token_address;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use bincode;
use solana_sdk::signature::Signer;

// 定义程序ID和类型
mod super_program {
    use solana_sdk::declare_id;
    declare_id!("super4XGGb7KWorPuoSNVQDHAVQjWzTpqcoRS86d9Us");
}

// BuyTokenArgs 结构体定义
#[derive(Serialize, Deserialize)]
pub struct BuyTokenArgs {
    pub buy_amount: u64,
    pub max_pay: u64,
    pub donate_rate: u32,
}

// SellTokenArgs 结构体定义
#[derive(Serialize, Deserialize)]
pub struct SellTokenArgs {
    pub sell_amount: u64,
    pub min_receive: u64,
}

// 查找账户工具函数
pub fn find_config_account_pubkey() -> Pubkey {
    // 根据您的项目逻辑实现
    Pubkey::from_str("7ci3rZLKS92bdvVBD9qGH8qpRk3o975TxViLAFRh7hUx").unwrap()
}

pub fn find_market_account_pubkey() -> Pubkey {
    // 根据您的项目逻辑实现
    Pubkey::from_str("Bbn98EBGWp1yZZ5NRpVCQBXUnYVQ8XQuz1yuer4mXDwE").unwrap()
}

pub fn find_native_vault_account_pubkey() -> Pubkey {
    // 根据您的项目逻辑实现
    Pubkey::from_str("8vRBJ23q3uAxwkj56BJe8td7bguYdSkJ7UVtuhxLzDBG").unwrap()
}

pub fn find_token_vault_account_pubkey() -> Pubkey {
    // 根据您的项目逻辑实现
    Pubkey::from_str("8eVmut2ripdnnTPwapuWYJBhVsq5nZbN87i9MRiJSsxW").unwrap()
}

pub fn find_token_mint_pubkey() -> Pubkey {
    // 根据您的项目逻辑实现
    Pubkey::from_str("supruCAzKLHdtZCHvCWLauYQUJUvmVJXHJJb2zRxUMv").unwrap()
}

pub fn get_fee_recipient_pubkey() -> Pubkey {
    // 根据您的项目逻辑实现
    Pubkey::from_str("7PG3b8wKLxb2tK36imsBwagnQWRc7axTcfCG61upQzoL").unwrap_or_default()
}

pub fn get_community_vault_pubkey() -> Pubkey {
    // 根据您的项目逻辑实现
    Pubkey::from_str("A985r8W5VjhYtaaCf8PAcX5vXJLYb11cUForjoqK3J1K").unwrap_or_default()
}

// impl buy token ix
pub fn buy_token_ix(payer: &Keypair, args: BuyTokenArgs) -> Result<Instruction> {
    // 获取需要的公钥
    let config = find_config_account_pubkey();
    let market = find_market_account_pubkey();
    let native_vault = find_native_vault_account_pubkey();
    let token_vault = find_token_vault_account_pubkey();
    
    // 获取其他账户地址
    let fee_recipient = get_fee_recipient_pubkey();
    let community_vault = get_community_vault_pubkey();
    
    // 使用市场关联的代币mint获取正确的token_recipient地址
    // 这里假设市场有一个关联的代币mint
    let token_mint = find_token_mint_pubkey();
    let token_recipient = get_associated_token_address(&payer.pubkey(), &token_mint);
    
    // 构建账户元数据
    let accounts = vec![
        solana_sdk::instruction::AccountMeta::new_readonly(config, false),
        solana_sdk::instruction::AccountMeta::new(market, false),
        solana_sdk::instruction::AccountMeta::new(native_vault, false),
        solana_sdk::instruction::AccountMeta::new(fee_recipient, false),
        solana_sdk::instruction::AccountMeta::new(token_vault, false),
        solana_sdk::instruction::AccountMeta::new(community_vault, false),
        solana_sdk::instruction::AccountMeta::new(token_recipient, false),
        solana_sdk::instruction::AccountMeta::new(payer.pubkey(), true),
        solana_sdk::instruction::AccountMeta::new(spl_token::ID, false),
        solana_sdk::instruction::AccountMeta::new(solana_sdk::system_program::ID, false),
    ];

    // 构建指令数据
    // buy_token 的指令标识符
    let mut data = vec![
        138,
        127,
        14,
        91,
        38,
        87,
        115,
        105
    ];
    
    // 将参数序列化并添加到数据中
    let args_data = bincode::serialize(&args)?;
    data.extend_from_slice(&args_data);

    Ok(Instruction {
        program_id: super_program::ID,
        accounts,
        data,
    })
}

// impl sell token ix
pub fn sell_token_ix(payer: &Keypair, args: SellTokenArgs) -> Result<Instruction> {
    // 获取需要的公钥
    let config = find_config_account_pubkey();
    let market = find_market_account_pubkey();
    let native_vault = find_native_vault_account_pubkey();
    let token_vault = find_token_vault_account_pubkey();
    
    // 获取其他账户地址
    let fee_recipient = get_fee_recipient_pubkey();
    
    // 使用市场关联的代币mint获取正确的token_payer地址
    let token_mint = find_token_mint_pubkey();
    let token_payer = get_associated_token_address(&payer.pubkey(), &token_mint);
    
    // 获取native_recipient地址（接收SOL的地址，通常就是payer本身）
    let native_recipient = payer.pubkey();
    
    // 构建账户元数据
    let accounts = vec![
        solana_sdk::instruction::AccountMeta::new_readonly(config, false),
        solana_sdk::instruction::AccountMeta::new(market, false),
        solana_sdk::instruction::AccountMeta::new(native_vault, false),
        solana_sdk::instruction::AccountMeta::new(fee_recipient, false),
        solana_sdk::instruction::AccountMeta::new(token_vault, false),
        solana_sdk::instruction::AccountMeta::new(native_recipient, false),
        solana_sdk::instruction::AccountMeta::new(token_payer, false),
        solana_sdk::instruction::AccountMeta::new(payer.pubkey(), true),
        solana_sdk::instruction::AccountMeta::new(spl_token::ID, false),
        solana_sdk::instruction::AccountMeta::new(solana_sdk::system_program::ID, false),
    ];

    // 构建指令数据
    // sell_token 的指令标识符
    let mut data = vec![
        109,
        61,
        40,
        187,
        230,
        176,
        135,
        174
    ];
    
    // 将参数序列化并添加到数据中
    let args_data = bincode::serialize(&args)?;
    data.extend_from_slice(&args_data);

    Ok(Instruction {
        program_id: super_program::ID,
        accounts,
        data,
    })
}