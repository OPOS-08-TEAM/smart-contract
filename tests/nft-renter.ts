import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftRenter } from "../target/types/nft_renter";
import * as web3 from "@solana/web3.js";
import { createMockNft, getProgramPdaInfo, getUserListInfo } from "./utils";

describe("nft-renter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.NftRenter as Program<NftRenter>;
  const wallet = anchor.workspace.NftRenter.provider.wallet;
  let mint: web3.PublicKey;
  let userTokenAccount: web3.PublicKey;
  let userListInfo: web3.PublicKey;
  let pdaNftAccount: web3.PublicKey;
  // let userInfo: web3.PublicKey;

  before(async () => {
    const nftMockResult = await createMockNft(
      program.provider.connection,
      wallet.payer
    );
    mint = nftMockResult.mint;
    userTokenAccount = nftMockResult.userTokenAccount;
    userListInfo = getUserListInfo(program, wallet.publicKey, mint);
    const pdaNftAccountResult = await getProgramPdaInfo(
      mint,
      wallet.publicKey,
      userListInfo
    );
    pdaNftAccount = pdaNftAccountResult.pdaNftAccount;
  });

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initializeInstruction().rpc();
    // console.log("Your transaction signature", tx);
    // console.log(JSON.stringify(wallet));
    // console.dir(wallet);
  });

  it("List NFT", async () => {
    const accounts = {
      userNftAccount: userTokenAccount,
      pdaNftAccount,
      mint,
      listInfo: userListInfo,
    };
    await program.methods.listNftInstruction().accounts(accounts).rpc();
  });

  it("Delist NFT", async () => {
    const accounts = {
      mint,
      userNftAccount: userTokenAccount,
      pdaNftAccount,
      listInfo: userListInfo,
    };
    await program.methods.delistNftInstruction().accounts(accounts).rpc();
  });
});
