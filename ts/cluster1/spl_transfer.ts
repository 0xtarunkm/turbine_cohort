import {
  Commitment,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from '@solana/web3.js';
import wallet from '../wba-wallet.json';
import { getOrCreateAssociatedTokenAccount, transfer } from '@solana/spl-token';
import { getExplorerLink } from '@solana-developers/helpers';

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = 'confirmed';
const connection = new Connection('https://api.devnet.solana.com', commitment);

// Mint address
const mint = new PublicKey('FtXP3o6bU1wJsZQWNmBqvxnpQzAnZXGmtisBKEDK12Kr');

// Recipient address
const to = new PublicKey('FdvRKFEgdqJeKXS8SdffMkbYKg2oKwDXfamciiinWXjN');

(async () => {
  try {
    // Get the token account of the fromWallet address, and if it does not exist, create it
    const fromTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey
    );
    // Get the token account of the toWallet address, and if it does not exist, create it
    const toTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      to
    );
    // Transfer the new token to the "toTokenAccount" we just created
    const transferTx = await transfer(
      connection,
      keypair,
      fromTokenAccount.address,
      toTokenAccount.address,
      keypair,
      1 * Math.pow(10, 6)
    );

    const link = getExplorerLink('tx', transferTx, 'devnet');
    console.log(`✅ Finished! Transaction: ${link}`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
