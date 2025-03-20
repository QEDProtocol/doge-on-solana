import { AccountRole, Address, getAddressEncoder, getProgramDerivedAddress, IAccountMeta, IInstruction, ReadonlyUint8Array, TransactionPartialSigner } from "@solana/kit";
import { ISolanaClient } from "./types";
import { getAppendBlockInstruction, getCreateInstruction, QED_SOLANA_DOGE_IBC_V3_PROGRAM_PROGRAM_ADDRESS, QSQDogeBlockHeader, QSQStandardBlockHeader } from "../generated";
import { createDefaultTransactionWithInstructions, signAndSendTransaction } from "./utils";
import {getSetComputeUnitLimitInstruction} from "@solana-program/compute-budget";

/*



    let qed_doge_ibc_address = find_pda_ibc(&tctx.payer.pubkey()).0;
    let ix = CreateBuilder::new()
        .qed_doge_ibc(qed_doge_ibc_address)
        .authority(tctx.payer.pubkey())
        .payer(tctx.payer.pubkey())
        .init_data(Vec::new())
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

    let tx = Transaction::new_signed_with_payer(
        &[dummy_ix],
        Some(&tctx.payer.pubkey()),
        &[&tctx.payer],
        last_blockhash,
    );

*/

  /** The program derived address of the qed_doge_ibc account to create (seeds: ['qed_doge_ibc', authority]) */
  //qedDogeIbc: Address<TAccountQedDogeIbc>;
  /** The authority of the qed_doge_ibc */
  //authority: TransactionSigner<TAccountAuthority>;
  /** The account paying for the storage fees */
  //payer?: TransactionSigner<TAccountPayer>;
  /** The system program */
  //systemProgram?: Address<TAccountSystemProgram>;
  //initData: CreateInstructionDataArgs['initData'];
async function getCreateInstructionFromDataAccount(dataAccount: string, payer: TransactionPartialSigner<any>) {

    const addressEncoder = getAddressEncoder();
    let [qedDogeIBCAddress, bump] = await getProgramDerivedAddress({
        programAddress: QED_SOLANA_DOGE_IBC_V3_PROGRAM_PROGRAM_ADDRESS,
        seeds: [new TextEncoder().encode('qed_doge_ibc'), addressEncoder.encode(payer.address)],
    });

    const createIx = getCreateInstruction({
        qedDogeIbc: qedDogeIBCAddress,
        authority: payer,
        payer: payer,
        initData: new Uint8Array(),
    });
    const dataAcc: IAccountMeta<any> = {
        address: dataAccount,
        role: AccountRole.READONLY,
    };
    const result: IInstruction<any> = {
        ...createIx,
        accounts: createIx.accounts.concat([dataAcc]),
        data: new Uint8Array(),
    };
    return {ix: result, pdaAddress: qedDogeIBCAddress, bump};
}
async function getCreateTransactionFromDataAccount(client: ISolanaClient, dataAccount: string, payer: TransactionPartialSigner<any>) {
    const {ix: createIx, pdaAddress, bump} = await getCreateInstructionFromDataAccount(dataAccount, payer);
    const tx = await createDefaultTransactionWithInstructions(client, payer, [createIx]);
    return {tx, pdaAddress, bump};
}

const emptyStandardBlockHeaderDefault: QSQStandardBlockHeader = {
  version: 0,
  previousBlockHash: new Uint8Array(32),
  merkleRoot: new Uint8Array(32),
  timestamp: 0,
  bits: 0,
  nonce: 0,
};
const emptyQSQBlockHeaderDefault: QSQDogeBlockHeader = {
    auxPow: null,
    header: emptyStandardBlockHeaderDefault,
};

async function getAppendBlockInstructionFromDataAccount(dataAccount: string, qedDogeIBCAddress: Address<string>, payer: TransactionPartialSigner<any>) {

    const appendIx = getAppendBlockInstruction({
        qedDogeIbc: qedDogeIBCAddress,
        authority: payer,
        blockHeader: emptyQSQBlockHeaderDefault,
        powHashMode: 0,
        blockNumber: 1,
    });
    const dataAcc: IAccountMeta<any> = {
        address: dataAccount,
        role: AccountRole.READONLY,
    };
    const result: IInstruction<any> = {
        ...appendIx,
        accounts: appendIx.accounts.concat([dataAcc]),
        data: new Uint8Array(),
    };
    return result;
}
async function getAppendBlockTransactionFromDataAccount(client: ISolanaClient, dataAccount: string, qedDogeIBCAddress: Address<string>, payer: TransactionPartialSigner<any>) {
    const appendIx = await getAppendBlockInstructionFromDataAccount(dataAccount, qedDogeIBCAddress, payer);

    const setComputeLimitIx = getSetComputeUnitLimitInstruction({units: 1_000_000});

    const tx = await createDefaultTransactionWithInstructions(client, payer, [setComputeLimitIx, appendIx]);
    return tx;
}

export {
    getCreateInstructionFromDataAccount,
    getCreateTransactionFromDataAccount,
    getAppendBlockInstructionFromDataAccount,
    getAppendBlockTransactionFromDataAccount,
}