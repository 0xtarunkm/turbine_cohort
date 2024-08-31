"use client"

import { useAnchorWallet, useConnection, useWallet } from "@solana/wallet-adapter-react";
import { useState } from "react"
import { program } from "../../anchor/setup";
import { BN } from "@coral-xyz/anchor";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

export const Withdraw = () => {
  const { sendTransaction } = useWallet();
  const { connection } = useConnection();

  const anchor = useAnchorWallet();

  const [amount, setAmount] = useState(0);

  const withdraw = async () => {
    try {
      if (!anchor?.publicKey) return;

      const depositTx = await program.methods
        .withdraw(new BN(amount * LAMPORTS_PER_SOL))
        .accounts({
          signer: anchor.publicKey
        })
        .transaction()

      const depositTxSig = await sendTransaction(
        depositTx,
        connection
      )
      console.log(`depositTx -> ${depositTxSig}`)
    } catch (error) {
      console.log(error);
    }
  }
  return (
    <div>
      <input type="number" placeholder="amount" value={amount} onChange={(e) => setAmount(parseInt(e.target.value))} />

      <button onClick={withdraw}>Withdraw</button>
    </div>
  )
}
