import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import {
  createMint,
  getAssociatedTokenAddress,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import { NftRenter } from "../target/types/nft_renter";

export const createMockNft = async (
  connection: web3.Connection,
  signer: web3.Keypair
): Promise<{ mint: web3.PublicKey; userTokenAccount: web3.PublicKey }> => {
  // Airdrop
  const keypair = web3.Keypair.generate();
  const sig = await connection.requestAirdrop(
    keypair.publicKey,
    web3.LAMPORTS_PER_SOL * 2
  );
  await connection.confirmTransaction(sig);
  // Create Mint and associate token account
  const mockNftMint = await createMint(
    connection,
    keypair,
    keypair.publicKey,
    keypair.publicKey,
    0
  );

  const tokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    signer,
    mockNftMint,
    signer.publicKey
  );

  await mintTo(
    connection,
    keypair,
    mockNftMint,
    tokenAccount.address,
    keypair,
    1
  );

  return { mint: mockNftMint, userTokenAccount: tokenAccount.address };
};

export const getUserListInfo = (
  program: anchor.Program<NftRenter>,
  userPubkey: web3.PublicKey,
  nftMint: web3.PublicKey
) => {
  const LIST_INFO_SEED = "list_info";
  const [userListInfo, _userListInfoBump] =
    web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode(LIST_INFO_SEED)),
        userPubkey.toBuffer(),
        nftMint.toBuffer(),
      ],
      program.programId
    );
  return userListInfo;
};

export const getProgramPdaInfo = async (
  mint: web3.PublicKey,
  lister: web3.PublicKey,
  userListInfo: web3.PublicKey
) => {
  const userNftAccount = await getAssociatedTokenAddress(mint, lister);

  const pdaNftAccount = await getAssociatedTokenAddress(
    mint,
    userListInfo,
    true
  );

  return { userNftAccount, pdaNftAccount };
};
