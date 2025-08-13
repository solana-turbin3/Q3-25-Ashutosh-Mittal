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
  // const mintA = Keypair.generate();
  // const mintB = Keypair.generate();
    
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

  const [miner_nft_master_edition_account, miner_nft_master_edition_account_bump] =  PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
        metadataProgram.toBuffer(),
        miner_nft_mint.toBuffer(),
        Buffer.from("edition"),
    ],
    metadataProgram
  );
  // const nft_collection_metadata = metaplex.nfts().pdas().metadata({ mint: collection_mint });
  const [miner_nft_metadata, miner_nft_metadata_bump] =  PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
        metadataProgram.toBuffer(),
        miner_nft_mint.toBuffer(),
    ],
    metadataProgram
  );

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

  describe("bhrt_token", () => {
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
  
    it("Initializes the protocol", async () => {
      try {
        // Log all account addresses
        console.log("authority", JSON.stringify(authority.publicKey));
        console.log("program_state", JSON.stringify(program_state));
        // ... all your other console.log statements ...
  
        // Execute the transaction
        await program.methods.authorityinitialization()
          .accountsPartial({
            authority: authority.publicKey,
            programState: program_state,
            bhrtMint: bhrt_mint,
            bhrtMetadata: bhrt_metadata,
            collectionMint: collection_mint,
            collectionTokenAccount: collection_token_account,
            nftCollectionMetadata: nft_collection_metadata,
            collectionMasterEditionAccount: collection_master_edition_account,
            metadataProgram: new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
            instructionSysvar: sysvar_instructions,
            tokenProgram: tokenProgram,
            systemProgram: system,
            associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
          })
          .signers([authority]) 
          .rpc()
          .then(confirm)
          .then(log);
  
        // Add logging to verify execution
        console.log("🔍 Verifying program state...");
        const programState = await program.account.programState.fetch(program_state);
        assert.ok(programState.authority.equals(authority.publicKey), "Authority mismatch");
        assert.equal(programState.nftIdCounter.toNumber(), 0, "Initial NFT counter should be 0");
        console.log("✅ Program state assertions passed");
  
        console.log("🔍 Verifying BHRT metadata...");
        const bhrtMetadata = await program.account.bhrtMetadata.fetch(bhrt_metadata);
        assert.ok(bhrtMetadata.collection.equals(collection_mint), "BHRT metadata collection link mismatch");
        assert.ok(bhrtMetadata.mint.equals(bhrt_mint), "BHRT metadata mint link mismatch");
        console.log("✅ BHRT metadata assertions passed");
        
      } catch (error) {
        console.error("Transaction failed!");
        if (error.logs) {
          console.error("Full logs:");
          for (const log of error.logs) {
            console.log(`- ${log}`);
          }
        } else {
          console.error(error);
        }
        throw error;
      }
    });
  
    // Remove the duplicate it() block
  });
    
 
  describe("approve_miner", () => {

    it("Approves a miner for onboarding", async () => {
      try {
        console.log("🔍 Approving miner for onboarding...");
        
        await program.methods.approveMiners(miner.publicKey)
          .accounts({
            authority: authority.publicKey,
            programState: program_state,
            systemProgram: system,
          })
          .signers([authority])
          .rpc()
          .then(confirm)
          .then(log);
  
        // Verify miner is approved
        const programState = await program.account.programState.fetch(program_state);
        assert.ok(
          programState.approvedMiners.some(approvedMiner => approvedMiner.equals(miner.publicKey)),
          "Miner should be in approved miners list"
        );
        console.log("✅ Miner approved successfully");
  
      } catch (error) {
        console.error("Miner approval failed!");
        if (error.logs) {
          console.error("Full logs:");
          for (const log of error.logs) {
            console.log(`- ${log}`);
          }
        }
        throw error;
      }
    });


  });

  describe("onboard miner", () => {
    it("Onboards an approved miner and mints NFT with BHRT tokens", async () => {
      try {
        
        const minerName = "Bitcoin Mining Farm #1";
        const minerUri = "https://arweave.net/miner-legal-document-hash";
        const nftId = NFT_ID;
        const miningPower = 1000; // 1000 hashrate units
  
        console.log("💰 Funding miner account...");
        const minerBalance = await provider.connection.getBalance(miner.publicKey);
        if (minerBalance < 1_000_000_000) {
          const signature = await provider.connection.requestAirdrop(miner.publicKey, 2_000_000_000);
          await provider.connection.confirmTransaction({
            signature,
            blockhash: (await provider.connection.getLatestBlockhash()).blockhash,
            lastValidBlockHeight: (await provider.connection.getLatestBlockhash()).lastValidBlockHeight,
          });
        }
  
        // const [miner_nft_master_edition_account] = PublicKey.findProgramAddressSync(
        //   [
        //     Buffer.from("metadata"),
        //     metadataProgram.toBuffer(),
        //     miner_nft_mint.toBuffer(),
        //     Buffer.from("edition"),
        //   ],
        //   metadataProgram
        // );
  
        console.log("🏗️ Onboarding miner and minting NFT...");
        console.log("Parameters:");
        console.log("- Name:", minerName);
        console.log("- URI:", minerUri);
        console.log("- NFT ID:", nftId);
        console.log("- Mining Power:", miningPower);
  
        await program.methods.onboardMiner(
          nftId,
          minerName,
          minerUri,
          new anchor.BN(miningPower)
        )
        .accountsPartial({
          miner: miner.publicKey,
          authority: authority.publicKey,
          programState: program_state,
          collectionMint: collection_mint,
          nftCollectionMetadata: nft_collection_metadata,
          metadataProgram: metadataProgram,
          collectionMasterEditionAccount: collection_master_edition_account,
          minerNftMint: miner_nft_mint,
          minerNftTokenAccount: miner_nft_token_account,
          minerNftMasterEditionAccount: miner_nft_master_edition_account,
          minerNftMetadata: miner_nft_metadata,
          minerInfo: miner_info,
          bhrtMint: bhrt_mint,
          minerBhrt: miner_bhrt,
          instructionSysvar: sysvar_instructions,
          associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
          rent: rent,
          systemProgram: system,
          tokenProgram: tokenProgram,
        })
        .signers([miner])
        .rpc()
        .then(confirm)
        .then(log);
  
        console.log("✅ Miner onboarded successfully");
  
        // Verify program state updates
        console.log("🔍 Verifying program state updates...");
        const updatedProgramState = await program.account.programState.fetch(program_state);
        assert.equal(
          updatedProgramState.nftIdCounter.toNumber(),
          1,
          "NFT ID counter should increment to 1"
        );
        console.log("✅ Program state NFT counter incremented");
  
        // Verify miner info account
        console.log("🔍 Verifying miner info account...");
        const minerInfoAccount = await program.account.minerInfo.fetch(miner_info);
        assert.equal(minerInfoAccount.hashratePower.toNumber(), miningPower, "Mining power should match");
        assert.equal(minerInfoAccount.legalDocumentUri, minerUri, "Legal document URI should match");
        assert.ok(minerInfoAccount.hashrateTokenMint.equals(bhrt_mint), "BHRT mint should match");
        assert.equal(minerInfoAccount.mintAmount.toNumber(), miningPower * 10, "Mint amount should be mining power * 10");
        console.log("✅ Miner info account verified");
  
        // Verify NFT mint account
        console.log("🔍 Verifying NFT mint account...");
        const nftMintInfo = await spl.getMint(provider.connection as any, miner_nft_mint, undefined, tokenProgram);
        assert.equal(nftMintInfo.supply, BigInt(1), "NFT mint supply should be 1");
        assert.equal(nftMintInfo.decimals, 0, "NFT mint should have 0 decimals");
        assert.ok(nftMintInfo.mintAuthority.equals(program_state), "Mint authority should be program state");
        console.log("✅ NFT mint account verified");
  
        // Verify NFT token account
        console.log("🔍 Verifying NFT token account...");
        const nftTokenAccount = await spl.getAccount( provider.connection as any, miner_nft_token_account, undefined, tokenProgram);
        assert.equal(nftTokenAccount.amount, BigInt(1), "Miner should own 1 NFT token");
        assert.ok(nftTokenAccount.owner.equals(miner.publicKey), "NFT should be owned by miner");
        assert.ok(nftTokenAccount.mint.equals(miner_nft_mint), "Token account mint should match NFT mint");
        console.log("✅ NFT token account verified");
  
        // Verify BHRT token balance
        console.log("🔍 Verifying BHRT token balance...");
        const bhrtTokenAccount = await spl.getAccount(provider.connection as any, miner_bhrt, undefined, tokenProgram);
        const expectedBhrtAmount = miningPower * 10;
        assert.equal(
          bhrtTokenAccount.amount, 
          BigInt(expectedBhrtAmount), 
          `Miner should have ${expectedBhrtAmount} BHRT tokens`
        );
        assert.ok(bhrtTokenAccount.owner.equals(miner.publicKey), "BHRT tokens should be owned by miner");
        console.log("✅ BHRT token balance verified");
  
        // Verify NFT metadata exists (basic check)
        console.log("🔍 Verifying NFT metadata account exists...");
        const metadataAccountInfo = await provider.connection.getAccountInfo(miner_nft_metadata);
        assert.ok(metadataAccountInfo, "NFT metadata account should exist");
        assert.ok(metadataAccountInfo.data.length > 0, "NFT metadata should have data");
        console.log("✅ NFT metadata account verified");
  
        // Verify master edition exists
        console.log("🔍 Verifying NFT master edition account exists...");
        const masterEditionAccountInfo = await provider.connection.getAccountInfo(miner_nft_master_edition_account);
        assert.ok(masterEditionAccountInfo, "NFT master edition account should exist");
        assert.ok(masterEditionAccountInfo.data.length > 0, "NFT master edition should have data");
        console.log("✅ NFT master edition account verified");
  
        console.log("\n=== Miner Onboarding Results ===");
        console.log("Miner Address:        ", miner.publicKey.toString());
        console.log("NFT Mint:            ", miner_nft_mint.toString());
        console.log("NFT Token Account:   ", miner_nft_token_account.toString());
        console.log("BHRT Token Account:  ", miner_bhrt.toString());
        console.log("Miner Info:          ", miner_info.toString());
        console.log("Mining Power:        ", miningPower);
        console.log("BHRT Tokens Minted:  ", expectedBhrtAmount);
        console.log("NFT ID:              ", nftId);
        console.log("================================\n");
  
      } catch (error) {
        console.error("Miner onboarding failed!");
        if (error.logs) {
          console.error("Full logs:");
          for (const log of error.logs) {
            console.log(`- ${log}`);
          }
        }
        throw error;
      }
    });
  

  });


  
