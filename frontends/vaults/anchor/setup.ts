import { clusterApiUrl, Connection, PublicKey } from '@solana/web3.js';
import { IdlAccounts, Program } from '@coral-xyz/anchor';
import type { Vault } from './idlTypes';
import idl from './idl.json';
import { useAnchorWallet } from '@solana/wallet-adapter-react';

const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

export const program = new Program(idl as Vault, {
  connection,
});

export type VaultData = IdlAccounts<Vault>['vaultState'];
