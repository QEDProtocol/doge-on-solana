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
  type ParsedCloseInstruction,
  type ParsedInitializeBufferInstruction,
  type ParsedSetAuthorityCheckedInstruction,
  type ParsedSetAuthorityInstruction,
  type ParsedWriteInstruction,
} from '../instructions';

export const QED_DATA_LOADER_PROGRAM_ADDRESS =
  'CzqeK66uHUYbauvaLJ3sfQd9JmiMqvvPvAudpZmhr6xF' as Address<'CzqeK66uHUYbauvaLJ3sfQd9JmiMqvvPvAudpZmhr6xF'>;

export enum QedDataLoaderInstruction {
  InitializeBuffer,
  Write,
  SetAuthority,
  Close,
  SetAuthorityChecked,
}

export function identifyQedDataLoaderInstruction(
  instruction: { data: ReadonlyUint8Array } | ReadonlyUint8Array
): QedDataLoaderInstruction {
  const data = 'data' in instruction ? instruction.data : instruction;
  if (containsBytes(data, getU8Encoder().encode(0), 0)) {
    return QedDataLoaderInstruction.InitializeBuffer;
  }
  if (containsBytes(data, getU8Encoder().encode(1), 0)) {
    return QedDataLoaderInstruction.Write;
  }
  if (containsBytes(data, getU8Encoder().encode(2), 0)) {
    return QedDataLoaderInstruction.SetAuthority;
  }
  if (containsBytes(data, getU8Encoder().encode(3), 0)) {
    return QedDataLoaderInstruction.Close;
  }
  if (containsBytes(data, getU8Encoder().encode(4), 0)) {
    return QedDataLoaderInstruction.SetAuthorityChecked;
  }
  throw new Error(
    'The provided instruction could not be identified as a qedDataLoader instruction.'
  );
}

export type ParsedQedDataLoaderInstruction<
  TProgram extends string = 'CzqeK66uHUYbauvaLJ3sfQd9JmiMqvvPvAudpZmhr6xF',
> =
  | ({
      instructionType: QedDataLoaderInstruction.InitializeBuffer;
    } & ParsedInitializeBufferInstruction<TProgram>)
  | ({
      instructionType: QedDataLoaderInstruction.Write;
    } & ParsedWriteInstruction<TProgram>)
  | ({
      instructionType: QedDataLoaderInstruction.SetAuthority;
    } & ParsedSetAuthorityInstruction<TProgram>)
  | ({
      instructionType: QedDataLoaderInstruction.Close;
    } & ParsedCloseInstruction<TProgram>)
  | ({
      instructionType: QedDataLoaderInstruction.SetAuthorityChecked;
    } & ParsedSetAuthorityCheckedInstruction<TProgram>);
