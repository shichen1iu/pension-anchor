import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Pension } from "../target/types/pension";

describe("pension", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Pension as Program<Pension>;

  const user = provider.wallet as anchor.Wallet;

  it("SOL Deposited!!", async () => {
    //获取pension_account的PDA地址
    const [pensionPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("doposit"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );
    // Add your test here.
    const tx = await program.methods
      .depositSol(new anchor.BN(1), 1, 5)
      .accounts({
        pension_account: pensionPda,
        user: user.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
