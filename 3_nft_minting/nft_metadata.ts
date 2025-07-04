// import wallet from "./Turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { writeFile, readFile } from "fs/promises"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');
const walletJson = await readFile("./Turbin3-wallet.json", "utf-8");
const wallet = JSON.parse(walletJson);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        // const image = ???
        const imageUri = [
  'https://gateway.irys.xyz/2GnmQHYeDXR6fRLGHwiji7amJoo5ixRz3X2VTGMbmXKu'
];
       const metadata = {
  name: 'Generug AASHU',
  description: 'Nature rugpull',
  image: imageUri[0],
  external_url: 'https://example.com',
  attributes: [
    {
      trait_type: 'Color Palette',
      value: 'Nature',
    },
    {
      trait_type: 'Number of color',
      value: '5',
    },
  ],
  properties: {
    files: [
      {
        uri: imageUri[0],
        type: 'image/jpeg',
      },
    ],
    category: 'image',
  },
}

const json_file = await writeFile("./metadata.json", JSON.stringify(metadata, null, 2));
console.log("JSON FILE: "+ json_file);
const metadata2 = await readFile("./metadata.json");

// Call upon Umi's `uploadJson()` function to upload our metadata to Arweave via Irys.
const metadataUri = await umi.uploader.uploadJson(metadata2).catch((err) => {
  throw new Error(err)
})

console.log("Metadata URI: "+ metadataUri);
        // const myUri = ???
        // console.log("Your metadata URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();