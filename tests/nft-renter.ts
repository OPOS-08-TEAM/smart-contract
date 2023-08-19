import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftRenter } from "../target/types/nft_renter";
import * as web3 from "@solana/web3.js";
import { createMockNft, getProgramPdaInfo, getUserListInfo } from "./utils";
import { expect } from "chai";

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
  const NFT_PRICE = 10;
  // Renter infor
  let renter = web3.Keypair.generate();

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

  it("[1]: Is initialized!", async () => {
    const tx = await program.methods.initializeInstruction().rpc();
  });

  it("[2]: List NFT - successfully", async () => {
    const accounts = {
      userNftAccount: userTokenAccount,
      pdaNftAccount,
      mint,
      listInfo: userListInfo,
    };
    const nftPrice = new anchor.BN(NFT_PRICE);
    await program.methods.listNftInstruction(nftPrice).accounts(accounts).rpc();
    // State modification
    const userListInfoState = await program.account.userListInfo.fetch(
      userListInfo
    );
    expect(userListInfoState.amount.toString()).to.equal(nftPrice.toString());
    expect(userListInfoState.lister.toBase58()).to.equal(
      wallet.publicKey.toBase58()
    );
    expect(userListInfoState.owner.toBase58()).to.equal(
      wallet.publicKey.toBase58()
    );
    expect(userListInfoState.mint.toBase58()).to.equal(mint.toBase58());
  });

  it("[3]: Delist NFT - successfully", async () => {
    const accounts = {
      mint,
      userNftAccount: userTokenAccount,
      pdaNftAccount,
      listInfo: userListInfo,
    };
    await program.methods.delistNftInstruction().accounts(accounts).rpc();
  });

  it("[4]: Rent NFT - successfully", async () => {
    const accounts = {
      userNftAccount: userTokenAccount,
      pdaNftAccount,
      mint,
      listInfo: userListInfo,
    };
    const nftPrice = new anchor.BN(NFT_PRICE);
    await program.methods.listNftInstruction(nftPrice).accounts(accounts).rpc();
  });
});
