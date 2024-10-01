'use client';

import {
  useAnchorWallet,
  useConnection,
  useWallet,
} from '@solana/wallet-adapter-react';
import { program } from '../../anchor/setup';
import { useState } from 'react';
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from '@solana/web3.js';
import { BN } from '@coral-xyz/anchor';

export const Deposit = () => {
  const { sendTransaction } = useWallet();
  const { connection } = useConnection();

  const anchor = useAnchorWallet();

  const [amount, setAmount] = useState(0);

  const init = async () => {
    if (!anchor?.publicKey) return;

    try {
      const initTx = await program.methods
        .initialize()
        .accounts({
          signer: anchor.publicKey,
        })
        .transaction();

      const initTxSig = await sendTransaction(initTx, connection);

      console.log(`initTx -> ${initTxSig}`);
    } catch (error) {
      console.log(error);
    }
  };

  const onClick = async () => {
    if (!anchor?.publicKey) return;

    try {
      const depositTx = await program.methods
        .deposit(new BN(amount * LAMPORTS_PER_SOL))
        .accounts({
          signer: anchor.publicKey,
        })
        .transaction();

      const depositTxSig = await sendTransaction(depositTx, connection);

      console.log(`depositTx -> ${depositTxSig}`);
    } catch (error) {
      console.log('transaction failed');
      console.log(error);
    }
  };
  return (
    <div>
      <input
        type="number"
        value={amount}
        onChange={(e) => setAmount(parseInt(e.target.value))}
      />
      <button onClick={init}>init</button>
      <button onClick={onClick}>Deposit</button>
    </div>
  );
};
