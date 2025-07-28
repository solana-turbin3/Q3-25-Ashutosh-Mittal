import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import * as spl from "@solana/spl-token";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram
} from "@solana/web3.js";
import { NftMintMiner } from "../target/types/nft_mint_miner";


describe("escrow", () => {
  const provider = anchor.getProvider();
  const program = anchor.workspace.escrow as Program<NftMintMiner>;
  const programId = program.programId;
  const tokenProgram = spl.TOKEN_PROGRAM_ID;

  console.log("RPC:", provider.connection.rpcEndpoint);

  




})