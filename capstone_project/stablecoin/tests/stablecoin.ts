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

    console.log(`✅ Initialization successful! tx: ${txSignature}`);
  });


  it("✅ User 1 can open a position!", async () => {
   try {

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

    await program.methods.openPosition(new anchor.BN(1000)).accountsPartial({
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
   } catch (error) {
    console.error("Error during initialization:", error);
    if (error instanceof anchor.web3.SendTransactionError) {
      console.error("Transaction Logs:", error.logs);
    }
    throw error;
  }
 

  });

  it("✅ User 2 can open a position!", async () => {
    try {
 
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
 
     await program.methods.openPosition(new anchor.BN(10000)).accountsPartial({
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
    } catch (error) {
     console.error("Error during initialization:", error);
     if (error instanceof anchor.web3.SendTransactionError) {
       console.error("Transaction Logs:", error.logs);
     }
     throw error;
   }
  
 
   });
 


   it("✅ Change the price oracle!", async () => {
    try {
      await program.methods.changePriceOracle(new anchor.BN(30)).accountsPartial({
        admin: admin.publicKey,
        bhrtPriceOracle: bhrtPriceOraclePda,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: web3.SystemProgram.programId,
      }).signers([admin]).rpc();
    } catch (error) {
      console.error("Error during initialization:", error);
      if (error instanceof anchor.web3.SendTransactionError) {
        console.error("Transaction Logs:", error.logs);
      }
      throw error;
    }
   });
     
   it("✅ Liquidator (user 2) can liquidate user 1's position!", async () => {
    try {
 
 
     await program.methods.liquidate(new anchor.BN(500)).accountsPartial({
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
    } catch (error) {
     console.error("Error during initialization:", error);
     if (error instanceof anchor.web3.SendTransactionError) {
       console.error("Transaction Logs:", error.logs);
     }
     throw error;
   }
  
 
   });


        
   it("✅ User 1 can settle their debt!", async () => {
    try {
 

     await program.methods.positionDebtSettlement(new anchor.BN(500)).accountsPartial({
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
    } catch (error) {
     console.error("Error during initialization:", error);
     if (error instanceof anchor.web3.SendTransactionError) {
       console.error("Transaction Logs:", error.logs);
     }
     throw error;
   }
  
 
   });


});
