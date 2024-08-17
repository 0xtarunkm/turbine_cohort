import wallet from '../wba-wallet.json';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import {
  createMetadataAccountV3,
  CreateMetadataAccountV3InstructionAccounts,
  CreateMetadataAccountV3InstructionArgs,
  DataV2Args,
} from '@metaplex-foundation/mpl-token-metadata';
import {
  createSignerFromKeypair,
  signerIdentity,
  publicKey,
  GenericFile,
} from '@metaplex-foundation/umi';
import { bundlrUploader } from '@metaplex-foundation/umi-uploader-bundlr';
import * as bs58 from 'bs58';
import * as fs from 'fs';
import * as path from 'path';

const mint = publicKey('FtXP3o6bU1wJsZQWNmBqvxnpQzAnZXGmtisBKEDK12Kr');

const umi = createUmi('https://api.devnet.solana.com');

umi.use(bundlrUploader());

const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

(async () => {
  try {
    // Read the image file
    const imageFileName = 'shinchan.png';
    const imageFilePath = path.join(__dirname, 'assets', imageFileName);
    const fileBuffer = fs.readFileSync(imageFilePath);

    // Create a GenericFile object
    const imageFile: GenericFile = {
      buffer: fileBuffer,
      fileName: imageFileName,
      displayName: imageFileName,
      uniqueName: imageFileName,
      contentType: 'image/png',
      extension: 'png',
      tags: [],
    };

    // Upload image and metadata
    const [imageUri] = await umi.uploader.upload([imageFile]);
    const uri = await umi.uploader.uploadJson({
      name: 'Tarun Kumar',
      description:
        'Token created by Tarun Kumar using Metaplex and Solana Devnet',
      image: imageUri,
    });

    let accounts: CreateMetadataAccountV3InstructionAccounts = {
      mint: mint,
      mintAuthority: signer,
      payer: signer,
      updateAuthority: keypair.publicKey,
    };

    let data: DataV2Args = {
      name: 'Tarun Kumar',
      symbol: 'TK',
      uri: uri,
      sellerFeeBasisPoints: 500,
      creators: [
        {
          address: keypair.publicKey,
          verified: true,
          share: 100,
        },
      ],
      collection: null,
      uses: null,
    };

    let args: CreateMetadataAccountV3InstructionArgs = {
      data: data,
      isMutable: true,
      collectionDetails: null,
    };

    let tx = createMetadataAccountV3(umi, {
      ...accounts,
      ...args,
    });

    let result = await tx.sendAndConfirm(umi);
    console.log(bs58.encode(result.signature));
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
