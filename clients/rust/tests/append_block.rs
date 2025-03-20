use borsh::BorshDeserialize;
use doge_light_client::core_data::QDogeBlockHeader;
use qed_solana_doge_ibc_v3_program_client::{
    helpers::{
        blob_buffer::BlobBufferHelper, blob_instruction::buffer_close_instruction, instruction_data::
            gen_instruction_data_append_block_zkp
    },
    instructions::AppendBlockBuilder, types::QSQDogeBlockHeader,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program_test::tokio;
use solana_sdk::{compute_budget::ComputeBudgetInstruction, hash::Hash};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
const NEXT_BLOCK_5610384: [u8; 872] = hex_literal::hex!("040162007719c88d931b972b9e1ef5a584bc4e770eb07f80662319bc01cdaf7f76a6560ac0a45789cb0fc16f8115a098dd076cc598d16a38de02e8f7e2d3000791a36b6955d3c567b9c2001a000000000101000000010000000000000000000000000000000000000000000000000000000000000000000000ffffffff6400000003c2912b2cfabe6d6de7a351f02d5991db19770f7db926171d47a5c19b80d91db8f206597de8c036f520000000f09f909f092f4632506f6f6c2f6200000000000000000000000000000000000000000000000000000000000000000000000500a3f374fb450074fb02000000d6614d25000000001900000076a914dfd88711bf5d20f2ac7049ba28d053a89186ce9288ac0000000000000000260000006a24aa21a9edd1985cf5d0dba1ee9292c50afb17fe185d04984c3262f83a0b4babb26fc35a2ee08e9a43401e4ba42fc34caacd9e84466d0989855fe678df35f630d9a6000000000000000800000038d1e740ffa6926988ee0933c2336ae4fcb04f2b06bda5317b224bd6b49d16a1691756511e20a25342d1ecd8eb7b7eea0acb60d4cbad4b58288a45e243569b8069704ec7c30270d539702a188fb64f838508e62f15526ae54525033602e2ebd383a4760dd545d6afff57177e0df84cb9a6430c5e3ed14ba9b3a7e75a54b49090f33fbd2befddc8b880080a5a8b8538db4f37f1f1192ab2e0fbbab8c6785271dc416b14713a7f8b7e691a02b7ba68185bca083041f7452c0feda88da3fca5c70c2bd665d0245bf97caecd4714ced24d89bba9fb921e8d7605c8e8c248a5e50b6155791f46c8350259b097afc2eb55b8a1fab1f6347eb91a037f18df7ab197fae900000000050000000900000000000000000000000000000000000000000000000000000000000000710b1b3f2df407a200dd8321454575d9c8a35228a0cb34775aef1d44a656a7c81dcd82cb4cd3151a461d862e9dc5e2627037851f68f40baf79cdae5fae783b99358f1877087c6cf154f1328b50422df66aa2df0dabee93d1d4a0437f36ef025b916a0cff862746fa2b332601fcf6692fc2d1a339a399406a8835ec777695dc1f08000000140000207828ac122e36994fb9efa1f64cfa6805468643d89a8b07825451b48c7e85474a9452fd8ab0e90773ac6f10c474b6a9a755401cb21a1ad03ac01309d4a1b2796f53d3c567f1a735196f09bc09"
);
const PROOF_BLOCK_5610384: [u8; 260] = hex_literal::hex!("11b6a09d27bdea0dd15f8b1b7cff33794bd0abf97d2f69c77fb87b987714d48bfc4736a326881af6ebe0f9e984cbea9f7e66a662cfdd44e38b0d8a85858aa7c146f183fa1f157359576c31a9ddd3e5b7b9557a804366c5ffce7858b091f36899847aefd10022420057bedee1f57534f4ddaa04bff16326dcb9d6fb4223276ec10c95be9b268f1b3cce7a725dfefc6f64fe981c05b71c4b758ad4af3a7fb8c0b96ec40d761d56737b3ef6cd7d48baf6c0abfdf60d8aa9d7c167921e711a1eaffe007b4a6118b1b2acc73cef172d940e580efd2482ad77182721d0f529dc81262a3140db91045ae6b220d3d82fffb40973bea4ab04a257cbde0ceab84b48fabd1da59c11d2");
const POW_HASH_BLOCK_5610384: [u8; 32] =
    hex_literal::hex!("401e4ba42fc34caacd9e84466d0989855fe678df35f630d9a600000000000000");

struct TestContext {
    client: RpcClient,
    payer: Keypair,
    last_blockhash: Hash,
}
impl TestContext {
    async fn setup_account() -> anyhow::Result<Self> {
        let client = RpcClient::new("http://127.0.0.1:8899".to_string());
        let payer = Keypair::from_base58_string(
            "3aQSf1fzHm1ueckkgYz7JPfj6LGJ6nRaNMZ2gdRJ8TkF5xEBBwjatc2QAtKXhNpVPSS5Mxx4w4yLUzwKKHQkozQ5",
        );
        let sig = client
            .request_airdrop(&payer.pubkey(), 100_100_000_000_000)
            .await?;
        client.poll_for_signature_confirmation(&sig, 10).await?;
        let last_blockhash = client.get_latest_blockhash().await?;

        Ok(Self {
            client,
            payer,
            last_blockhash,
        })
    }
}

fn find_pda_ibc(payer: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"qed_doge_ibc", payer.as_ref()],
        &qed_solana_doge_ibc_v3_program_client::ID,
    )
}
async fn run_append_block_test() -> Result<(), Box<dyn std::error::Error>> {
    let tctx = TestContext::setup_account().await?;
    println!("setup test context");

    let doge_header = QDogeBlockHeader::try_from_slice(&NEXT_BLOCK_5610384)?;

    let append_block_ix_blob = gen_instruction_data_append_block_zkp(
        5610384,
        &doge_header,
        &PROOF_BLOCK_5610384,
        POW_HASH_BLOCK_5610384,
    );

    println!(
        "append_block_ix_blob: {}",
        hex::encode(&append_block_ix_blob)
    );

    let buffer_helper =
        BlobBufferHelper::new(&tctx.client, &tctx.payer, append_block_ix_blob.len()).await?;

    println!(
        "new_account_signer: {:?}",
        buffer_helper.new_account_signer.pubkey()
    );
    buffer_helper
        .send_write_transactions_and_confirm(&tctx.client, &tctx.payer, &append_block_ix_blob, 600)
        .await?;

    let account = tctx
        .client
        .get_account_data(&buffer_helper.new_account_signer.pubkey())
        .await?;
    println!("account_data: {}", hex::encode(account));

    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    let account = tctx
        .client
        .get_account_data(&buffer_helper.new_account_signer.pubkey())
        .await?;
    println!("account_data: {}", hex::encode(account));

    let qed_doge_ibc_address =
        Pubkey::from_str_const("3qwcETtnFYjqxJ8GMdZyACnMwddrkJb4YABzm7cjiGvf");

    let ix = AppendBlockBuilder::new()
        .qed_doge_ibc(qed_doge_ibc_address)
        .authority(tctx.payer.pubkey())
        .block_header(QSQDogeBlockHeader::default())
        .block_number(1)
        .pow_hash_mode(0)
        .instruction();

    let dummy_ix = Instruction {
        program_id: ix.program_id,
        accounts: [
            ix.accounts,
            vec![AccountMeta::new(
                buffer_helper.new_account_signer.pubkey(),
                false,
            )],
        ]
        .concat(),
        data: Vec::new(),
    };
    let last_blockhash = tctx.client.get_latest_blockhash().await.unwrap();

    let set_compute_budget_ix = ComputeBudgetInstruction::set_compute_unit_limit(1000_000);

    let tx = Transaction::new_signed_with_payer(
        &[set_compute_budget_ix, dummy_ix],
        Some(&tctx.payer.pubkey()),
        &[&tctx.payer],
        last_blockhash,
    );

    println!("prepared tx");
    let sig = tctx.client.send_and_confirm_transaction(&tx).await?;

    println!("sent append block tx: {:?}", sig);

    let free_tx = buffer_close_instruction(
        &buffer_helper.new_account_signer.pubkey(),
        &tctx.payer.pubkey(),
        Some(tctx.payer.pubkey()),
        None,
    );
    let last_blockhash = tctx.client.get_latest_blockhash().await.unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[free_tx],
        Some(&tctx.payer.pubkey()),
        &[&tctx.payer],
        last_blockhash,
    );
    let sig = tctx.client.send_and_confirm_transaction(&tx).await?;
    println!("freed buffer: {:?}", sig);
    Ok(())
}

#[tokio::test]
async fn append_block_test_1() {
    // note: you must run deploy before this
    run_append_block_test().await.unwrap();
}
