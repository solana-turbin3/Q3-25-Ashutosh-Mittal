import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OnchainVerifiedNftCollection } from "../target/types/onchain_verified_nft_collection";
import { PublicKey, Keypair, Transaction } from "@solana/web3.js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint,
  getAssociatedTokenAddressSync,
  createAssociatedTokenAccountInstruction,
  createInitializeMintInstruction,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { assert } from "chai";
import { Metaplex } from "@metaplex-foundation/js";

describe("onchain-verified-nft-collection", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const authority = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.OnchainVerifiedNftCollection as Program<OnchainVerifiedNftCollection>;

  // Store collection details for use across tests
  let collectionMint: PublicKey;
  let collectionMetadata: PublicKey;
  let collectionMasterEdition: PublicKey;
  let collectionPda: PublicKey;
  let collectionTx: string;
  let mintTx: string;

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

  describe("Collection", () => {
    it("Initializes a collection", async () => {
      // Create collection mint
      collectionMint = await createMint(
        provider.connection,
        authority.payer,
        authority.publicKey,
        authority.publicKey,
        0,
        undefined,
        undefined,
        TOKEN_PROGRAM_ID
      );

      // Get collection metadata PDA
      [collectionMetadata] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").toBuffer(),
          collectionMint.toBuffer(),
        ],
        new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
      );

      // Get collection master edition PDA
      [collectionMasterEdition] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").toBuffer(),
          collectionMint.toBuffer(),
          Buffer.from("edition"),
        ],
        new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
      );

      // Get collection PDA
      [collectionPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("collection"), collectionMint.toBuffer()],
        program.programId
      );

      // Create collection token account
      const collectionTokenAccount = await getAssociatedTokenAddressSync(
        collectionMint,
        authority.publicKey,
        true,
        TOKEN_PROGRAM_ID
      );

      // Create the token account if it doesn't exist
      try {
        await provider.connection.getAccountInfo(collectionTokenAccount);
      } catch (error) {
        const createAtaIx = createAssociatedTokenAccountInstruction(
          authority.publicKey,
          collectionTokenAccount,
          authority.publicKey,
          collectionMint,
          TOKEN_PROGRAM_ID
        );
        
        const tx = new Transaction().add(createAtaIx);
        await provider.sendAndConfirm(tx, [authority.payer]);
      }

      console.log("Initializing collection...");
      collectionTx = await program.methods
        .initializeCollection(
          "My Collection",
          "MC",
          "https://arweave.net/your-collection-metadata-uri"
        )
        .accountsPartial({
          authority: authority.publicKey,
          collection: collectionPda,
          collectionMint: collectionMint,
          collectionMetadata: collectionMetadata,
          collectionMasterEdition: collectionMasterEdition,
          collectionTokenAccount: collectionTokenAccount,
          tokenMetadataProgram: new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          sysvarInstructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
        })
        .signers([authority.payer])
        .rpc();

      await provider.connection.confirmTransaction({
        signature: collectionTx,
        blockhash: (await provider.connection.getLatestBlockhash()).blockhash,
        lastValidBlockHeight: (await provider.connection.getLatestBlockhash()).lastValidBlockHeight,
      });
      await new Promise(resolve => setTimeout(resolve, 2000));

      // Verify the collection was initialized correctly
      const collectionAccount = await program.account.collectionState.fetch(collectionPda);
      assert.ok(collectionAccount.authority.equals(authority.publicKey), "Collection authority should match");
      assert.ok(collectionAccount.mint.equals(collectionMint), "Collection mint should match");
      assert.ok(collectionAccount.metadata.equals(collectionMetadata), "Collection metadata should match");
      assert.ok(collectionAccount.masterEdition.equals(collectionMasterEdition), "Collection master edition should match");
      console.log("Collection initialized successfully");
    });

    it("Mints an NFT and adds it to the collection", async () => {
      // Create NFT mint
      const nftMintKeypair = Keypair.generate();
      const nftMint = nftMintKeypair.publicKey;

      // Get NFT metadata PDA
      const [nftMetadata] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").toBuffer(),
          nftMint.toBuffer(),
        ],
        new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
      );

      // Get NFT master edition PDA
      const [nftMasterEdition] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").toBuffer(),
          nftMint.toBuffer(),
          Buffer.from("edition"),
        ],
        new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
      );

      // Create NFT token account
      const nftTokenAccount = await getAssociatedTokenAddressSync(
        nftMint,
        authority.publicKey,
        true,
        TOKEN_PROGRAM_ID
      );

      // Create the token account if it doesn't exist
      try {
        await provider.connection.getAccountInfo(nftTokenAccount);
      } catch (error) {
        const createAtaIx = createAssociatedTokenAccountInstruction(
          authority.publicKey,
          nftTokenAccount,
          authority.publicKey,
          nftMint,
          TOKEN_PROGRAM_ID
        );
        
        const tx = new Transaction().add(createAtaIx);
        await provider.sendAndConfirm(tx, [authority.payer]);
      }

      // Create and initialize the mint account
      const createMintIx = anchor.web3.SystemProgram.createAccount({
        fromPubkey: authority.publicKey,
        newAccountPubkey: nftMint,
        space: 82,
        lamports: await provider.connection.getMinimumBalanceForRentExemption(82),
        programId: TOKEN_PROGRAM_ID,
      });

      const initMintIx = await createInitializeMintInstruction(
        nftMint,
        0,
        authority.publicKey,
        authority.publicKey,
        TOKEN_PROGRAM_ID
      );

      const createMintTx = new Transaction()
        .add(createMintIx)
        .add(initMintIx);
      
      await provider.sendAndConfirm(createMintTx, [nftMintKeypair, authority.payer]);

      // Define account roles
      const accounts = {
        authority: authority.publicKey,
        collection: collectionPda,
        collectionMint: collectionMint,
        collectionMetadata: collectionMetadata,
        collectionMasterEdition: collectionMasterEdition,
        mint: nftMint,
        metadata: nftMetadata,
        masterEdition: nftMasterEdition,
        nftTokenAccount: nftTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        tokenMetadataProgram: new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        sysvarInstructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
      };

      // Verify collection authority
      const collectionAccount = await program.account.collectionState.fetch(collectionPda);
      assert.ok(
        collectionAccount.authority.equals(accounts.authority),
        "Collection authority must match authority"
      );

      console.log("Minting NFT...");
      mintTx = await program.methods
        .mintNft(
          "NFT 1",
          "https://arweave.net/your-nft-metadata-uri"
        )
        .accountsPartial(accounts)
        .preInstructions([
          anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 400000 })
        ])
        .signers([authority.payer, nftMintKeypair])
        .rpc({
          skipPreflight: false,
          commitment: "confirmed",
          maxRetries: 3,
          preflightCommitment: "confirmed"
        });

      await provider.connection.confirmTransaction({
        signature: mintTx,
        blockhash: (await provider.connection.getLatestBlockhash()).blockhash,
        lastValidBlockHeight: (await provider.connection.getLatestBlockhash()).lastValidBlockHeight,
      });
      console.log("NFT minted and added to collection successfully");

      // Fetch on-chain data
      const collectionMetadataData = await provider.connection.getAccountInfo(collectionMetadata);
      const nftMetadataData = await provider.connection.getAccountInfo(nftMetadata);
      
      // Initialize Metaplex
      const metaplex = new Metaplex(provider.connection);
      
      // Get NFT metadata
      const nftMetadataAccount = await metaplex.nfts().findByMint({ mintAddress: nftMint });
      const collectionInfo = nftMetadataAccount.collection;
      
      // Display NFT relationship with on-chain data
      console.log("\n=== NFT Collection Relationship ===");
      console.log("Collection NFT:");
      console.log("├─ Mint:      " + collectionMint.toString());
      console.log("├─ Metadata:  " + collectionMetadata.toString());
      console.log("├─ Edition:   " + collectionMasterEdition.toString());
      console.log("└─ Size:      " + (collectionMetadataData?.data.length || 0) + " bytes");
      console.log("\nMinted NFT:");
      console.log("├─ Mint:      " + nftMint.toString());
      console.log("├─ Metadata:  " + nftMetadata.toString());
      console.log("├─ Edition:   " + nftMasterEdition.toString());
      console.log("├─ Size:      " + (nftMetadataData?.data.length || 0) + " bytes");
      console.log("└─ Collection:");
      console.log("   ├─ Verified: " + (collectionInfo?.verified ? "Yes" : "No"));
      console.log("   └─ Key:      " + (collectionInfo?.address.toString() ?? "None"));
      console.log("\nTransaction Links:");
      console.log("├─ Collection Init: https://solscan.io/tx/" + collectionTx + "?cluster=devnet");
      console.log("└─ NFT Mint:        https://solscan.io/tx/" + mintTx + "?cluster=devnet");
      console.log("================================\n");
    });
  });
}); 