import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Pension } from "../target/types/pension";
import { assert } from "chai";
import { createMintToInstruction } from "@solana/spl-token";

describe("pension", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Pension as Program<Pension>;

  const user = provider.wallet as anchor.Wallet;

  it("SOL Initialized!!", async () => {
    //获取pension_account的PDA地址
    const [pensionPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("doposit"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );
    // Add your test here.
    const tx = await program.methods
      .initializeSol(new anchor.BN(1), 1, 5)
      .accounts({
        pensionAccount: pensionPda,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const pensionAccount = await program.account.pension.fetch(pensionPda);
    // Assert the account data
    assert.ok(pensionAccount.expectedLamports === 1);
    assert.equal(pensionAccount.expectedYear, 5);
    assert.equal(pensionAccount.expectedLamports, 1);

    console.log("Your transaction signature", tx);
  });

  it("USDC Initialized!!", async () => {
    const USDC = "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU";
    const tx = await program.methods
      .initializeUsdc(new anchor.BN(1000), 1, 5)
      .accounts({
        userUsdcTokenAccount: userUsdcTokenAccount.publicKey,
        user: user.publicKey,
        usdc: new anchor.web3.PublicKey(USDC),
        tokenProgram: anchor.web3.TokenProgram.programId,
        systemProgram: anchor.web3.SystemProgram.programId,
        associatedTokenProgram: anchor.web3.AssociatedTokenProgram.programId,
      })
      .rpc();
  });
});
