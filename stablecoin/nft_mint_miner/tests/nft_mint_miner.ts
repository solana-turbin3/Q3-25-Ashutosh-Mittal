// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { NftMintMiner } from "../target/types/nft_mint_miner";

// describe("nft_mint_miner", () => {
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.AnchorProvider.env());

//   const program = anchor.workspace.nftMintMiner as Program<NftMintMiner>;

//   it("Is initialized!", async () => {
//     // Add your test here.
//     const tx = await program.methods.initialize().rpc();
//     console.log("Your transaction signature", tx);
//   });
// });


////////////////////////////////////////////////////////////////////////////////////////


// tests/nft-mint-miner.test.ts

import {
  BankrunProvider,
  startAnchor,
} from "anchor-bankrun";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  LAMPORTS_PER_SOL,
  TransactionInstruction,
} from "@solana/web3.js";
import {
  Program,
  BN,
} from "@coral-xyz/anchor";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  getAccount,
  getMint,
} from "@solana/spl-token";
import {
  getMetadataAccount,
  getEditionAccount,
} from "@metaplex-foundation/mpl-token-metadata";
import {
  PROGRAM_ID as METAPLEX_METADATA_PROGRAM_ID
} from "@metaplex-foundation/mpl-token-metadata";

// Import your program's IDL and types
import { NftMintMiner, IDL } from "../target/types/nft_mint_miner"; // Adjust path if necessary

describe("nft_mint_miner", () => {
  let provider: BankrunProvider;
  let program: Program<NftMintMiner>;
  let authority: Keypair; // The overall program authority
  let approvedMiner: Keypair; // An approved miner
  let unapprovedMiner: Keypair; // An unapproved miner for negative testing

  // PDAs for ProgramInfo
  let programInfoPubKey: PublicKey;

  // Define some constant data for the NFT
  const NFT_USER_PROVIDED_NAME = "GPU-Farm-Alpha"; // User-provided part of the name
  const NFT_URI = "https://arweave.net/example-gpu-nft-metadata.json";
  const NFT_PRICE = 0.5; // Example price

  beforeAll(async () => {
    // Start Bankrun Anchor Context
    const context = await startAnchor("programs/nft_mint_miner", [], []); // Path to your program's BPF
    provider = new BankrunProvider(context);
    program = new Program<NftMintMiner>(IDL, program.programId, provider);

    // Create Test Wallets
    authority = provider.wallet.payer; // Use default payer for authority
    approvedMiner = Keypair.generate();
    unapprovedMiner = Keypair.generate();

    // Fund accounts
    await provider.connection.requestAirdrop(approvedMiner.publicKey, 10 * LAMPORTS_PER_SOL);
    await provider.connection.requestAirdrop(unapprovedMiner.publicKey, 10 * LAMPORTS_PER_SOL);

    // Derive program_info PDA
    [programInfoPubKey] = PublicKey.findProgramAddressSync(
      [Buffer.from("program_info")],
      program.programId
    );
  });

  // Test Case 1: Initialize the program
  it("should initialize the program successfully", async () => {
    await program.methods
      .initialize()
      .accounts({
        authority: authority.publicKey,
        programInfo: programInfoPubKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([authority])
      .rpc();

    // Assert program_info account exists and has correct initial state
    const programInfoAccount = await program.account.programInfo.fetch(programInfoPubKey);
    expect(programInfoAccount.authority.toBase58()).toEqual(authority.publicKey.toBase58());
    expect(programInfoAccount.nftIds.toNumber()).toEqual(0);
    expect(programInfoAccount.miners.length).toEqual(0);
  });

  // Test Case 2: Approve a miner
  it("should approve a miner successfully", async () => {
    await program.methods
      .approveMiner(approvedMiner.publicKey)
      .accounts({
        authority: authority.publicKey,
        programInfo: programInfoPubKey,
        systemProgram: SystemProgram.programId, // System program needed for rent
      })
      .signers([authority])
      .rpc();

    // Assert miner is added to the approved miners list
    const programInfoAccount = await program.account.programInfo.fetch(programInfoPubKey);
    expect(programInfoAccount.miners.length).toEqual(1);
    expect(programInfoAccount.miners[0].toBase58()).toEqual(approvedMiner.publicKey.toBase58());
  });

  // Test Case 3: Attempt to approve an already approved miner (Negative Test)
  it("should fail to approve an already approved miner", async () => {
    let errorOccurred = false;
    try {
      await program.methods
        .approveMiner(approvedMiner.publicKey)
        .accounts({
          authority: authority.publicKey,
          programInfo: programInfoPubKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([authority])
        .rpc();
    } catch (e: any) {
      errorOccurred = true;
      expect(e.error.errorMessage).toContain("This miner has already been approved.");
    }
    expect(errorOccurred).toBe(true);
  });

  // Test Case 4: Create a single NFT with an APPROVED miner
  it("should create a single NFT with an approved miner", async () => {
    // Fetch the current nft_ids from program_info
    const initialProgramInfo = await program.account.programInfo.fetch(programInfoPubKey);
    const nextNftId = initialProgramInfo.nftIds.add(new BN(1));

    // Derive PDAs for the new NFT
    const [mintAuthorityPubKey] = PublicKey.findProgramAddressSync(
      [Buffer.from("mint_authority"), nextNftId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );
    const [nftMintPubKey] = PublicKey.findProgramAddressSync(
      [Buffer.from("nft_mint"), nextNftId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );
    const nftTokenAccountPubKey = getAssociatedTokenAddressSync(
      nftMintPubKey,
      approvedMiner.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );
    const [nftMetadataPubKey] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        METAPLEX_METADATA_PROGRAM_ID.toBuffer(),
        nftMintPubKey.toBuffer(),
      ],
      METAPLEX_METADATA_PROGRAM_ID
    );
    const [masterEditionPubKey] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        METAPLEX_METADATA_PROGRAM_ID.toBuffer(),
        nftMintPubKey.toBuffer(),
        Buffer.from("edition"),
      ],
      METAPLEX_METADATA_PROGRAM_ID
    );

    // Call create_single_nft instruction
    await program.methods
      .createSingleNft(
        NFT_USER_PROVIDED_NAME,
        nextNftId.toNumber(), // The instruction takes u64 id but uses nft_ids from program_info
        NFT_URI,
        NFT_PRICE,
      )
      .accounts({
        miner: approvedMiner.publicKey,
        programInfo: programInfoPubKey,
        mintAuthority: mintAuthorityPubKey,
        mint: nftMintPubKey,
        tokenAccount: nftTokenAccountPubKey,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        metadataProgram: METAPLEX_METAPLEX_PROGRAM_ID,
        masterEditionAccount: masterEditionPubKey,
        nftMetadata: nftMetadataPubKey,
      })
      .signers([approvedMiner]) // Only the miner signs
      .rpc();

    // Assertions after NFT creation
    const updatedProgramInfo = await program.account.programInfo.fetch(programInfoPubKey);
    expect(updatedProgramInfo.nftIds.toNumber()).toEqual(nextNftId.toNumber());

    // Verify Mint Account
    const mintAccountInfo = await getMint(provider.connection, nftMintPubKey);
    expect(mintAccountInfo.decimals).toEqual(0);
    expect(mintAccountInfo.supply.toString()).toEqual("1");
    expect(mintAccountInfo.mintAuthority?.toBase58()).toEqual(mintAuthorityPubKey.toBase58());
    expect(mintAccountInfo.freezeAuthority?.toBase58()).toEqual(mintAuthorityPubKey.toBase58());

    // Verify Token Account
    const tokenAccountInfo = await getAccount(provider.connection, nftTokenAccountPubKey);
    expect(tokenAccountInfo.mint.toBase58()).toEqual(nftMintPubKey.toBase58());
    expect(tokenAccountInfo.owner.toBase58()).toEqual(approvedMiner.publicKey.toBase58());
    expect(tokenAccountInfo.amount.toString()).toEqual("1");

    // Verify Metaplex Metadata Account
    const fetchedMetadataAccount = await getMetadataAccount(nftMintPubKey);
    const metadataAccountInfo = await provider.connection.getAccountInfo(fetchedMetadataAccount);
    expect(metadataAccountInfo).not.toBeNull();
    // For deeper metadata checks, you'd parse it:
    // const metadata = MetaplexMetadata.fromAccountInfo(metadataAccountInfo).data;
    // expect(metadata.name).toBe(`Bitcoin Standard Hashrate Token Agreement: ${NFT_USER_PROVIDED_NAME}`);
    // expect(metadata.symbol).toBe("BSHA");
    // expect(metadata.uri).toBe(NFT_URI);

    // Verify Master Edition Account
    const fetchedMasterEditionAccount = await getEditionAccount(nftMintPubKey);
    const masterEditionAccountInfo = await provider.connection.getAccountInfo(fetchedMasterEditionAccount);
    expect(masterEditionAccountInfo).not.toBeNull();
  });

  // Test Case 5: Attempt to create NFT with an UNAPPROVED miner (Negative Test)
  it("should fail to create NFT if miner is not approved", async () => {
    let errorOccurred = false;
    // Attempt to derive PDAs with a dummy next NFT ID for the unapproved miner
    const nextNftIdForUnapproved = new BN(100); // Just a unique ID for this test attempt
    const [dummyMintAuthorityPubKey] = PublicKey.findProgramAddressSync(
      [Buffer.from("mint_authority"), nextNftIdForUnapproved.toArrayLike(Buffer, "le", 8)],
      program.programId
    );
    const [dummyNftMintPubKey] = PublicKey.findProgramAddressSync(
      [Buffer.from("nft_mint"), nextNftIdForUnapproved.toArrayLike(Buffer, "le", 8)],
      program.programId
    );
    const dummyNftTokenAccountPubKey = getAssociatedTokenAddressSync(
      dummyNftMintPubKey,
      unapprovedMiner.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );
    const [dummyNftMetadataPubKey] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        METAPLEX_METAPLEX_PROGRAM_ID.toBuffer(),
        dummyNftMintPubKey.toBuffer(),
      ],
      METAPLEX_METAPLEX_PROGRAM_ID
    );
    const [dummyMasterEditionPubKey] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        METAPLEX_METAPLEX_PROGRAM_ID.toBuffer(),
        dummyNftMintPubKey.toBuffer(),
        Buffer.from("edition"),
      ],
      METAPLEX_METAPLEX_PROGRAM_ID
    );


    try {
      await program.methods
        .createSingleNft(
          "Malicious NFT",
          nextNftIdForUnapproved.toNumber(),
          "https://evil.com/nft.json",
          100.0,
        )
        .accounts({
          miner: unapprovedMiner.publicKey,
          programInfo: programInfoPubKey,
          mintAuthority: dummyMintAuthorityPubKey,
          mint: dummyNftMintPubKey,
          tokenAccount: dummyNftTokenAccountPubKey,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          metadataProgram: METAPLEX_METADATA_PROGRAM_ID,
          masterEditionAccount: dummyMasterEditionPubKey,
          nftMetadata: dummyNftMetadataPubKey,
        })
        .signers([unapprovedMiner])
        .rpc();
    } catch (e: any) {
      errorOccurred = true;
      expect(e.error.errorMessage).toContain("The signer is not an approved miner.");
    }
    expect(errorOccurred).toBe(true);

    // Verify that nft_ids counter did NOT increment
    const finalProgramInfo = await program.account.programInfo.fetch(programInfoPubKey);
    expect(finalProgramInfo.nftIds.toNumber()).toEqual(1); // Should still be 1 from the successful mint
  });
});

// Helper for SYSVAR_RENT_PUBKEY, which is not directly exported by web3.js by name
const SYSVAR_RENT_PUBKEY = new PublicKey("SysvarRent1111111111111111111111111111111");