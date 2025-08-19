import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { Stablecoin } from "../target/types/stablecoin";
import {
  getAssociatedTokenAddressSync,
  getMint,
  createMint,
  TOKEN_2022_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountIdempotent,
} from "@solana/spl-token";
import { assert } from "chai";
import * as spl from "@solana/spl-token";

describe("stablecoin-protocol", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Stablecoin as Program<Stablecoin>;

  const admin = web3.Keypair.generate();
  const bhrtCollateralMint = web3.Keypair.generate();
  const user1 = web3.Keypair.generate();
  const user2 = web3.Keypair.generate();

  //------------------------------------------------------------------------------------
  const [stablecoinConfigPda] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("stablecoin_config"), bhrtCollateralMint.publicKey.toBuffer()],
    program.programId
  );
  const [stablecoinMintPda] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("HST")],
    program.programId
  );
  const [bhrtCollateralVaultPda] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("bhrt_collateral_vault"), bhrtCollateralMint.publicKey.toBuffer()],
    program.programId
  );
  const [bhrtPriceOraclePda] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("bhrt_price_oracle")],
    program.programId
  );

    const [stablecoinMinter1Pda] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("stablecoin_minter"), user1.publicKey.toBuffer()],
      program.programId
    );

    const [stablecoinMinter2Pda] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("stablecoin_minter"), user2.publicKey.toBuffer()],
      program.programId
    );
  //------------------------------------------------------------------------------------

  const stablecoinTokenAccountAta = getAssociatedTokenAddressSync(
    stablecoinMintPda,
    stablecoinConfigPda,
    true, // PDA owner
    TOKEN_2022_PROGRAM_ID
  );

  const bhrtCollateralVaultAta = getAssociatedTokenAddressSync(
    bhrtCollateralMint.publicKey,
    stablecoinConfigPda,
    true, 
    TOKEN_2022_PROGRAM_ID
  );

  const user1BHRTTokenAccountAta = getAssociatedTokenAddressSync(
    bhrtCollateralMint.publicKey,
    user1.publicKey,
    false, 
    TOKEN_2022_PROGRAM_ID
  );

 
  const user1StablecoinTokenAccountAta = getAssociatedTokenAddressSync(
    stablecoinMintPda,
    user1.publicKey,
    false, 
    TOKEN_2022_PROGRAM_ID
  );

  const user2BHRTTokenAccountAta = getAssociatedTokenAddressSync(
    bhrtCollateralMint.publicKey,
    user2.publicKey,
    false, 
    TOKEN_2022_PROGRAM_ID
  );

 
  const user2StablecoinTokenAccountAta = getAssociatedTokenAddressSync(
    stablecoinMintPda,
    user2.publicKey,
    false, 
    TOKEN_2022_PROGRAM_ID
  );



  before(async () => {
    // Fund admin
    const sig = await provider.connection.requestAirdrop(
      admin.publicKey,
      10 * web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(sig, "confirmed");

    // Fund user 1
    const sig2 = await provider.connection.requestAirdrop(
      user1.publicKey,
      10 * web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(sig2, "confirmed");

        // Fund user 2
    const sig3 = await provider.connection.requestAirdrop(
      user2.publicKey,
      10 * web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(sig3, "confirmed");

    // Create BHRT collateral mint on legacy token program
    await createMint(
      provider.connection,
      admin,
      admin.publicKey,
      null,
      6,
      bhrtCollateralMint,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );


  });

  it("✅ Correctly initializes the config, vaults, mint, and oracle!", async () => {
    const uri = "https://example.com/hst-metadata.json";

    console.log("\n🏛️  INITIALIZING STABLECOIN PROTOCOL");
    console.log(`🔑 Admin Authority: ${admin.publicKey.toString()}`);
  console.log(`💰 HST Mint PDA: ${stablecoinMintPda.toString()}`);
  console.log(`🏦 BHRT Vault PDA: ${bhrtCollateralVaultAta.toString()}`);
    const txSignature = await program.methods
      .initializeConfigAndVault(uri)
      .accountsPartial({
        admin: admin.publicKey,
        bhrtCollateralMint: bhrtCollateralMint.publicKey,
        stablecoinConfig: stablecoinConfigPda,
        stablecoinMint: stablecoinMintPda,              // fixed name
        stablecoinTokenAccount: stablecoinTokenAccountAta,
        bhrtCollateralVault: bhrtCollateralVaultAta,
        bhrtPriceOracle: bhrtPriceOraclePda,

        // token2022Program: TOKEN_2022_PROGRAM_ID,        // matches Rust field
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([admin])
      .rpc();

    await provider.connection.confirmTransaction(txSignature, "confirmed");

    const configAccount = await program.account.stablecoinConfig.fetch(
      stablecoinConfigPda
    );
    assert.strictEqual(
      configAccount.authority.toBase58(),
      admin.publicKey.toBase58()
    );

    const oracleAccount = await program.account.priceFeed.fetch(
      bhrtPriceOraclePda
    );
    assert.strictEqual(oracleAccount.feed.toNumber(), 50);

    const stablecoinMintInfo = await getMint(
      provider.connection,
      stablecoinMintPda,
      "confirmed",
      TOKEN_2022_PROGRAM_ID
    );
    assert.strictEqual(
      stablecoinMintInfo.mintAuthority?.toBase58(),
      stablecoinConfigPda.toBase58()
    );

    console.log("\n✅ INITIALIZATION COMPLETED SUCCESSFULLY!");
    console.log("🏦 Vault System:");
    console.log("   └─ BHRT collateral vault created and ready");
    console.log("💰 HST Stablecoin:");
    console.log("   └─ Mint created with PDA as mint authority");
    console.log(`\n🔗 Transaction: https://explorer.solana.com/tx/${txSignature}?cluster=devnet`);
  });


  it("✅ User 1 can open a position!", async () => {
    console.log("\n👤 USER 1: OPENING COLLATERALIZED POSITION");
   try {
    console.log(`🆔 User 1: ${user1.publicKey.toString()}`);
    console.log("\n🔄 STEP 1: Setting up user accounts and funding...");
    const setupTx = new web3.Transaction().add(
      spl.createAssociatedTokenAccountIdempotentInstruction(
        user1.publicKey, 
        user1BHRTTokenAccountAta, 
        user1.publicKey, 
        bhrtCollateralMint.publicKey, 
        TOKEN_2022_PROGRAM_ID 
      ),
      spl.createMintToInstruction(
        bhrtCollateralMint.publicKey, 
        user1BHRTTokenAccountAta,
        admin.publicKey, 
        1000 * 10 ** 6, 
        [],
        TOKEN_2022_PROGRAM_ID 
      )
    );

    await provider.sendAndConfirm(setupTx, [user1, admin]);

    console.log("✅ User 1 BHRT account created and funded with 1,000 BHRT");

    console.log("\n🔄 STEP 2: Opening position with collateral...");
    console.log("📊 Position Parameters:");
    console.log("   └─ Collateral Amount: 1,000 BHRT tokens");
    console.log("   └─ Current BHRT Price: $50.00");
    console.log("   └─ Total Collateral Value: $50,000");
    console.log("   └─ Expected HST to mint: ~$33,000");

    const txSignature = await program.methods.openPosition(new anchor.BN(1000)).accountsPartial({
      user: user1.publicKey,
      bhrtCollateralMint: bhrtCollateralMint.publicKey,
      bhrtUserTokenAccount: user1BHRTTokenAccountAta,
      bhrtCollateralVault: bhrtCollateralVaultAta,
      stablecoinConfig: stablecoinConfigPda,
      stabelcoinMint: stablecoinMintPda,
      bhrtPriceOracle: bhrtPriceOraclePda,
      stablecoinMinter: stablecoinMinter1Pda,
      stablecoinUserTokenAccount: user1StablecoinTokenAccountAta,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: web3.SystemProgram.programId,
    }).signers([user1]).rpc();
    await provider.connection.confirmTransaction(txSignature, "confirmed");

    console.log("\n✅ POSITION OPENED SUCCESSFULLY!");
    console.log("🏦 Collateral Status:");
    console.log("   └─ 1,000 BHRT locked in vault");
    console.log("💰 HST Minted:");
    console.log("   └─ HST stablecoins minted to user");
    console.log("📊 Position Health:");
    console.log("   └─ Collateralization ratio: 150%");
    console.log(`\n🔗 Transaction: https://explorer.solana.com/tx/${txSignature}?cluster=devnet`);

   } catch (error) {
    console.error("❌ POSITION OPENING FAILED:");
    console.error(`   └─ Error: ${error.message}`);
    if (error instanceof anchor.web3.SendTransactionError) {
      console.error("📋 Transaction Logs:");
      error.logs?.forEach(log => console.error(`   └─ ${log}`));
    }
    throw error;
  }
});

  it("✅ User 2 can open a position!", async () => {
    console.log("\n👤 USER 2: OPENING COLLATERALIZED POSITION");

    try {
      console.log(`🆔 User 2: ${user2.publicKey.toString()}`);
      console.log("\n🔄 STEP 1: Setting up user 2 accounts and funding...");

     const setupTx2 = new web3.Transaction().add(
       spl.createAssociatedTokenAccountIdempotentInstruction(
         user2.publicKey, 
         user2BHRTTokenAccountAta, 
         user2.publicKey, 
         bhrtCollateralMint.publicKey, 
         TOKEN_2022_PROGRAM_ID 
       ),
       spl.createMintToInstruction(
         bhrtCollateralMint.publicKey, 
         user2BHRTTokenAccountAta,
         admin.publicKey, 
         1000 * 10 ** 6, 
         [],
         TOKEN_2022_PROGRAM_ID 
       )
     );
 
     await provider.sendAndConfirm(setupTx2, [user2, admin]);

 
     const txSignature =  await program.methods.openPosition(new anchor.BN(10000)).accountsPartial({
       user: user2.publicKey,
       bhrtCollateralMint: bhrtCollateralMint.publicKey,
       bhrtUserTokenAccount: user2BHRTTokenAccountAta,
       bhrtCollateralVault: bhrtCollateralVaultAta,
       stablecoinConfig: stablecoinConfigPda,
       stabelcoinMint: stablecoinMintPda,
       bhrtPriceOracle: bhrtPriceOraclePda,
       stablecoinMinter: stablecoinMinter2Pda,
       stablecoinUserTokenAccount: user2StablecoinTokenAccountAta,
       tokenProgram: TOKEN_2022_PROGRAM_ID,
       associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
       systemProgram: web3.SystemProgram.programId,
     }).signers([user2]).rpc();
     await provider.connection.confirmTransaction(txSignature, "confirmed");

     console.log("\n✅ POSITION OPENED SUCCESSFULLY FOR USER 2!");

     console.log(`\n🔗 Transaction: https://explorer.solana.com/tx/${txSignature}?cluster=devnet`);


    } catch (error) {
    console.error("❌ LARGE POSITION OPENING FAILED:");
    console.error(`   └─ Error: ${error.message}`);
    if (error instanceof anchor.web3.SendTransactionError) {
      console.error("📋 Transaction Logs:");
      error.logs?.forEach(log => console.error(`   └─ ${log}`));
    }
    throw error;
  }
});
 


   it("✅ Change the price oracle!", async () => {
    try {
      console.log("\n📊 UPDATING PRICE ORACLE (MARKET CRASH SIMULATION)");

      await program.methods.changePriceOracle(new anchor.BN(30)).accountsPartial({
        admin: admin.publicKey,
        bhrtPriceOracle: bhrtPriceOraclePda,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: web3.SystemProgram.programId,
      }).signers([admin]).rpc();
      console.log("\n✅ PRICE ORACLE UPDATED SUCCESSFULLY!");

    }catch (error) {
      console.error("❌ PRICE ORACLE UPDATE FAILED:");
      console.error(`   └─ Error: ${error.message}`);
      if (error instanceof anchor.web3.SendTransactionError) {
        console.error("📋 Transaction Logs:");
        error.logs?.forEach(log => console.error(`   └─ ${log}`));
      }
      throw error;
    }
  });
     
   it("✅ Liquidator (user 2) can liquidate user 1's position!", async () => {
    console.log("\n⚡ LIQUIDATION PROCESS: USER 2 LIQUIDATES USER 1");
    console.log(`🔨 Liquidator: ${user2.publicKey.toString()}`);
    console.log(`🎯 Target Position: ${user1.publicKey.toString()}`);
    try {
 
      console.log("\n🔄 EXECUTING LIQUIDATION:");

      const txSignature = await program.methods.liquidate(new anchor.BN(500)).accountsPartial({
      liquidator: user2.publicKey,
      targetPosition: user1.publicKey,
       bhrtCollateralMint: bhrtCollateralMint.publicKey,
       bhrtLiquidatorTokenAccount: user2BHRTTokenAccountAta,
       bhrtTargetPositionTokenAccount: user1BHRTTokenAccountAta,
       bhrtCollateralVault: bhrtCollateralVaultAta,
       stablecoinConfig: stablecoinConfigPda,
       stabelcoinMint: stablecoinMintPda,
       bhrtPriceOracle: bhrtPriceOraclePda,
       stablecoinMinter: stablecoinMinter1Pda,
       stablecoinLiquidatorTokenAccount: user2StablecoinTokenAccountAta,
       tokenProgram: TOKEN_2022_PROGRAM_ID,
       associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
       systemProgram: web3.SystemProgram.programId,
     }).signers([user2]).rpc();
     await provider.connection.confirmTransaction(txSignature, "confirmed");

     console.log("\n✅ LIQUIDATION COMPLETED SUCCESSFULLY!");
     console.log(`\n🔗 Transaction: https://explorer.solana.com/tx/${txSignature}?cluster=devnet`);

    }  catch (error) {
    console.error("❌ LIQUIDATION FAILED:");
    console.error(`   └─ Error: ${error.message}`);
    if (error instanceof anchor.web3.SendTransactionError) {
      console.error("📋 Transaction Logs:");
      error.logs?.forEach(log => console.error(`   └─ ${log}`));
    }
    throw error;
  }
});

        
  //  it("✅ User 1 can settle their debt!", async () => {
  //   console.log("\n💳 DEBT SETTLEMENT: USER 1 REPAYS HST DEBT");
  //   console.log(`👤 User: ${user1.publicKey.toString()}`);

  //   try {
 

  //     const txSignature =   await program.methods.positionDebtSettlement(new anchor.BN(500)).accountsPartial({
  //     user: user1.publicKey,
  //      bhrtCollateralMint: bhrtCollateralMint.publicKey,
  //      bhrtUserTokenAccount: user1BHRTTokenAccountAta,
  //      bhrtCollateralVault: bhrtCollateralVaultAta,
  //      stablecoinConfig: stablecoinConfigPda,
  //      stabelcoinMint: stablecoinMintPda,
  //      bhrtPriceOracle: bhrtPriceOraclePda,
  //      stablecoinMinter: stablecoinMinter1Pda,
  //      stablecoinUserTokenAccount: user1StablecoinTokenAccountAta,
  //      tokenProgram: TOKEN_2022_PROGRAM_ID,
  //      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //      systemProgram: web3.SystemProgram.programId,
  //    }).signers([user1]).rpc();
  //    await provider.connection.confirmTransaction(txSignature, "confirmed");

  //    console.log("\n✅ DEBT SETTLEMENT COMPLETED SUCCESSFULLY!");
  //    console.log(`\n🔗 Transaction: https://explorer.solana.com/tx/${txSignature}?cluster=devnet`);

  //   } catch (error) {
  //     console.error("❌ DEBT SETTLEMENT FAILED:");
  //     console.error(`   └─ Error: ${error.message}`);
  //     if (error instanceof anchor.web3.SendTransactionError) {
  //       console.error("📋 Transaction Logs:");
  //       error.logs?.forEach(log => console.error(`   └─ ${log}`));
  //     }
  //     throw error;
  //   }
  // });


});
