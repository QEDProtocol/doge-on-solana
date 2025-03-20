use std::sync::Arc;

use solana_client::{connection_cache::ConnectionCache, nonblocking::{rpc_client::RpcClient, tpu_client::TpuClient}, send_and_confirm_transactions_in_parallel::{send_and_confirm_transactions_in_parallel, SendAndConfirmConfig}, tpu_client::TpuClientConfig};
use solana_quic_client::{QuicConfig, QuicConnectionManager, QuicPool};
use solana_sdk::{bpf_loader_upgradeable::UpgradeableLoaderState, instruction::{Instruction, InstructionError}, message::Message, packet::PACKET_DATA_SIZE, pubkey::Pubkey, signature::Signature, signer::Signer, system_instruction, system_program, transaction::Transaction};

use super::{blob_buffer::QBLOB_PROGRAM_ID, blob_instruction::{buffer_initialize_buffer_instruction, buffer_write_instruction}};

pub struct SolClient {
    pub tpu: TpuClient<QuicPool, QuicConnectionManager, QuicConfig>,
    pub rpc: Arc<RpcClient>,
}

impl SolClient {
    pub async fn new_from_urls(rpc_url: &str, websocket_url: &str) -> anyhow::Result<Self> {
        let rpc = Arc::new(RpcClient::new(rpc_url.to_string()));
        let tpu = TpuClient::new_with_connection_cache(
            rpc.clone(),
            websocket_url,
            solana_client::tpu_client::TpuClientConfig::default(),
            match ConnectionCache::new_quic("connection_cache_cli_program_quic", 1) {
                ConnectionCache::Quic(c) => c,
                ConnectionCache::Udp(_) => anyhow::bail!("should never be udp"),
            },
        ).await?;
        Ok(Self { tpu, rpc })

        
    }

    pub async fn new_from_rpc(rpc: Arc<RpcClient>, websocket_url: &str) -> anyhow::Result<Self> {
        let rpc = rpc.clone();
        let tpu = TpuClient::new_with_connection_cache(
            rpc.clone(),
            websocket_url,
            solana_client::tpu_client::TpuClientConfig::default(),
            match ConnectionCache::new_quic("connection_cache_cli_program_quic", 1) {
                ConnectionCache::Quic(c) => c,
                ConnectionCache::Udp(_) => anyhow::bail!("should never be udp"),
            },
        ).await?;
        Ok(Self { tpu, rpc })

        
    }
}

pub fn create_buffer(
    payer_address: &Pubkey,
    buffer_address: &Pubkey,
    authority_address: &Pubkey,
    lamports: u64,
    program_len: usize,
) -> Result<Vec<Instruction>, InstructionError> {
    Ok(vec![
        system_instruction::create_account(
            payer_address,
            buffer_address,
            lamports,
            UpgradeableLoaderState::size_of_buffer(program_len) as u64,
            &QBLOB_PROGRAM_ID,
        ),
        buffer_initialize_buffer_instruction(
            buffer_address,
            authority_address,
        ),
    ])
}

pub fn calculate_max_chunk_size<F>(create_msg: &F) -> usize
where
    F: Fn(u32, Vec<u8>) -> Message,
{
    let baseline_msg = create_msg(0, Vec::new());
    let tx_size = bincode::serialized_size(&Transaction {
        signatures: vec![
            Signature::default();
            baseline_msg.header.num_required_signatures as usize
        ],
        message: baseline_msg,
    })
    .unwrap() as usize;
    // add 1 byte buffer to account for shortvec encoding
    PACKET_DATA_SIZE.saturating_sub(tx_size).saturating_sub(1)
}


#[allow(clippy::too_many_arguments)]
async fn send_deploy_messages(
    rpc_client: Arc<RpcClient>,
    initial_message: Option<Message>,
    mut write_messages: Vec<Message>,
    fee_payer_signer: &dyn Signer,
    initial_signer: Option<&dyn Signer>,
    write_signer: Option<&dyn Signer>,
    max_sign_attempts: usize,
    websocket_url: &str,
) -> Result<Option<Signature>, Box<dyn std::error::Error>> {
    if let Some(mut message) = initial_message {
        if let Some(initial_signer) = initial_signer {
            println!("Preparing the required accounts");
            let mut initial_transaction = Transaction::new_unsigned(message.clone());
            let blockhash = rpc_client.get_latest_blockhash().await?;

            // Most of the initial_transaction combinations require both the fee-payer and new program
            // account to sign the transaction. One (transfer) only requires the fee-payer signature.
            // This check is to ensure signing does not fail on a KeypairPubkeyMismatch error from an
            // extraneous signature.
            if message.header.num_required_signatures == 2 {
                initial_transaction.try_sign(&[fee_payer_signer, initial_signer], blockhash)?;
            } else {
                initial_transaction.try_sign(&[fee_payer_signer], blockhash)?;
            }
            let result = rpc_client.send_and_confirm_transaction_with_spinner(&initial_transaction).await?;
        } else {
            return Err("Buffer account not created yet, must provide a key pair".into());
        }
    }

    if !write_messages.is_empty() {
        if let Some(write_signer) = write_signer {
            println!("Writing program data");

            let connection_cache = 
                ConnectionCache::new_quic("connection_cache_cli_program_quic", 1);
            let transaction_errors = match connection_cache {
                ConnectionCache::Udp(cache) => TpuClient::new_with_connection_cache(
                    rpc_client.clone(),
                    &websocket_url,
                    TpuClientConfig::default(),
                    cache,
                ).await?
                .send_and_confirm_messages_with_spinner(
                    &write_messages,
                    &[fee_payer_signer, write_signer],
                ).await,
                ConnectionCache::Quic(cache) => {
                    let tpu_client_fut = solana_client::nonblocking::tpu_client::TpuClient::new_with_connection_cache(
                        rpc_client.clone(),
                        websocket_url,
                        solana_client::tpu_client::TpuClientConfig::default(),
                        cache,
                    ).await?;
                    let tpu_client = Some(tpu_client_fut);

                    let res = send_and_confirm_transactions_in_parallel(
                        rpc_client.clone(),
                        tpu_client,
                        &write_messages,
                        &[fee_payer_signer, write_signer],
                        SendAndConfirmConfig {
                            resign_txs_count: Some(max_sign_attempts),
                            with_spinner: true,
                        },
                    ).await?;
                    Ok(res)
                },
            }
            .map_err(|err| format!("Data writes to account failed: {err}"))?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

            if !transaction_errors.is_empty() {
                for transaction_error in &transaction_errors {
                    anyhow::anyhow!("{:?}", transaction_error);
                }
                return Err(
                    format!("{} write transactions failed", transaction_errors.len()).into(),
                );
            }
        }
    }
    Ok(None)
}
async fn do_process_write_buffer(
    rpc_client: Arc<RpcClient>,
    program_data: &[u8], // can be empty, hence we have program_len
    program_len: usize,
    min_rent_exempt_program_data_balance: u64,
    fee_payer_signer: &dyn Signer,
    buffer_signer: Option<&dyn Signer>,
    buffer_pubkey: &Pubkey,
    buffer_program_data: Option<Vec<u8>>,
    buffer_authority_signer: &dyn Signer,
    skip_fee_check: bool,
    compute_unit_price: Option<u64>,
    max_sign_attempts: usize,
    websocket_url: &str,
) -> anyhow::Result<Pubkey> {
    let blockhash = rpc_client.get_latest_blockhash().await?;
    

    let (initial_instructions, balance_needed, buffer_program_data) =
        if let Some(buffer_program_data) = buffer_program_data {
            (vec![], 0, buffer_program_data)
        } else {
            (
                create_buffer(
                    &fee_payer_signer.pubkey(),
                    buffer_pubkey,
                    &buffer_authority_signer.pubkey(),
                    min_rent_exempt_program_data_balance,
                    program_len,
                )?,
                min_rent_exempt_program_data_balance,
                vec![0; program_len],
            )
        };

    let initial_message = if !initial_instructions.is_empty() {
        Some(Message::new_with_blockhash(&initial_instructions,
            Some(&fee_payer_signer.pubkey()),
            &blockhash,
        ))
    } else {
        None
    };

    // Create and add write messages
    let create_msg = |offset: u32, bytes: Vec<u8>| {
        let instruction = buffer_write_instruction(
            buffer_pubkey,
            &buffer_authority_signer.pubkey(),
            offset,
            bytes,
        );

        let instructions = vec![instruction];
        Message::new_with_blockhash(&instructions, Some(&fee_payer_signer.pubkey()), &blockhash)
    };

    let mut write_messages = vec![];
    let chunk_size = calculate_max_chunk_size(&create_msg);
    for (chunk, i) in program_data.chunks(chunk_size).zip(0usize..) {
        let offset = i.saturating_mul(chunk_size);
        if chunk != &buffer_program_data[offset..offset.saturating_add(chunk.len())] {
            write_messages.push(create_msg(offset as u32, chunk.to_vec()));
        }
    }
    /* 

    let _final_tx_sig = send_deploy_messages(
        rpc_client,
        initial_message,
        write_messages,
        fee_payer_signer,
        buffer_signer,
        Some(buffer_authority_signer),
        websocket_url,
    ).await?;*/

    Ok(*buffer_pubkey)
}

