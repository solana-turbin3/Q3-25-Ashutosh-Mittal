// import wallet from "./Turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

// Load wallet JSON from file (fixes your error)
const walletJson = await readFile("./Turbin3-wallet.json", "utf-8");
const wallet = JSON.parse(walletJson);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        //1. Load image
        //2. Convert image to generic file.
        //3. Upload image

        const image = readFile("./generug.png");

        const umiImageFile =  createGenericFile(await image, 'ashu.jpeg', {
  tags: [{ name: 'Content-Type', value: 'image/jpeg' }],
});

const imageUri = await umi.uploader.upload([umiImageFile]).catch((err) => {
  throw new Error(err)
})
        console.log("Your image URI: ", imageUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();