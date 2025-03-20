import { appendTransactionMessageInstruction, Commitment, CompilableTransactionMessage, createTransactionMessage, getSignatureFromTransaction, IInstruction, pipe, sendAndConfirmTransactionFactory, setTransactionMessageFeePayerSigner, setTransactionMessageLifetimeUsingBlockhash, signTransactionMessageWithSigners, TransactionMessageWithBlockhashLifetime, TransactionSigner } from "@solana/kit";
import { ISolanaClient } from "./types";



export const createDefaultTransaction = async (
    client: ISolanaClient,
    feePayer: TransactionSigner
) => {
    const { value: latestBlockhash } = await client.rpc
        .getLatestBlockhash()
        .send();
    return pipe(
        createTransactionMessage({ version: 0 }),
        (tx) => setTransactionMessageFeePayerSigner(feePayer, tx),
        (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx)
    );
};


export const createDefaultTransactionWithInstructions = async (
    client: ISolanaClient,
    feePayer: TransactionSigner,
    instructions: IInstruction[],
) => {
    let base = await createDefaultTransaction(client, feePayer);
    for (const instruction of instructions) {
        base = appendTransactionMessageInstruction(instruction, base);
    }
    return base;
};
export const signAndSendTransaction = async (
    client: ISolanaClient,
    transactionMessage: CompilableTransactionMessage &
        TransactionMessageWithBlockhashLifetime,
    commitment: Commitment = 'confirmed'
) => {
    const signedTransaction =
        await signTransactionMessageWithSigners(transactionMessage);
    const signature = getSignatureFromTransaction(signedTransaction);
    await sendAndConfirmTransactionFactory(client)(signedTransaction, {
        commitment,
    });
    return signature;
};

