import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createSignerFromKeypair,
  signerIdentity,
  generateSigner,
  percentAmount,
} from "@metaplex-foundation/umi";
import {
  createNft,
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";
import { writeFile, readFile } from "fs/promises";
import base58 from "bs58";
import { create } from "@metaplex-foundation/mpl-core";

const walletJson = await readFile("../Turbin3-wallet.json", "utf-8");
const wallet = JSON.parse(walletJson);

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata());

const mint = generateSigner(umi);
const metadataUri =
  "https://gateway.irys.xyz/3ZUd1gbTBK81MnGjvTARBcyVUS3VzAiaq39DhJ9yyU9i";

(async () => {
  const tx = await create(umi, {
    asset: mint,
    name: "Generug AASHU",
    uri: metadataUri,
  }).sendAndConfirm(umi);

  const signature = base58.encode(tx.signature);

  console.log("\nNFT Created");
  console.log("View Transaction on Solana Explorer");
  console.log(`https://explorer.solana.com/tx/${signature}?cluster=devnet`);

  //   https://explorer.solana.com/tx/2Q6vxZTgfUHeyp5hhCAX4F6RoSe68y1LL5k1ogvqrs3wX5kEsTYqE32DBUSB6eyzrwF5TS2fpv7WKYiikgVBkLAv?cluster=devnet
  // Mint Address:  HvABVVSKQXgcanzm52pazY5Wkr1WH4hcfMQ2vCXaHCiW

  console.log("Mint Address: ", mint.publicKey);
})();
