/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  addDecoderSizePrefix,
  addEncoderSizePrefix,
  combineCodec,
  getBytesDecoder,
  getBytesEncoder,
  getStructDecoder,
  getStructEncoder,
  getU32Decoder,
  getU32Encoder,
  getU64Decoder,
  getU64Encoder,
  type Codec,
  type Decoder,
  type Encoder,
  type ReadonlyUint8Array,
} from '@solana/kit';

export type QSBTCTransactionOutput = {
  value: bigint;
  script: ReadonlyUint8Array;
};

export type QSBTCTransactionOutputArgs = {
  value: number | bigint;
  script: ReadonlyUint8Array;
};

export function getQSBTCTransactionOutputEncoder(): Encoder<QSBTCTransactionOutputArgs> {
  return getStructEncoder([
    ['value', getU64Encoder()],
    ['script', addEncoderSizePrefix(getBytesEncoder(), getU32Encoder())],
  ]);
}

export function getQSBTCTransactionOutputDecoder(): Decoder<QSBTCTransactionOutput> {
  return getStructDecoder([
    ['value', getU64Decoder()],
    ['script', addDecoderSizePrefix(getBytesDecoder(), getU32Decoder())],
  ]);
}

export function getQSBTCTransactionOutputCodec(): Codec<
  QSBTCTransactionOutputArgs,
  QSBTCTransactionOutput
> {
  return combineCodec(
    getQSBTCTransactionOutputEncoder(),
    getQSBTCTransactionOutputDecoder()
  );
}
