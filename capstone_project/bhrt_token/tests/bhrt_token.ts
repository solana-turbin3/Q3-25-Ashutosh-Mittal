import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import * as spl from "@solana/spl-token";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram
} from "@solana/web3.js";
import {BhrtToken} from "../target/types/bhrt_token";
import { Metaplex, keypairIdentity } from "@metaplex-foundation/js";
import { assert } from "chai";


describe("bhrt_token", () => {
  // const provider = anchor.getProvider();
  const provider = anchor.AnchorProvider.env();   
  anchor.setProvider(provider);
  const program = anchor.workspace.BhrtToken as Program<BhrtToken>;
  const programId = program.programId;
  const tokenProgram = spl.TOKEN_2022_PROGRAM_ID;
  const metaplex = Metaplex.make(provider.connection as any);

  console.log("RPC:", provider.connection.rpcEndpoint);

  const NFT_ID = new anchor.BN(1);

  const confirm = async (signature: string): Promise<string> => {
    const block = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({ signature, ...block });
    return signature;
  };

  const log = async (signature: string): Promise<string> => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=devnet`
    );
    return signature;
  };



  const authority = Keypair.generate();
  const miner = Keypair.generate();
  const mintA = Keypair.generate();
  const mintB = Keypair.generate();
    
  // const [authority_ata_bhrt, miner_ata_bhrt, takerAtaA, takerAtaB] = [authority, miner]
  //   .map((a) =>
  //     [mintA, mintB].map((m) =>
  //       spl.getAssociatedTokenAddressSync(
  //         m.publicKey,
  //         a.publicKey,
  //         false,
  //         tokenProgram
  //       )
  //     )
  //   )
  //   .flat();

  const [program_state, program_state_bump] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("program_state")
    ],
    programId
  );

  const [bhrt_mint, bhrt_mint_bump] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("BHRT")
    ],
    programId
  );

  const [bhrt_metadata, bhrt_metadata_bump] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("bhrt_metadata"), program_state.toBuffer()
    ],
    programId
  );

  const [collection_mint, collection_mint_bump] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("collection_mint")
    ],
    programId
  );
  // const collectionMintKp = Keypair.generate();
  // const collection_mint = collectionMintKp.publicKey;


  const collection_token_account = spl.getAssociatedTokenAddressSync(
    collection_mint,
    program_state,
    true,
    tokenProgram
  );

  const metadataProgram = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  // const collection_master_edition_account = PublicKey.findProgramAddressSync(
  //   [
  //     Buffer.from("metadata"),
  //     metadataProgram.toBuffer(),
  //     collection_mint.toBuffer(),
  //     Buffer.from("edition")
  //   ],
  //   metadataProgram
  // )[0];

  // const nft_collection_metadata = PublicKey.findProgramAddressSync(
  //   [
  //     Buffer.from("metadata"),
  //     metadataProgram.toBuffer(),
  //     collection_mint.toBuffer()
  //   ],
  //   metadataProgram
  // )[0];


  // const collection_master_edition_account = metaplex.nfts().pdas().masterEdition({ mint: collection_mint  });
  const getMasterEditionAddress = async (mint) => {
    return (
      await PublicKey.findProgramAddressSync([
        Buffer.from("metadata"),
        metadataProgram.toBuffer(),
        mint.toBuffer(),
        Buffer.from("edition"),
      ],
      tokenProgram
    ))[0];
  };

  const getMetadataAddress = async (mint) => {
    return (
      await PublicKey.findProgramAddressSync([
        Buffer.from("metadata"),
        metadataProgram.toBuffer(),
        mint.toBuffer(),
      ],
      tokenProgram
    ))[0];
  };

  const [collection_master_edition_account, collection_master_edition_account_bump] =  PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
        metadataProgram.toBuffer(),
        collection_mint.toBuffer(),
        Buffer.from("edition"),
    ],
    metadataProgram
  );
  // const nft_collection_metadata = metaplex.nfts().pdas().metadata({ mint: collection_mint });
  const [nft_collection_metadata, nft_collection_metadata_bump] =  PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
        metadataProgram.toBuffer(),
        collection_mint.toBuffer(),
    ],
    metadataProgram
  );


  const rent = anchor.web3.SYSVAR_RENT_PUBKEY;
  const system = anchor.web3.SystemProgram.programId;
  const sysvar_instructions= anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY;


  const [miner_nft_mint, miner_nft_mint_bump] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("nft_mint"), miner.publicKey.toBuffer(), NFT_ID.toArrayLike(Buffer, "le", 8)
    ],
    programId
  );
  
  const miner_nft_token_account = spl.getAssociatedTokenAddressSync(
    miner_nft_mint,
    miner.publicKey,
    false,
    tokenProgram
  );

  // const miner_nft_metadata = PublicKey.findProgramAddressSync(

  //   [
  //     Buffer.from("metadata"),
  //     metadataProgram.toBuffer(),
  //     miner_nft_mint.toBuffer()
  //   ],
  //   metadataProgram
  // )[0];

  const miner_nft_metadata = metaplex.nfts().pdas().metadata({ mint: miner_nft_mint });

  const [miner_info, miner_info_bump] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("miner"), miner.publicKey.toBuffer()
    ],
    programId
  );

  const miner_bhrt = spl.getAssociatedTokenAddressSync(
    bhrt_mint,
    miner.publicKey,
    false,
    tokenProgram
  );

    // --- Airdrops ---
    // before(async () => {
    //   await provider.connection.requestAirdrop(authority.publicKey, 10 * LAMPORTS_PER_SOL).then(confirm);
    //   await provider.connection.requestAirdrop(miner.publicKey, 10 * LAMPORTS_PER_SOL).then(confirm);
    // });
    before(async () => {
      const balance = await provider.connection.getBalance(authority.publicKey);
      if (balance < 1_000_000_000) {
        console.log("Requesting airdrop for authority...");
        const signature = await provider.connection.requestAirdrop(authority.publicKey, 2_000_000_000);
        await provider.connection.confirmTransaction({
          signature,
          blockhash: (await provider.connection.getLatestBlockhash()).blockhash,
          lastValidBlockHeight: (await provider.connection.getLatestBlockhash()).lastValidBlockHeight,
        });
      }
    });
  
    // --- Test Cases ---
    it("Initializes the protocol", async () => {
      try {
        console.log("authority", JSON.stringify(authority.publicKey));
        console.log("program_state", JSON.stringify(program_state));
        console.log("bhrt_mint", JSON.stringify(bhrt_mint));
        console.log("bhrt_metadata", JSON.stringify(bhrt_metadata));
        console.log("collection_mint", JSON.stringify(collection_mint));
        console.log("collection_token_account", JSON.stringify(collection_token_account));
        console.log("nft_collection_metadata", JSON.stringify(nft_collection_metadata));
        console.log("collection_master_edition_account", JSON.stringify(collection_master_edition_account));  
        console.log("metadataProgram", JSON.stringify(metadataProgram));
        console.log("sysvar_instructions", JSON.stringify(sysvar_instructions));
        console.log("tokenProgram", JSON.stringify(tokenProgram));
        console.log("system", JSON.stringify(system));
        console.log("rent", JSON.stringify(rent));
        console.log("associatedTokenProgram", JSON.stringify(anchor.utils.token.ASSOCIATED_PROGRAM_ID));
        
        await program.methods.authorityinitialization()
        .accountsPartial({
          authority: authority.publicKey,
          programState: program_state,
          bhrtMint: bhrt_mint,
          bhrtMetadata: bhrt_metadata,
          collectionMint: collection_mint,
          collectionTokenAccount: collection_token_account,
          nftCollectionMetadata:  nft_collection_metadata,
          collectionMasterEditionAccount:  collection_master_edition_account,
          metadataProgram: new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
          instructionSysvar: sysvar_instructions,
          tokenProgram: tokenProgram,
          systemProgram: system,
          // rent: rent,
          associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        })
        .signers([authority]) 
        .rpc()
        .then(confirm)
        .then(log);

        // await provider.connection.confirmTransaction({
        //   signature: initialize_authority_tx,
        //   blockhash: (await provider.connection.getLatestBlockhash()).blockhash,
        //   lastValidBlockHeight: (await provider.connection.getLatestBlockhash()).lastValidBlockHeight,
        // });
        // await new Promise(resolve => setTimeout(resolve, 2000));

    } catch (error) {
      // --- This is the error handling block ---
      console.error("Transaction failed!");

      // Check if the error has logs and print them
      if (error.logs) {
        console.error("Full logs:");
        for (const log of error.logs) {
          console.log(`- ${log}`);
        }
      } else {
        // If it's another type of error, print the whole thing
        console.error(error);
      }

      // Re-throw the error to make sure the test still fails
      throw error;
    }
  
      // Assertions
      const programState = await program.account.programState.fetch(program_state);
      assert.ok(programState.authority.equals(authority.publicKey), "Authority mismatch");
      assert.equal(programState.nftIdCounter.toNumber(), 0, "Initial NFT counter should be 0");
  
      const bhrtMetadata = await program.account.bhrtMetadata.fetch(bhrt_metadata);
      assert.ok(bhrtMetadata.collection.equals(collection_mint), "BHRT metadata collection link mismatch");
      assert.ok(bhrtMetadata.mint.equals(bhrt_mint), "BHRT metadata mint link mismatch");
    });
  });


