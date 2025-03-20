/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  containsBytes,
  getU8Encoder,
  type Address,
  type ReadonlyUint8Array,
} from '@solana/kit';
import {
  type ParsedAppendBlockInstruction,
  type ParsedAppendBlockZKPInstruction,
  type ParsedCreateInstruction,
} from '../instructions';

export const QED_SOLANA_DOGE_IBC_V3_PROGRAM_PROGRAM_ADDRESS =
  'Fu4pdQiKyrBKnUyvtbGUPkcg2HKp9d6Ji8JLGvS6E7UQ' as Address<'Fu4pdQiKyrBKnUyvtbGUPkcg2HKp9d6Ji8JLGvS6E7UQ'>;

export enum QedSolanaDogeIbcV3ProgramAccount {
  QEDDogeIBC,
}

export enum QedSolanaDogeIbcV3ProgramInstruction {
  Create,
  AppendBlock,
  AppendBlockZKP,
}

export function identifyQedSolanaDogeIbcV3ProgramInstruction(
  instruction: { data: ReadonlyUint8Array } | ReadonlyUint8Array
): QedSolanaDogeIbcV3ProgramInstruction {
  const data = 'data' in instruction ? instruction.data : instruction;
  if (containsBytes(data, getU8Encoder().encode(0), 0)) {
    return QedSolanaDogeIbcV3ProgramInstruction.Create;
  }
  if (containsBytes(data, getU8Encoder().encode(1), 0)) {
    return QedSolanaDogeIbcV3ProgramInstruction.AppendBlock;
  }
  if (containsBytes(data, getU8Encoder().encode(2), 0)) {
    return QedSolanaDogeIbcV3ProgramInstruction.AppendBlockZKP;
  }
  throw new Error(
    'The provided instruction could not be identified as a qedSolanaDogeIbcV3Program instruction.'
  );
}

export type ParsedQedSolanaDogeIbcV3ProgramInstruction<
  TProgram extends string = 'Fu4pdQiKyrBKnUyvtbGUPkcg2HKp9d6Ji8JLGvS6E7UQ',
> =
  | ({
      instructionType: QedSolanaDogeIbcV3ProgramInstruction.Create;
    } & ParsedCreateInstruction<TProgram>)
  | ({
      instructionType: QedSolanaDogeIbcV3ProgramInstruction.AppendBlock;
    } & ParsedAppendBlockInstruction<TProgram>)
  | ({
      instructionType: QedSolanaDogeIbcV3ProgramInstruction.AppendBlockZKP;
    } & ParsedAppendBlockZKPInstruction<TProgram>);
