#!/usr/bin/env zx
import 'zx/globals';
import * as c from 'codama';
import { rootNodeFromAnchor } from '@codama/nodes-from-anchor';
import { renderVisitor as renderJavaScriptVisitor } from '@codama/renderers-js';
import { renderVisitor as renderRustVisitor } from '@codama/renderers-rust';
import { getAllProgramIdls } from './utils.mjs';

// Instanciate Codama.
const [idl, ...additionalIdls] = getAllProgramIdls().map((idl) =>
  rootNodeFromAnchor(require(idl))
);
const codama = c.createFromRoot(idl, additionalIdls);

// Update programs.
codama.update(
  c.updateProgramsVisitor({
    qedSolanaDogeIbcProgram: { name: 'qedSolanaDogeIbc' },
  })
);

// Update accounts.
codama.update(
  c.updateAccountsVisitor({
    qed_doge_ibc: {
      seeds: [
        c.constantPdaSeedNodeFromString('utf8', 'qed_doge_ibc'),
        c.variablePdaSeedNode(
          'authority',
          c.publicKeyTypeNode(),
          'The authority of the qed_doge_ibc account'
        ),
      ],
    },
  })
);

// Update instructions.
codama.update(
  c.updateInstructionsVisitor({
    create: {
      byteDeltas: [c.instructionByteDeltaNode(c.accountLinkNode('qed_doge_ibc'))],
      accounts: {
        qed_doge_ibc: { defaultValue: c.pdaValueNode('qed_doge_ibc') },
        payer: { defaultValue: c.accountValueNode('authority') },
      },
    },
    append_block: {
      accounts: {
        qed_doge_ibc: { defaultValue: c.pdaValueNode('qed_doge_ibc') },
      },
      arguments: {
        /*
        block_number: { defaultValue: c.numberValueNode(0), type: c.numberTypeNode("u32") },
        pow_hash_mode:  { defaultValue: c.numberValueNode(0),type: c.numberTypeNode("u32") },
        block_header:  { type: c.definedTypeLinkNode("QSQBlockHeader")},*/
      },
    },
  })
);

// Set account discriminators.
const key = (name) => ({ field: 'key', value: c.enumValueNode('Key', name) });
codama.update(
  c.setAccountDiscriminatorFromFieldVisitor({
    qed_doge_ibc: key('qed_doge_ibc'),
  })
);

// Render JavaScript.
const jsClient = path.join(__dirname, '..', 'clients', 'js');
codama.accept(
  renderJavaScriptVisitor(path.join(jsClient, 'src', 'generated'), {
    prettierOptions: require(path.join(jsClient, '.prettierrc.json')),
  })
);

// Render Rust.
const rustClient = path.join(__dirname, '..', 'clients', 'rust');
codama.accept(
  renderRustVisitor(path.join(rustClient, 'src', 'generated'), {
    formatCode: true,
    crateFolder: rustClient,
  })
);
