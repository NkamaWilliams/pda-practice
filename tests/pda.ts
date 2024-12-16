import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { getAccount, getAssociatedTokenAddress, TOKEN_PROGRAM_ID } from "@solana/spl-token"
import { Pda } from "../target/types/pda";
import { assert, expect } from "chai";

describe("pda", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.Pda as Program<Pda>;
  const movieDetails = {
    title: "The One Above",
    description: "I am the one looking down at you",
    rating: 5
  }
  const movieUpdate = {
    ...movieDetails,
    rating: 4
  }
  const [moviePda, bump] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("review"), provider.publicKey.toBuffer(), Buffer.from(movieDetails.title)], program.programId)
  const [mintPda, mint_bump] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("mint")], program.programId)

  it("Mint created!", async () => {
    const tx = await program.methods.initTokenMint().accountsPartial({
      mint: mintPda,
      tokenProgram: TOKEN_PROGRAM_ID
    }).rpc();

    console.log("Your transaction signature", tx);
  })

  it("Movie review created!", async () => {
    // Add your test here.
    const tokenAccount = await getAssociatedTokenAddress(mintPda, provider.publicKey)
    const tx = await program.methods
      .initialize(movieDetails.title, movieDetails.description, movieDetails.rating)
      .accountsPartial({
        movieReview: moviePda,
        mint: mintPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        tokenAccount
      })
      .rpc();

    const movie = await program.account.movieAccountState.fetch(moviePda);
    expect(movie.title == movieDetails.title)
    expect(movie.description == movieDetails.description)
    expect(movie.rating == movieDetails.rating)
    expect(movie.reviewer == provider.publicKey)

    const userAta = await getAccount(provider.connection, tokenAccount);
    expect(Number(userAta.amount)).to.equal(10 * (10 ^ 6));
    console.log("Your transaction signature", tx);
  });

  it("Movie Review Updated", async () => {
    const tx = await program.methods.update(movieUpdate.title, movieUpdate.description, movieUpdate.rating)
      .accountsPartial({
        movieReview: moviePda
      }).rpc()
    const movie = await program.account.movieAccountState.fetch(moviePda);

    expect(movie.title == movieUpdate.title)
    expect(movie.description == movieUpdate.description)
    expect(movie.rating == movieUpdate.rating)

    console.log("Your transaction signature", tx)
  })

  it("Delete Movie Review", async () => {
    const tx = await program.methods.delete(movieDetails.title).rpc()

    const accounts = await program.account.movieAccountState.all()
    expect(accounts.length == 0)
  })
});
