'use client';

import {
  useAnchorWallet,
  useConnection,
  useWallet,
} from '@solana/wallet-adapter-react';
import { useEffect, useState } from 'react';
import { PublicKey } from '@solana/web3.js';
import { BN } from '@coral-xyz/anchor';
import { LAMPORTS_PER_SOL } from '@solana/web3.js';
import { program } from '../../anchor/setup'; // Assuming you already have your Anchor program setup

const VAULT_SEED = 'vault'; // Your vault seed constant
const STATE_SEED = 'state'; // Your state seed constant

export const Withdraw = () => {
  const { sendTransaction, publicKey } = useWallet();
  const { connection } = useConnection();
  const anchor = useAnchorWallet();

  const [amount, setAmount] = useState(0);
  const [vaultBalance, setVaultBalance] = useState<number | null>(null);

  const [vaultState] = PublicKey.findProgramAddressSync(
    [Buffer.from(STATE_SEED), publicKey!.toBytes()],
    program.programId
  );

  const [vault] = PublicKey.findProgramAddressSync(
    [Buffer.from(VAULT_SEED), vaultState.toBytes()],
    program.programId
  );

  // Withdraw function
  const withdraw = async () => {
    try {
      if (!publicKey) return;

      const withdrawTx = await program.methods
        .withdraw(new BN(amount * LAMPORTS_PER_SOL)) // Withdraw amount in lamports
        .accounts({
          signer: publicKey,
        })
        .transaction();

      const withdrawTxSig = await sendTransaction(withdrawTx, connection);
      console.log(`withdrawTx -> ${withdrawTxSig}`);
    } catch (error) {
      console.error('Error during withdrawal:', error);
    }
  };

  useEffect(() => {
    program.account.vault.fetch(vault).then((data) => {
      setVaultBalance(data);
    });
  }, []);

  return (
    <div>
      <input
        type="number"
        placeholder="amount"
        value={amount}
        onChange={(e) => setAmount(parseInt(e.target.value))}
      />
      <button onClick={withdraw}>Withdraw</button>

      {/* Display the vault balance */}
      <div>
        Vault Balance:{' '}
        {vaultBalance !== null ? `${vaultBalance} SOL` : 'Loading...'}
      </div>
    </div>
  );
};
