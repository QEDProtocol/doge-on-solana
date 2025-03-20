use std::sync::Arc;

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::send_and_confirm_transactions_in_parallel::{
    self, send_and_confirm_transactions_in_parallel, SendAndConfirmConfig,
};
use solana_sdk::hash::Hash;
use solana_sdk::message::Message;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction::create_account,
    transaction::Transaction,
};

use super::blob_instruction::{
    buffer_close_instruction, buffer_initialize_buffer_instruction, buffer_write_instruction,
};
use super::client::SolClient;

pub const QBLOB_PROGRAM_ID: Pubkey =
    solana_sdk::pubkey!("CzqeK66uHUYbauvaLJ3sfQd9JmiMqvvPvAudpZmhr6xF");

pub struct BlobBufferHelper {
    pub buffer: Pubkey,
    pub new_account_signer: Keypair,
}

impl BlobBufferHelper {
    pub async fn new(
        client: &RpcClient,
        payer: &Keypair,
        buffer_size: usize,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let new_account_signer = Keypair::new();
        let tx =
            initialize_buffer_for_authority_tx(client, buffer_size, payer, &new_account_signer)
                .await?;
        let _signature = client.send_and_confirm_transaction(&tx).await?;
        let buffer = new_account_signer.pubkey();
        Ok(Self {
            buffer,
            new_account_signer,
        })
    }

    pub async fn get_write_transactions(
        &self,
        client: &RpcClient,
        payer: &Keypair,
        buffer_data: &[u8],
        chunk_length: usize,
    ) -> Result<Vec<Transaction>, Box<dyn std::error::Error>> {
        let recent_block_hash = client.get_latest_blockhash().await.unwrap();
        let mut transactions = Vec::with_capacity(buffer_data.len() / chunk_length + 1);

        let num_chunks = buffer_data.len() / chunk_length;
        for i in 0..num_chunks {
            let tx = write_data_for_authority_tx(
                &payer,
                &self.new_account_signer,
                &buffer_data,
                i * chunk_length,
                chunk_length,
                recent_block_hash,
            )?;
            transactions.push(tx);
        }
        if buffer_data.len() % chunk_length != 0 {
            let tx = write_data_for_authority_tx(
                &payer,
                &self.new_account_signer,
                &buffer_data,
                num_chunks * chunk_length,
                buffer_data.len() % chunk_length,
                recent_block_hash,
            )?;
            transactions.push(tx);
        }
        Ok(transactions)
    }

    pub fn get_write_messages(
        &self,
        payer: &Keypair,
        buffer_data: &[u8],
        chunk_length: usize,
    ) -> Result<Vec<Message>, Box<dyn std::error::Error>> {
        let mut messages = Vec::with_capacity(buffer_data.len() / chunk_length + 1);

        let num_chunks = buffer_data.len() / chunk_length;
        for i in 0..num_chunks {
            let msg = write_data_for_authority_msg(
                &payer,
                &self.new_account_signer,
                &buffer_data,
                i * chunk_length,
                chunk_length,
            )?;
            messages.push(msg);
        }
        if buffer_data.len() % chunk_length != 0 {
            let msg = write_data_for_authority_msg(
                &payer,
                &self.new_account_signer,
                &buffer_data,
                num_chunks * chunk_length,
                buffer_data.len() % chunk_length,
            )?;
            messages.push(msg);
        }
        Ok(messages)
    }
    pub async fn send_write_transactions_and_confirm(
        &self,
        client: &RpcClient,
        payer: &Keypair,
        buffer_data: &[u8],
        chunk_length: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let transactions = self
            .get_write_transactions(client, payer, buffer_data, chunk_length)
            .await?;

        let mut sigs = Vec::with_capacity(transactions.len());
        for tx in transactions {
            sigs.push(client.send_transaction(&tx).await?);
        }
        for sig in sigs {
            client.confirm_transaction(&sig).await?;
        }
        Ok(())
    }
    pub async fn send_write_transactions_and_confirm_v2(
        &self,
        client: Arc<RpcClient>,
        websocket_url: &str,
        payer: &Keypair,
        buffer_data: &[u8],
        chunk_length: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let messages = self.get_write_messages(payer, buffer_data, chunk_length)?;

        //send_and_confirm_transactions_in_parallel(rpc_client, tpu_client, messages, signers, config)

        let tpu = SolClient::new_from_rpc(client.clone(), websocket_url).await?.tpu;
        let results = send_and_confirm_transactions_in_parallel(
            client.clone(),
            Some(tpu),
            &messages,
            &[&payer, &self.new_account_signer],
            SendAndConfirmConfig {
                resign_txs_count: Some(5),
                with_spinner: true,
            },
        )
        .await?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
        if results.is_empty() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("error writing batch txs: {:?}", results[0]).into())
        }
    }
    pub async fn close_buffer(
        &self,
        client: &RpcClient,
        payer: &Keypair,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let close_ix = buffer_close_instruction(
            &self.new_account_signer.pubkey(),
            &payer.pubkey(),
            Some(payer.pubkey()),
            None,
        );
        let last_blockhash = client.get_latest_blockhash().await.unwrap();
        let tx = Transaction::new_signed_with_payer(
            &[close_ix],
            Some(&payer.pubkey()),
            &[&payer, &self.new_account_signer],
            last_blockhash,
        );
        let sig = client.send_transaction(&tx).await.unwrap();
        client.confirm_transaction(&sig).await.unwrap();
        Ok(())
    }
}
async fn initialize_buffer_for_authority_tx<S: Signer>(
    client: &RpcClient,
    buffer_size: usize,
    payer: &S,
    new_account: &S,
) -> Result<Transaction, Box<dyn std::error::Error>> {
    let lamports = client
        .get_minimum_balance_for_rent_exemption(buffer_size + 40)
        .await
        .unwrap();

    let create_acc_ix = create_account(
        &payer.pubkey(),
        &new_account.pubkey(),
        lamports,
        buffer_size as u64 + 40u64,
        &QBLOB_PROGRAM_ID,
    );

    let init_buffer_ix =
        buffer_initialize_buffer_instruction(&new_account.pubkey(), &payer.pubkey());
    let last_blockhash = client.get_latest_blockhash().await.unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[create_acc_ix, init_buffer_ix],
        Some(&payer.pubkey()),
        &[&payer, &new_account],
        last_blockhash,
    );

    Ok(tx)
}

fn write_data_for_authority_msg<S: Signer>(
    payer: &S,
    new_account: &Keypair,
    data: &[u8],
    offset: usize,
    length: usize,
) -> Result<Message, Box<dyn std::error::Error>> {
    println!("new_acc: {}", new_account.pubkey());
    let write_ix = buffer_write_instruction(
        &new_account.pubkey(),
        &payer.pubkey(),
        offset as u32,
        data[offset..offset + length].to_vec(),
    );
    println!("write_ix: {:?}", write_ix);
    Ok(Message::new(&[write_ix], Some(&payer.pubkey())))
}

fn write_data_for_authority_tx<S: Signer>(
    payer: &S,
    new_account: &Keypair,
    data: &[u8],
    offset: usize,
    length: usize,
    recent_blockhash: Hash,
) -> Result<Transaction, Box<dyn std::error::Error>> {
    println!("new_acc: {}", new_account.pubkey());
    let write_ix = buffer_write_instruction(
        &new_account.pubkey(),
        &payer.pubkey(),
        offset as u32,
        data[offset..offset + length].to_vec(),
    );
    println!("write_ix: {:?}", write_ix);

    let tx = Transaction::new_signed_with_payer(
        &[write_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    Ok(tx)
}
