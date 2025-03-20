import { appendTransactionMessageInstruction, Blockhash, createTransactionMessage, generateKeyPairSigner, KeyPairSigner, pipe, setTransactionMessageFeePayerSigner, setTransactionMessageLifetimeUsingBlockhash, Signature, TransactionPartialSigner } from "@solana/kit";
import { ISolanaClient } from "./types";

import {getCreateAccountInstruction} from "@solana-program/system";
import { getCloseInstruction, getInitializeBufferInstruction, getWriteInstruction, QED_DATA_LOADER_PROGRAM_ADDRESS } from "../buffer";
import { createDefaultTransactionWithInstructions, signAndSendTransaction } from "./utils";

class QEDSolanaBufferManager {
    payer: TransactionPartialSigner<any>;
    client: ISolanaClient;
    bufferAccount: KeyPairSigner<string>;
    totalBufferSize: number;
    offset: number;
    constructor(client: ISolanaClient, payer: TransactionPartialSigner<any>, bufferAccount: KeyPairSigner<string>) {
        this.client = client;
        this.payer = payer;
        this.bufferAccount = bufferAccount;
        this.totalBufferSize = 0;
        this.offset = 0;
    }

    static async createBase(
        payer: TransactionPartialSigner<any>,
        client: ISolanaClient,
    ): Promise<QEDSolanaBufferManager> {
        const bufferAccount = await generateKeyPairSigner();
        return new QEDSolanaBufferManager(client, payer, bufferAccount);
    }

    async initBuffer(totalBufferSize: number) {
        this.totalBufferSize = totalBufferSize;
        const minBalance = await this.client.rpc.getMinimumBalanceForRentExemption(BigInt(totalBufferSize + 40)).send();
        const createAccIx = getCreateAccountInstruction({
            lamports: minBalance,
            payer: this.payer,
            newAccount: this.bufferAccount,
            space: totalBufferSize + 40,
            programAddress: QED_DATA_LOADER_PROGRAM_ADDRESS,
        });

        const initBufferIx = getInitializeBufferInstruction({
            sourceAccount: this.bufferAccount.address,
            bufferAuthority: this.payer.address,
        });

        const tx = await createDefaultTransactionWithInstructions(
            this.client,
            this.payer,
            [createAccIx, initBufferIx],
        );
        const sig = await signAndSendTransaction(this.client, tx);

        return sig;
    }

    async writeBuffer(data: Uint8Array): Promise<Signature[]> {
        const chunkSize = 600;
        let offset = this.offset;
        let {value: latestBlockhash} = await this.client.rpc.getLatestBlockhash().send();
        const transactions: any[] = [];
        while (offset < data.length) {
            const chunk = data.slice(offset, offset + chunkSize);
            const tx = this.createWriteBufferTx(chunk, offset, latestBlockhash);
            transactions.push(tx);
            offset += chunkSize;
        }
        this.offset = offset;
        const sigs = await Promise.all(
            transactions.map((tx) => signAndSendTransaction(this.client, tx))
        );
        return sigs;
    }

    async freeBuffer() {
        const closeIx = getCloseInstruction({
            bufferOrProgramDataAccount: this.bufferAccount.address,
            destinationAccount: this.payer.address,
            authority: this.payer,
        });

        const tx = await createDefaultTransactionWithInstructions(
            this.client,
            this.payer,
            [closeIx],
        );
        const sig = await signAndSendTransaction(this.client, tx);
        return sig;
    }

    createWriteBufferTx(dataChunk: Uint8Array, offset: number, latestBlockhash: {
        blockhash: Blockhash;
        lastValidBlockHeight: bigint;
    }) {
        const writeIx = getWriteInstruction({
            bufferAccount: this.bufferAccount.address,
            bufferAuthority: this.payer,
            bytes: dataChunk,
            offset,
        });

        const base = pipe(
            createTransactionMessage({ version: 0 }),
            (tx) => setTransactionMessageFeePayerSigner(this.payer, tx),
            (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
            (tx) => appendTransactionMessageInstruction(writeIx, tx),
        );
        return base;
    }

    static async createAndInit(
        payer: TransactionPartialSigner<any>,
        client: ISolanaClient,
        size: number,
    ): Promise<QEDSolanaBufferManager> {
        const manager = await QEDSolanaBufferManager.createBase(payer, client);
        //console.log("created base: ",manager.bufferAccount.address);

        await manager.initBuffer(size);
        //console.log("init buffer: ",manager.bufferAccount.address);
        
        return manager;
    }

    static async createAndWrite(
        payer: TransactionPartialSigner<any>,
        client: ISolanaClient,
        buffer: Uint8Array,
        size = -1,
    ): Promise<QEDSolanaBufferManager> {
        const realSize = size === -1 ? buffer.length : size;
        //console.log("real size: ",realSize);
        const manager = await QEDSolanaBufferManager.createAndInit(payer, client, realSize);
        //console.log("init at ",manager.bufferAccount.address);

        await manager.writeBuffer(buffer);
        return manager;
    }
}


export {
    QEDSolanaBufferManager,
}