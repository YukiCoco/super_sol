use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_sdk::message::Message;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::system_instruction::transfer;
use solana_sdk::transaction::Transaction;
use super_sol::buy_token_ix;
use super_sol::sell_token_ix;
use super_sol::BuyTokenArgs;
use super_sol::SellTokenArgs;
use spl_associated_token_account::instruction::create_associated_token_account_idempotent;
use solana_sdk::signature::Signer;
use clap::Parser;
use rand::Rng;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Solana RPC URL with API key
    #[arg(short, long)]
    rpc_url: String,

    /// Alternative: Base58 encoded private key
    #[arg(short, long)]
    private_key_base58:String,

    #[arg(short, long)]
    buy_amount: u64,

    #[arg(short, long, default_value_t = 1_000_000_000.0)]
    max_sol: f64,

    #[arg(short, long)]
    jito_url: String,
}

fn get_random_tip_receiver() -> Pubkey {
    let tip_receivers = ["96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5", "Cw8CFyM9FkoMi7K7Crf6HNQqf4uEMzpKw6QNghXLvLkY", "DttWaMuVvTiduZRnguLF7jNxTgiMBZ1hyAumKUiL2KRL", "ADaUMid9yfUytqMBgopwjb2DTLSokTSzL1zt6iGPaS49", "DfXygSm4jCyNCybVYYK6DwvWqjKee8pbDmJGcLWNDXjh", "ADuUkR4vqLUMWXxW9gh6D6L8pMSawimctcNZ5pGwDcEt", "HFqU5x63VTqvQss8hp11i4wVV8bD44PvwucfZ2bU7gRe", "3AVi9Tg9Uo68tJfuvoKvqKNWKkC5wPdSSdeBnizKZ6jT"];
    
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..tip_receivers.len());
    Pubkey::from_str(tip_receivers[random_index]).unwrap()
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    let payer = Keypair::from_base58_string(&cli.private_key_base58); // 从 cli 读取
    let rpc = RpcClient::new(&cli.rpc_url); // 从 cli 读
    let buy_args = BuyTokenArgs {
        buy_amount: cli.buy_amount * 10_u64.pow(6), // 购买的代币数量，例如1亿（取决于精度） // 从 cli 读取
        max_pay: (cli.max_sol * LAMPORTS_PER_SOL as f64) as u64,    // 最多支付的SOL数量，例如1.5 SOL
        donate_rate: 0,            // 捐赠率，例如0表示不捐赠
    };
    let sell_args = SellTokenArgs {
        sell_amount: cli.buy_amount * 10_u64.pow(6),  // 出售的代币数量，例如5千万 // 从 cli 读取
        min_receive: 0,  // 最少接收的SOL数量，例如0.6 SOL
    };
    let send_mint = Pubkey::from_str("supruCAzKLHdtZCHvCWLauYQUJUvmVJXHJJb2zRxUMv").unwrap();
    let create_token_ix = create_associated_token_account_idempotent(&payer.pubkey(), &payer.pubkey(), &send_mint, &spl_token::id());
    let buy_ix = buy_token_ix(&payer, buy_args).unwrap();
    let sell_ix = sell_token_ix(&payer, sell_args).unwrap();
    let lamports = 1000;
    let tip_ix = transfer(&payer.pubkey(), &get_random_tip_receiver(), lamports);

    let message = Message::new(
        &[create_token_ix, buy_ix, sell_ix, tip_ix],
        Some(&payer.pubkey()),
    );

    let recent_blockhash = rpc.get_latest_blockhash().unwrap();
    let mut transaction = Transaction::new(&[&payer], message, recent_blockhash);
    transaction.sign(&[&payer], recent_blockhash);

    // let signature = rpc.send_transaction(&transaction).unwrap();
    let client = reqwest::Client::new();
    let signature = send_bundle_via_jsonrpc(&client, &[transaction], &cli.jito_url).await.unwrap();
    println!("Signature: {:?}", signature);
}

pub async fn send_bundle_via_jsonrpc(client: &reqwest::Client, transactions: &[Transaction], endpoint: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // 构建 reqwest client

    let url = format!("{}{}", endpoint, "/api/v1/bundles");

    // 设置 headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE, 
        "application/json".parse().unwrap()
    );

    // 将交易序列化为 base64 字符串
    let txs_base64: Vec<String> = transactions
        .iter()
        .map(|tx| {
            let serialized = bincode::serialize(tx).expect("Failed to serialize transaction");
            base64::encode(&serialized)
        })
        .collect();

    // 构建 JSON-RPC 请求
    let json_request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "sendBundle",
        "params": [
            txs_base64,
            {
                "encoding": "base64"
            }
        ]
    });

    // 发送请求
    let response = client
        .post(url)
        .headers(headers)
        .json(&json_request)
        .send()
        .await?;

    // 获取响应文本
    let response_text = response.text().await?;
    
    Ok(response_text)
}
