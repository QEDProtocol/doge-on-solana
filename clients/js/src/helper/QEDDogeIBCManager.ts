import { Address, TransactionPartialSigner } from "@solana/kit";
import { ISolanaClient } from "./types";
import { signAndSendTransaction } from "./utils";
import { QEDSolanaBufferManager } from "./QEDSolanaBufferManager";
import { genAppendBlockIBCInstructionData, genAppendBlockZKPIBCInstructionData, genCreateIBCInstructionData } from "./instructionBuilder";
import { getAppendBlockTransactionFromDataAccount, getCreateTransactionFromDataAccount } from "./dataAccountInstructionHelper";
import { ensureHexBytes } from "./dataUtils";

class QEDDogeIBCManager {
    payer: TransactionPartialSigner<any>;
    client: ISolanaClient;
    pdaAddress: Address<string>;
    constructor(client: ISolanaClient, payer: TransactionPartialSigner<any>, address: Address<string> = "" as any) {
        this.client = client;
        this.payer = payer;
        this.pdaAddress = address;
    }

    async initDogeIBCBridge(initStateData: Uint8Array | string) {
        const createIxData = genCreateIBCInstructionData(ensureHexBytes(initStateData));

        const bufferHelper = await QEDSolanaBufferManager.createAndWrite(this.payer, this.client, createIxData);

        const {tx, pdaAddress, bump} = await getCreateTransactionFromDataAccount(this.client, bufferHelper.bufferAccount.address, this.payer);
        const sig = await signAndSendTransaction(this.client, tx);
        this.pdaAddress = pdaAddress;

        const freeSig = await bufferHelper.freeBuffer();

        return sig;
    }

    async appendBlock(blockNumber: number, blockHeaderBytes: Uint8Array | string) {
        const appendBlockIxData = genAppendBlockIBCInstructionData(blockNumber, ensureHexBytes(blockHeaderBytes));


        const bufferHelper = await QEDSolanaBufferManager.createAndWrite(this.payer, this.client, appendBlockIxData);

        const tx = await getAppendBlockTransactionFromDataAccount(this.client, bufferHelper.bufferAccount.address, this.pdaAddress, this.payer);

        const sig = await signAndSendTransaction(this.client, tx);
        

        const freeSig = await bufferHelper.freeBuffer();

        return sig;
    }

    async appendBlockZKP(blockNumber: number, blockHeaderBytes: Uint8Array | string, scryptHash: Uint8Array | string, proof: Uint8Array | string) {
        const appendBlockIxData = genAppendBlockZKPIBCInstructionData(blockNumber, ensureHexBytes(blockHeaderBytes), ensureHexBytes(scryptHash), ensureHexBytes(proof));


        const bufferHelper = await QEDSolanaBufferManager.createAndWrite(this.payer, this.client, appendBlockIxData);

        const tx = await getAppendBlockTransactionFromDataAccount(this.client, bufferHelper.bufferAccount.address, this.pdaAddress, this.payer);

        const sig = await signAndSendTransaction(this.client, tx);
        

        const freeSig = await bufferHelper.freeBuffer();

        return sig;
    }

    static async createAndInit(
        payer: TransactionPartialSigner<any>,
        client: ISolanaClient,
        initStateData: Uint8Array | string,
    ): Promise<QEDDogeIBCManager> {
        const manager = new QEDDogeIBCManager(client, payer);
        await manager.initDogeIBCBridge(initStateData);
        return manager;
    }


}

export {
    QEDDogeIBCManager,
}