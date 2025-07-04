import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor";
import { IDL, Turbin3Prereq } from "./programs/Turbin3_prereq";
import wallet from "./Turbin3-wallet.json";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection("https://api.devnet.solana.com");
const provider = new AnchorProvider(connection, new Wallet(keypair), { commitment: "confirmed" });
const MPL_CORE_PROGRAM_ID = new PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");
const program = new Program<Turbin3Prereq>(IDL, provider);

const account_seeds = [Buffer.from("prereqs"), keypair.publicKey.toBuffer()];
const [account_key, _account_bump]  = PublicKey.findProgramAddressSync(account_seeds, program.programId);

const mintCollection = new PublicKey("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2");
const [authority_pda, authority_bump] = PublicKey.findProgramAddressSync([Buffer.from("collection"),mintCollection.toBuffer()],program.programId);
const mintTs = Keypair.generate();

(async () => {
  try {
    const initTx = await program.methods
      .initialize("AAshu1412")
      .accountsPartial({
        user: keypair.publicKey,
        account: account_key,
        system_program: SystemProgram.programId,
      })
      .signers([keypair])
      .rpc();
    console.log(
      `Init TX: https://explorer.solana.com/tx/${initTx}?cluster=devnet`
    );

    const submitTx = await program.methods
      .submitTs()
      .accountsPartial({
        user: keypair.publicKey,
        account: account_key,
        mint: mintTs.publicKey,
        collection: mintCollection,
        authority: authority_pda,
        mpl_core_program: MPL_CORE_PROGRAM_ID,
        system_program: SystemProgram.programId,
      })
      .signers([keypair, mintTs])
      .rpc();
    console.log(
      `Mint TX: https://explorer.solana.com/tx/${submitTx}?cluster=devnet`
    );
    
    console.log("Keypair Public key: " + keypair.publicKey);
    console.log("account_key prereqs: " + account_key);
    console.log("authority_pda collection: " + authority_pda);
    console.log("mintTs.publicKey: " + mintTs.publicKey);
    console.log("Program ID: " + program.programId);
    console.log("SystemProgram ID: " + SystemProgram.programId);
  } catch (e) {
    console.error(`Error: ${e}`);
  }
})();
