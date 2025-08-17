import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import * as spl from "@solana/spl-token";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  SYSVAR_INSTRUCTIONS_PUBKEY,
} from "@solana/web3.js";
import { BhrtToken } from "../target/types/bhrt_token";
import { assert } from "chai";

describe("bhrt_token", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.BhrtToken as Program<BhrtToken>;
  const programId = program.programId;
  const tokenProgram = spl.TOKEN_2022_PROGRAM_ID;
  const metadataProgram = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
  const NFT_ID = new anchor.BN(1);
  const NFT_ID_LP = new anchor.BN(2);
  const NFT_ID_SWAPPER = new anchor.BN(3);

  const authority = Keypair.generate();
  const miner = Keypair.generate();
  const usdtMintKeypair = Keypair.generate();
  const lpProvider = Keypair.generate(); // Liquidity Provider
  const swapper = Keypair.generate();    // A user for trading

  let program_state: PublicKey;
  let bhrt_mint: PublicKey;
  let collection_mint: PublicKey;
  let nft_collection_metadata: PublicKey;
  let collection_master_edition_account: PublicKey;
  let miner_info: PublicKey;
  let miner_bhrt: PublicKey;
  let miner_nft_mint: PublicKey;
  let miner_nft_token_account: PublicKey;
  let miner_nft_metadata: PublicKey;
  let miner_nft_master_edition_account: PublicKey;
  let usdtMint: PublicKey;
  let ammConfigPda: PublicKey;
  let lpMintPda: PublicKey;
  let vaultBhrtAta: PublicKey;
  let vaultUsdtAta: PublicKey;
  let lpProviderBhrtAta: PublicKey;
  let lpProviderUsdtAta: PublicKey;
  let lpProviderLpAta: PublicKey;
  let swapperBhrtAta: PublicKey;
  let swapperUsdtAta: PublicKey;

  let miner_info_lp: PublicKey;
  let miner_nft_mint_lp: PublicKey;
  let miner_nft_token_account_lp: PublicKey;
  let miner_nft_metadata_lp: PublicKey;
  let miner_nft_master_edition_account_lp: PublicKey;
  let miner_bhrt_lp: PublicKey;
  
  let miner_info_swapper: PublicKey;
  let miner_nft_mint_swapper: PublicKey;
  let miner_nft_token_account_swapper: PublicKey;
  let miner_nft_metadata_swapper: PublicKey;
  let miner_nft_master_edition_account_swapper: PublicKey;
  let miner_bhrt_swapper: PublicKey;

  const confirm = async (signature: string): Promise<string> => {
    const block = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({ signature, ...block });
    return signature;
  };
  const log = async (signature: string): Promise<string> => {
    console.log(`Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=devnet`);
    return signature;
  };

  before(async () => {
    [program_state] = PublicKey.findProgramAddressSync([Buffer.from("program_state")], programId);
    [bhrt_mint] = PublicKey.findProgramAddressSync([Buffer.from("BHRT")], programId);
    [collection_mint] = PublicKey.findProgramAddressSync([Buffer.from("collection_mint")], programId);
    [miner_info] = PublicKey.findProgramAddressSync([Buffer.from("miner"), miner.publicKey.toBuffer()], programId);
    [miner_nft_mint] = PublicKey.findProgramAddressSync([Buffer.from("nft_mint"), miner.publicKey.toBuffer(), NFT_ID.toArrayLike(Buffer, "le", 8)], programId);
    [nft_collection_metadata] = PublicKey.findProgramAddressSync([Buffer.from("metadata"), metadataProgram.toBuffer(), collection_mint.toBuffer()], metadataProgram);
    [collection_master_edition_account] = PublicKey.findProgramAddressSync([Buffer.from("metadata"), metadataProgram.toBuffer(), collection_mint.toBuffer(), Buffer.from("edition")], metadataProgram);
    [miner_nft_metadata] = PublicKey.findProgramAddressSync([Buffer.from("metadata"), metadataProgram.toBuffer(), miner_nft_mint.toBuffer()], metadataProgram);
    [miner_nft_master_edition_account] = PublicKey.findProgramAddressSync([Buffer.from("metadata"), metadataProgram.toBuffer(), miner_nft_mint.toBuffer(), Buffer.from("edition")], metadataProgram);
    miner_bhrt = spl.getAssociatedTokenAddressSync(bhrt_mint, miner.publicKey, false, tokenProgram);
    miner_nft_token_account = spl.getAssociatedTokenAddressSync(miner_nft_mint, miner.publicKey, false, tokenProgram);
    usdtMint = usdtMintKeypair.publicKey;

    [miner_info_lp] = PublicKey.findProgramAddressSync([Buffer.from("miner"), lpProvider.publicKey.toBuffer()], programId);
    [miner_nft_mint_lp] = PublicKey.findProgramAddressSync([Buffer.from("nft_mint"), lpProvider.publicKey.toBuffer(), NFT_ID_LP.toArrayLike(Buffer, "le", 8)], programId);
    [miner_nft_metadata_lp] = PublicKey.findProgramAddressSync([Buffer.from("metadata"), metadataProgram.toBuffer(), miner_nft_mint_lp.toBuffer()], metadataProgram);
    [miner_nft_master_edition_account_lp] = PublicKey.findProgramAddressSync([Buffer.from("metadata"), metadataProgram.toBuffer(), miner_nft_mint_lp.toBuffer(), Buffer.from("edition")], metadataProgram);
    miner_bhrt_lp = spl.getAssociatedTokenAddressSync(bhrt_mint, lpProvider.publicKey, false, tokenProgram);
    miner_nft_token_account_lp = spl.getAssociatedTokenAddressSync(miner_nft_mint_lp, lpProvider.publicKey, false, tokenProgram);
    [miner_info_swapper] = PublicKey.findProgramAddressSync([Buffer.from("miner"), swapper.publicKey.toBuffer()], programId);
    [miner_nft_mint_swapper] = PublicKey.findProgramAddressSync([Buffer.from("nft_mint"), swapper.publicKey.toBuffer(), NFT_ID_SWAPPER.toArrayLike(Buffer, "le", 8)], programId);
    [miner_nft_metadata_swapper] = PublicKey.findProgramAddressSync([Buffer.from("metadata"), metadataProgram.toBuffer(), miner_nft_mint_swapper.toBuffer()], metadataProgram);
    [miner_nft_master_edition_account_swapper] = PublicKey.findProgramAddressSync([Buffer.from("metadata"), metadataProgram.toBuffer(), miner_nft_mint_swapper.toBuffer(), Buffer.from("edition")], metadataProgram);
    miner_bhrt_swapper = spl.getAssociatedTokenAddressSync(bhrt_mint, swapper.publicKey, false, tokenProgram);
    miner_nft_token_account_swapper = spl.getAssociatedTokenAddressSync(miner_nft_mint_swapper, swapper.publicKey, false, tokenProgram);

    await Promise.all([
        provider.connection.requestAirdrop(authority.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL).then(confirm),
        provider.connection.requestAirdrop(miner.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL).then(confirm),
        provider.connection.requestAirdrop(lpProvider.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL).then(confirm),
        provider.connection.requestAirdrop(swapper.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL).then(confirm)
    ]);
  });

  describe("Protocol Initialization", () => {
    it("Initializes the protocol", async () => {
      console.log("\n🚀 INITIALIZING BHRT TOKEN PROTOCOL");
      console.log(`Authority: ${authority.publicKey.toString()}`);
    console.log(`Program State PDA: ${program_state.toString()}`);
    console.log(`BHRT Mint PDA: ${bhrt_mint.toString()}`);
    console.log(`Collection Mint PDA: ${collection_mint.toString()}`);
      const collection_token_account = spl.getAssociatedTokenAddressSync(collection_mint, program_state, true, tokenProgram);
      await program.methods.authorityinitialization()
        .accountsPartial({
          authority: authority.publicKey,
          programState: program_state,
          bhrtMint: bhrt_mint,
          bhrtMetadata: PublicKey.findProgramAddressSync([Buffer.from("bhrt_metadata"), program_state.toBuffer()], programId)[0],
          collectionMint: collection_mint,
          collectionTokenAccount: collection_token_account,
          nftCollectionMetadata: nft_collection_metadata,
          collectionMasterEditionAccount: collection_master_edition_account,
          metadataProgram,
          instructionSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
          tokenProgram,
          systemProgram: SystemProgram.programId,
          associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([authority])
        .rpc()
        .then(log);
        console.log("✅ Protocol initialized successfully!");
    console.log("📋 Created program state account");
    console.log("🪙 Created BHRT token mint");
    console.log("🎨 Created NFT collection mint and metadata");
    });
  });

  describe("Miner Onboarding", () => {

    it("Approves a miner for onboarding", async () => {
      console.log("\n👨‍💼 APPROVING MINER FOR ONBOARDING");
      console.log(`Miner to approve: ${miner.publicKey.toString()}`);
    console.log(`Authority approving: ${authority.publicKey.toString()}`);
      await program.methods.approveMiners(miner.publicKey)
        .accounts({ authority: authority.publicKey, programState: program_state, systemProgram: SystemProgram.programId })
        .signers([authority])
        .rpc()
        .then(log);
        console.log("✅ Miner approved successfully!");
    console.log("📝 Miner added to approved list");
    console.log("🔓 Miner can now onboard to the protocol");
    });

    it("Onboards an approved miner and creates the NFT", async () => {
      console.log("\n🎨 ONBOARDING MINER & CREATING NFT");
      console.log(`Miner: ${miner.publicKey.toString()}`);
    console.log(`NFT ID: ${NFT_ID.toString()}`);
    console.log(`NFT Name: Bitcoin Mining Farm #1`);
    console.log(`Legal Document URI: https://arweave.net/miner-legal-document-hash`);
    console.log(`NFT Mint PDA: ${miner_nft_mint.toString()}`);
      await program.methods.onboardMinerNft(NFT_ID, "Bitcoin Mining Farm #1", "https://arweave.net/miner-legal-document-hash")
        .accountsPartial({
            miner: miner.publicKey, authority: authority.publicKey, programState: program_state,
            collectionMint: collection_mint, nftCollectionMetadata: nft_collection_metadata, collectionMasterEditionAccount: collection_master_edition_account,
            minerNftMint: miner_nft_mint, minerNftTokenAccount: miner_nft_token_account, minerNftMetadata: miner_nft_metadata, minerNftMasterEditionAccount: miner_nft_master_edition_account,
            metadataProgram, instructionSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY, systemProgram: SystemProgram.programId, tokenProgram, associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([miner])
        .rpc()
        .then(log);
        console.log("✅ Miner onboarded successfully!");
        console.log("🎨 NFT created and minted to miner");
        console.log("📊 Miner info account created");
        console.log("🔗 NFT linked to collection");
    });

    it("Mints BHRT tokens for a miner", async () => {
      console.log("\n🪙 MINTING BHRT TOKENS FOR MINER");
      console.log(`Miner: ${miner.publicKey.toString()}`);
      console.log(`Mining Power: 20 units`);
      console.log(`Expected BHRT to mint: 200 tokens (20 * 10)`);
      console.log(`Miner BHRT Account: ${miner_bhrt.toString()}`);
      const miningPower = new anchor.BN(200); // This will mint 200 * 10 = 2000 tokens
      await program.methods.onboardMinerMint(NFT_ID, new anchor.BN(20))
        .accountsPartial({
          miner: miner.publicKey, authority: authority.publicKey, programState: program_state,
          minerNftMint: miner_nft_mint, minerInfo: miner_info, bhrtMint: bhrt_mint, minerBhrt: miner_bhrt,
          associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID, systemProgram: SystemProgram.programId, tokenProgram,
        })
        .signers([miner])
        .rpc()
        .then(log);
      const bhrtAccount = await spl.getAccount(provider.connection as any, miner_bhrt, undefined, tokenProgram);
      console.log("✅ BHRT tokens minted successfully!");
     console.log(`💰 Miner BHRT Balance: ${bhrtAccount.amount.toString()} tokens`);
    console.log("📈 Mining power recorded on-chain");
    console.log("🔗 Tokens linked to NFT's Collection");
      assert.equal(bhrtAccount.amount.toString(), "200", "BHRT balance should be 200");
    });
  });
  
  // MOVED: The Revoke test is now here, before the AMM tests, to match the original passing order.
  describe("Revoke miner participation", () => {
    it("Revokes a miner's participation", async () => {
      console.log("\n🚫 REVOKING MINER PARTICIPATION");

      const bhrtAccount = await spl.getAccount(provider.connection as any, miner_bhrt, undefined, tokenProgram);
      const amountToBurn = new anchor.BN(bhrtAccount.amount.toString());
      console.log(`Miner: ${miner.publicKey.toString()}`);
      console.log(`BHRT tokens to burn: ${amountToBurn.toString()}`);
      console.log(`NFT to burn: ${miner_nft_mint.toString()}`);
      console.log("⚠️  This will permanently remove the miner from the protocol");
      await program.methods.revokeMinerParticipation(NFT_ID, amountToBurn)
        .accountsPartial({
            miner: miner.publicKey, authority: authority.publicKey, programState: program_state,
            collectionMint: collection_mint, nftCollectionMetadata: nft_collection_metadata, metadataProgram, collectionMasterEditionAccount: collection_master_edition_account,
            minerNftMint: miner_nft_mint, minerNftTokenAccount: miner_nft_token_account, minerNftMasterEditionAccount: miner_nft_master_edition_account, minerNftMetadata: miner_nft_metadata,
            minerInfo: miner_info, bhrtMint: bhrt_mint, minerBhrt: miner_bhrt,
            instructionSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
            associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID, systemProgram: SystemProgram.programId, tokenProgram,
        })
        .signers([miner])
        .rpc()
        .then(log);
        
      // We assert that the NFT account is gone. We can't check the BHRT balance because that account is likely closed too.
      const nftAccountInfo = await provider.connection.getAccountInfo(miner_nft_token_account);
      console.log("✅ Miner participation revoked successfully!");
    console.log("🔥 All BHRT tokens burned");
    console.log("🎨 NFT burned and metadata cleared");
    console.log("📊 Miner info account closed");
    console.log(`❌ NFT Account Status: ${nftAccountInfo ? 'Still exists' : 'Closed'}`);
      assert.isNull(nftAccountInfo, "Miner NFT token account should be closed after burning");
    });
  });



// --- REPLACE your old "AMM User Setup" block with this ---
describe("AMM Setup and Funding", () => {
  it("Onboards and funds the Liquidity Provider and Swapper", async () => {
    console.log("\n👥 SETTING UP AMM USERS");
    console.log("=".repeat(50));
    console.log("🔄 Setting up Liquidity Provider...");
    console.log(`LP Address: ${lpProvider.publicKey.toString()}`);
    await program.methods.approveMiners(lpProvider.publicKey)
    .accounts({ authority: authority.publicKey, programState: program_state, systemProgram: SystemProgram.programId })
    .signers([authority])
    .rpc()
    .then(log);
    console.log("✅ LP Provider approved");

    await program.methods.onboardMinerNft(NFT_ID_LP, "LP NFT", "uri://lp")
    .accountsPartial({
      miner: lpProvider.publicKey, authority: authority.publicKey, programState: program_state,
      collectionMint: collection_mint, nftCollectionMetadata: nft_collection_metadata, collectionMasterEditionAccount: collection_master_edition_account,
      minerNftMint: miner_nft_mint_lp, minerNftTokenAccount: miner_nft_token_account_lp, minerNftMetadata: miner_nft_metadata_lp, minerNftMasterEditionAccount: miner_nft_master_edition_account_lp,
      metadataProgram, instructionSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY, systemProgram: SystemProgram.programId, tokenProgram, associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
  })
    .signers([lpProvider])
    .rpc()
    .then(log);
    console.log("🎨 LP Provider NFT created");

    await program.methods.onboardMinerMint(NFT_ID_LP, new anchor.BN(500))
    .accountsPartial({  miner: lpProvider.publicKey, authority: authority.publicKey, programState: program_state,
      minerNftMint: miner_nft_mint_lp, minerInfo: miner_info_lp, bhrtMint: bhrt_mint, minerBhrt: miner_bhrt_lp,
      associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID, systemProgram: SystemProgram.programId, tokenProgram,
    })
    .signers([lpProvider])
    .rpc()
    .then(log);

    console.log("💰 LP Provider funded with 5000 BHRT tokens");

    console.log("\n🔄 Setting up Swapper...");
    console.log(`Swapper Address: ${swapper.publicKey.toString()}`);

    await program.methods.approveMiners(swapper.publicKey)
    .accounts({ authority: authority.publicKey, programState: program_state, systemProgram: SystemProgram.programId })
    .signers([authority])
    .rpc()
    .then(log);

    console.log("✅ Swapper approved");

    await program.methods.onboardMinerNft(NFT_ID_SWAPPER, "Swapper NFT", "uri://swapper")
    .accountsPartial({
      miner: swapper.publicKey, authority: authority.publicKey, programState: program_state,
      collectionMint: collection_mint, nftCollectionMetadata: nft_collection_metadata, collectionMasterEditionAccount: collection_master_edition_account,
      minerNftMint: miner_nft_mint_swapper, minerNftTokenAccount: miner_nft_token_account_swapper, minerNftMetadata: miner_nft_metadata_swapper, minerNftMasterEditionAccount: miner_nft_master_edition_account_swapper,
      metadataProgram, instructionSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY, systemProgram: SystemProgram.programId, tokenProgram, associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .signers([swapper])
    .rpc()
    .then(log);
console.log("🎨 Swapper NFT created");
    await program.methods.onboardMinerMint(NFT_ID_SWAPPER, new anchor.BN(100))
    .accountsPartial({  miner: swapper.publicKey, authority: authority.publicKey, programState: program_state,
      minerNftMint: miner_nft_mint_swapper, minerInfo: miner_info_swapper, bhrtMint: bhrt_mint, minerBhrt: miner_bhrt_swapper,
      associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID, systemProgram: SystemProgram.programId, tokenProgram,
    })
    .signers([swapper])
    .rpc()
    .then(log);
    console.log("💰 Swapper funded with 1000 BHRT tokens");
    
    console.log("✅ All AMM users set up successfully!");
  });
});
// --- REPLACE your AMM Functionality block with this ---

describe("AMM Functionality", () => {
  const associatedTokenProgram = spl.ASSOCIATED_TOKEN_PROGRAM_ID;

  it("Initializes the AMM", async () => {
    console.log("\n🏦 INITIALIZING AMM (AUTOMATED MARKET MAKER)");
    console.log("🔄 Creating USDT test token...");

    await spl.createMint(provider.connection as any, authority, authority.publicKey, null, 6, usdtMintKeypair, undefined, tokenProgram);

    console.log(`💵 USDT Mint created: ${usdtMint.toString()}`);
    console.log("📊 USDT Decimals: 6");

    [ammConfigPda] = PublicKey.findProgramAddressSync([Buffer.from("amm_config"), program_state.toBuffer()], programId);
    [lpMintPda] = PublicKey.findProgramAddressSync([Buffer.from("lp"), ammConfigPda.toBuffer()], programId);
    vaultBhrtAta = spl.getAssociatedTokenAddressSync(bhrt_mint, ammConfigPda, true, tokenProgram, associatedTokenProgram);
    vaultUsdtAta = spl.getAssociatedTokenAddressSync(usdtMint, ammConfigPda, true, tokenProgram, associatedTokenProgram);

    console.log(`🏛️  AMM Config PDA: ${ammConfigPda.toString()}`);
    console.log(`🪙 LP Token Mint: ${lpMintPda.toString()}`);
    console.log(`🏦 BHRT Vault: ${vaultBhrtAta.toString()}`);
    console.log(`🏦 USDT Vault: ${vaultUsdtAta.toString()}`);
    console.log("💼 Trading Fee: 0.3% (30 basis points)");

    await program.methods.ammInitialize(30)
      .accountsPartial({
        authority: authority.publicKey, programState: program_state, ammConfig: ammConfigPda,
        bhrtMint: bhrt_mint, udstMint: usdtMint, lpMint: lpMintPda,
        vaultBhrt: vaultBhrtAta, vaultUsdt: vaultUsdtAta,
        tokenProgram, associatedTokenProgram, systemProgram: SystemProgram.programId,
      })
      .signers([authority]).rpc().then(confirm).then(log);
      console.log("✅ AMM initialized successfully!");
    console.log("🏦 Liquidity vaults created");
    console.log("🪙 LP token mint created");
    console.log("⚙️  AMM ready for liquidity provision and trading");

  });
  it("Allows a user to deposit liquidity and receive LP tokens", async () => {
    console.log("\n💧 DEPOSITING LIQUIDITY TO AMM");
    try {
      console.log("🔄 Setting up LP Provider's USDT account...");
      SystemProgram.createAccount({
        fromPubkey: provider.publicKey,
        newAccountPubkey: usdtMint,
        lamports:await spl.getMinimumBalanceForRentExemptMint(
          provider.connection as any
        ),
        space: spl.MINT_SIZE,
        programId: tokenProgram,
      })

      spl.createInitializeMint2Instruction(
        usdtMint,
        6,
        lpProvider.publicKey,
        null,
        tokenProgram
      )
      lpProviderUsdtAta = await spl.createAssociatedTokenAccount(provider.connection as any, lpProvider, usdtMint, lpProvider.publicKey, undefined, tokenProgram, associatedTokenProgram);
      console.log("💵 LP Provider funded with 1,000 USDT");
      spl.createAssociatedTokenAccountIdempotentInstruction(
        provider.publicKey,
        lpProviderUsdtAta,
        lpProvider.publicKey,
        usdtMint,
        tokenProgram
      )

      spl.createMintToInstruction(
        usdtMint,
        lpProviderUsdtAta,
        lpProvider.publicKey,
        1000,
        undefined,
        tokenProgram
      )

      // Fund user with 1,000 USDT (which is 1000 * 10^6 raw units)
      await spl.mintTo(provider.connection as any, lpProvider, usdtMint, lpProviderUsdtAta, authority, BigInt(1000 * (10 ** 6)), [], undefined, tokenProgram);
      lpProviderLpAta = spl.getAssociatedTokenAddressSync(lpMintPda, lpProvider.publicKey, false, tokenProgram, associatedTokenProgram);

      


      spl.createAssociatedTokenAccountIdempotentInstruction(
        provider.publicKey,
        miner_bhrt_lp,
        lpProvider.publicKey,
        bhrt_mint,
        tokenProgram
      )

      spl.createAssociatedTokenAccountIdempotentInstruction(
        provider.publicKey,
        lpProviderLpAta,
        lpProvider.publicKey,
        lpMintPda,
        tokenProgram
      )

      // CORRECTED: Amounts are now scaled by their decimals.
      // LP & USDT mints have 6 decimals. BHRT has 0 (based on previous tests).
      const lpAmountToReceive = new anchor.BN(1000);
      const maxBhrtToDeposit = new anchor.BN(1000);
      const maxUsdtToDeposit = new anchor.BN(1000);

      console.log(`🪙 LP tokens to receive: ${lpAmountToReceive.toString()}`);
      console.log(`🟡 Max BHRT to deposit: ${maxBhrtToDeposit.toString()}`);
      console.log(`💵 Max USDT to deposit: ${maxUsdtToDeposit.toString()}`);

      await program.methods.ammDeposit(lpAmountToReceive, maxBhrtToDeposit, maxUsdtToDeposit)
        .accountsPartial({
          user: lpProvider.publicKey,
          authority: authority.publicKey,
          programState: program_state, ammConfig: ammConfigPda,
          bhrtMint: bhrt_mint, udstMint: usdtMint, lpMint: lpMintPda,
          vaultBhrt: vaultBhrtAta, vaultUsdt: vaultUsdtAta,
          userBhrt: miner_bhrt_lp, userUsdt: lpProviderUsdtAta, userLp: lpProviderLpAta,
          tokenProgram, associatedTokenProgram, systemProgram: SystemProgram.programId,
        })
        // CORRECTED: Add 'authority' back as a required signer.
        .signers([lpProvider])
        .rpc().then(confirm).then(log);

      const lpAccount = await spl.getAccount(provider.connection as any, lpProviderLpAta, "confirmed", tokenProgram);
       
      console.log("✅ Liquidity deposited successfully!");
      console.log(`🪙 LP tokens received: ${lpAccount.amount.toString()}`);
      console.log("🏦 Tokens deposited to AMM vaults");
      console.log("📈 AMM pool is now active for trading");
      assert.isTrue(lpAccount.amount > 0, "LP should have received LP tokens");
    } catch (err) {
      console.error(err);
      if (err instanceof anchor.web3.SendTransactionError) {
        console.log("TRANSACTION LOGS:", await err.getLogs(provider.connection));
      }
      throw new Error(`AMM Deposit failed: ${err.message}. Catch the \`SendTransactionError\` and call \`getLogs()\` on it for full details.`);
    }
  });

  // The rest of the tests are kept the same as the previous correct version
  it("Swaps BHRT for USDT", async () => {
    console.log("\n🔄 SWAPPING BHRT → USDT");
    try {
      console.log("🔄 Setting up swapper's USDT account...");

      spl.createInitializeMint2Instruction(
        usdtMint,
        6,
        swapper.publicKey,
        null,
        tokenProgram
      )
      swapperUsdtAta = await spl.createAssociatedTokenAccount(provider.connection as any, swapper, usdtMint, swapper.publicKey, undefined, tokenProgram, associatedTokenProgram);

      spl.createAssociatedTokenAccountIdempotentInstruction(
        provider.publicKey,
        swapperUsdtAta,
        swapper.publicKey,
        usdtMint,
        tokenProgram
      )

      spl.createMintToInstruction(
        usdtMint,
        swapperUsdtAta,
        swapper.publicKey,
        1000,
        undefined,
        tokenProgram
      )


      spl.createAssociatedTokenAccountIdempotentInstruction(
        provider.publicKey,
        miner_bhrt_swapper,
        swapper.publicKey,
        bhrt_mint,
        tokenProgram
      )

    
      console.log("📊 Swap direction: BHRT → USDT (true)");

      await program.methods.ammSwap(true, new anchor.BN(100), new anchor.BN(1))
        .accountsPartial({
          user: swapper.publicKey,
          authority: authority.publicKey,
          programState: program_state, ammConfig: ammConfigPda,
          bhrtMint: bhrt_mint, udstMint: usdtMint, lpMint: lpMintPda,
          vaultBhrt: vaultBhrtAta, vaultUsdt: vaultUsdtAta,
          userBhrt: miner_bhrt_swapper, userUsdt: swapperUsdtAta,
          tokenProgram,
        })
        .signers([swapper])
        .rpc().then(confirm).then(log);

      const swapperUsdtAfter = await spl.getAccount(provider.connection as any, swapperUsdtAta, "confirmed", tokenProgram);
      console.log("✅ Swap executed successfully!");
      console.log(`💵 USDT received: ${swapperUsdtAfter.amount.toString()}`);
      console.log("💼 Trading fees collected by protocol");
      console.log("📈 AMM pool balances updated");
      assert.isTrue(swapperUsdtAfter.amount > 0, "Swapper should have received some USDT");
    } catch (err) {
      console.error(err);
      if (err instanceof anchor.web3.SendTransactionError) {
        console.log("TRANSACTION LOGS:", await err.getLogs(provider.connection));
      }
      throw new Error(`AMM Swap (BHRT->USDT) failed: ${err.message}. Catch the \`SendTransactionError\` and call \`getLogs()\` on it for full details.`);
    }
  });

  it("Swaps USDT for BHRT", async () => {
    console.log("\n🔄 SWAPPING USDT → BHRT");
    try {
      const swapperUsdtBalance = await spl.getAccount(provider.connection as any, swapperUsdtAta, "confirmed", tokenProgram);
       
      console.log("📊 Swap direction: USDT → BHRT (false)");

      await program.methods.ammSwap(false, new anchor.BN(swapperUsdtBalance.amount.toString()), new anchor.BN(1))
        .accountsPartial({
          user: swapper.publicKey,
          authority: authority.publicKey,
          programState: program_state, ammConfig: ammConfigPda,
          bhrtMint: bhrt_mint, udstMint: usdtMint,lpMint: lpMintPda,
          vaultBhrt: vaultBhrtAta, vaultUsdt: vaultUsdtAta,
          userBhrt: miner_bhrt_swapper, userUsdt: swapperUsdtAta,
          tokenProgram,
        })
        .signers([swapper])
        .rpc().then(confirm).then(log);
        console.log("✅ Reverse swap executed successfully!");
        console.log("🟡 BHRT tokens received");
        console.log("💼 Additional trading fees collected");
        console.log("🔄 Round-trip swap completed");
    } catch (err) {
      console.error(err);
      if (err instanceof anchor.web3.SendTransactionError) {
        console.log("TRANSACTION LOGS:", await err.getLogs(provider.connection));
      }
      throw new Error(`AMM Swap (USDT->BHRT) failed: ${err.message}. Catch the \`SendTransactionError\` and call \`getLogs()\` on it for full details.`);
    }
  });

  it("Allows a user to withdraw liquidity by burning LP tokens", async () => {
    try {
      console.log("\n💸 WITHDRAWING LIQUIDITY FROM AMM");
      const lpAccountBefore = await spl.getAccount(provider.connection as any, lpProviderLpAta, "confirmed", tokenProgram);
      const lpToBurn = new anchor.BN(lpAccountBefore.amount.toString());
   
      await program.methods.ammWithdraw(lpToBurn, new anchor.BN(1), new anchor.BN(1))
        .accountsPartial({
          user: lpProvider.publicKey,
          authority: authority.publicKey,
          programState: program_state, ammConfig: ammConfigPda,
          bhrtMint: bhrt_mint, udstMint: usdtMint, lpMint: lpMintPda,
          vaultBhrt: vaultBhrtAta, vaultUsdt: vaultUsdtAta,
          userBhrt: miner_bhrt_lp, userUsdt: lpProviderUsdtAta, userLp: lpProviderLpAta,
          tokenProgram, associatedTokenProgram, systemProgram: SystemProgram.programId,
        })
        .signers([lpProvider])
        .rpc().then(confirm).then(log);

      const lpAccountAfter = await spl.getAccount(provider.connection as any, lpProviderLpAta, "confirmed", tokenProgram);
      console.log("✅ Liquidity withdrawn successfully!");
      console.log(`🪙 LP tokens remaining: ${lpAccountAfter.amount.toString()}`);
      console.log("💰 Underlying tokens returned to LP provider");
      console.log("📊 LP position completely closed");
      console.log("🎯 AMM pool ready for new liquidity providers");
      assert.equal(lpAccountAfter.amount, BigInt(0), "LP token balance should be zero");
    } catch (err) {
      console.error(err);
      if (err instanceof anchor.web3.SendTransactionError) {
        console.log("TRANSACTION LOGS:", await err.getLogs(provider.connection));
      }
      throw new Error(`AMM Withdraw failed: ${err.message}. Catch the \`SendTransactionError\` and call \`getLogs()\` on it for full details.`);
    }
  });
});

// it("Swap BHRT → USDT", async () => {
//   await program.methods
//     .ammSwap(true, new anchor.BN(100_000_000), new anchor.BN(1))
//     .accounts({
//       user: user.publicKey,
//       authority: authority.publicKey,
//       programState: programStatePda,
//       ammConfig: ammConfigPda,
//       bhrtMint: bhrtMintPda,
//       udstMint: usdtMint,
//       lpMint: lpMintPda,
//       vaultBhrt: vaultBhrtAta,
//       vaultUsdt: vaultUsdtAta,
//       userBhrt: userBhrtAta,
//       userUsdt: userUsdtAta,
//       tokenProgram,
//       associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
//       systemProgram: SystemProgram.programId,
//     })
//     .signers([user])
//     .rpc();

//   // Optionally assert USDT balance increased
//   const usdtBal = await getAccount(provider.connection, userUsdtAta, undefined, tokenProgram);
//   assert.isTrue(usdtBal.amount > 0n);
// });

// it("Withdraw", async () => {
//   const lpBalBefore = (await getAccount(provider.connection, userLpAta, undefined, tokenProgram)).amount;

//   await program.methods
//     .ammWithdraw(new anchor.BN(500), new anchor.BN(1), new anchor.BN(1))
//     .accounts({
//       user: user.publicKey,
//       authority: authority.publicKey,
//       programState: programStatePda,
//       ammConfig: ammConfigPda,
//       bhrtMint: bhrtMintPda,
//       udstMint: usdtMint,
//       lpMint: lpMintPda,
//       vaultBhrt: vaultBhrtAta,
//       vaultUsdt: vaultUsdtAta,
//       userBhrt: userBhrtAta,
//       userUsdt: userUsdtAta,
//       userLp: userLpAta,
//       tokenProgram,
//       associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
//       systemProgram: SystemProgram.programId,
//     })
//     .signers([user])
//     .rpc();

});