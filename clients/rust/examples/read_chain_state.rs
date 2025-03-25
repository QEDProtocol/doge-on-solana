use qed_solana_doge_ibc_v3_program_client::
    doge_chain_state::QEDDogeChainState
;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program_test::tokio;
use solana_sdk::
    pubkey::Pubkey
;
use zerocopy::FromBytes;

fn hex_reversed(data: &[u8]) -> String {
    let mut copy = data.to_vec();
    copy.reverse();
    hex::encode(&copy)
}

async fn get_doge_solana_on_chain_data() -> Result<(), Box<dyn std::error::Error>> {
    let client = RpcClient::new("http://127.0.0.1:8899".to_string());

    let program_acc_data = client
        .get_account_data(&Pubkey::from_str_const(
            "BnySK72urJNHzjnZWRpwq7bi6xCkcinpBH7MxQMWubL",
        ))
        .await?;

    let doge_state = QEDDogeChainState::ref_from_bytes(&program_acc_data[33..]).unwrap();

    // all this data is on chain for dApps to use =)
    println!("tip_height: {}", doge_state.get_tip_block_number());
    println!(
        "tip_hash: {}",
        hex_reversed(&doge_state.get_tip_block_hash())
    );

    println!(
        "finalized_height: {}",
        doge_state.get_finalized_block_number()
    );
    println!(
        "finalized_hash: {}",
        hex_reversed(&doge_state.get_finalized_block_hash())
    );

    let tip_number = doge_state.get_tip_block_number();

    (0..32)
        .map(|x| tip_number - (31 - x))
        .map(|x| (x, doge_state.block_data_tracker.get_record(x).unwrap()))
        .for_each(|(block_number, record)| {
            println!("================ Cache Block On-Chain Data ================");
            println!(
                "block_number: {}, timestamp: {}, difficulty bits: {}",
                block_number, record.timestamp, record.bits
            );
            println!("block_hash: {}", hex_reversed(&record.block_hash));
            // use a merkle proof to prove the existence of any transaction in the block
            println!(
                "block_tx_tree_merkle_root: {}",
                hex::encode(&record.tx_tree_merkle_root)
            );

            // use a merkle proof to prove the existence of any historical block
            println!(
                "global_block_tree_merkle_root: {}",
                hex::encode(&record.block_hash_tree_root)
            );
            println!("===========================================================");
        });

    Ok(())
}


#[tokio::main]
async fn main() {
    get_doge_solana_on_chain_data().await.unwrap();
}
