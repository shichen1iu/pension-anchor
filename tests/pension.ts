import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Pension } from "../target/types/pension";
import { assert } from "chai";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint,
  createAssociatedTokenAccount,
  mintTo,
  getAssociatedTokenAddress,
  getAccount,
} from "@solana/spl-token";

describe("pension", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Pension as Program<Pension>;
  const user = provider.wallet as anchor.Wallet;

  let usdcMint: anchor.web3.PublicKey;
  let userUsdcAccount: anchor.web3.PublicKey;
  let pensionTokenAccount: anchor.web3.PublicKey;
  let pensionUserInfoPda: anchor.web3.PublicKey;
  let pensionSolPda: anchor.web3.PublicKey;

  before(async () => {
    // Create a mock USDC token
    usdcMint = await createMint(
      provider.connection,
      user.payer,
      user.publicKey,
      null,
      6
    );

    userUsdcAccount = await createAssociatedTokenAccount(
      provider.connection,
      user.payer,
      usdcMint,
      user.publicKey
    );

    // Mint some USDC to the user
    await mintTo(
      provider.connection,
      user.payer,
      usdcMint,
      userUsdcAccount,
      user.payer,
      10_000_000_000 // 1000 USDC
    );

    [pensionUserInfoPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("pension_userinfo"), user.publicKey.toBuffer()],
      program.programId
    );

    [pensionSolPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("pension_sol"), user.publicKey.toBuffer()],
      program.programId
    );

    pensionTokenAccount = await getAssociatedTokenAddress(
      usdcMint,
      user.publicKey,
      true
    );
  });

  it("Initialize SOL pension account", async () => {
    //一次存1sol,存5年
    const tx = await program.methods
      .initializeSol(new anchor.BN(1), 5)
      .accounts({
        pensionAccount: pensionSolPda,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const pensionAccount = await program.account.pension.fetch(pensionSolPda);
    // console.log("当前Amount为:  " + pensionAccount.expectedAmount);
    assert.equal(pensionAccount.expectedAmount.toString(), "1");
    assert.equal(pensionAccount.expectedYear, 5);
    assert.equal(pensionAccount.amount.toString(), "1");
  });

  it("Initialize USDC pension account", async () => {
    //一次存10usdc,存5年
    const tx = await program.methods
      .initializeToken(new anchor.BN(10), 5)
      .accounts({
        userTokenAccount: userUsdcAccount,
        user: user.publicKey,
        usdcUsdtMint: usdcMint,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .rpc();

    const pensionUserInfo = await program.account.pension.fetch(
      pensionUserInfoPda
    );
    assert.equal(pensionUserInfo.expectedAmount.toString(), "10");
    assert.equal(pensionUserInfo.expectedYear, 5);
    assert.equal(pensionUserInfo.amount.toString(), "10");
  });

  it("Deposit SOL", async () => {
    try {
      const tx = await program.methods
        .depositSol()
        .accounts({
          pensionAccount: pensionSolPda,
          user: user.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
    } catch (error) {
      console.log("Tips: " + error.message);
    }
    const pensionAccount = await program.account.pension.fetch(pensionSolPda);
    assert.equal(pensionAccount.amount.toString(), "2");
  });

  it("Deposit USDC", async () => {
    // Wait for cooldown (in real tests, you'd need to simulate time passing)
    await new Promise((resolve) => setTimeout(resolve, 1000));

    try {
      const tx = await program.methods
        .depositToken()
        .accounts({
          userTokenAccount: userUsdcAccount,
          pensionTokenAccount: pensionTokenAccount,
          pensionUserInfo: pensionUserInfoPda,
          user: user.publicKey,
          usdcUsdtMint: usdcMint,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();
    } catch (error) {
      console.log("Tips: " + error.message);
    }

    // 获取 pensionTokenAccount 的信息
    const pensionTokenAccountInfo = await getAccount(
      provider.connection,
      pensionTokenAccount
    );
    console.log(
      "当前 pensionTokenAccount 的 Amount:",
      pensionTokenAccountInfo.amount.toString()
    );

    const pensionUserInfo = await program.account.pension.fetch(
      pensionUserInfoPda
    );
    console.log("当前PensionUserInfoAmount为:  " + pensionUserInfo.amount);
    assert.equal(pensionUserInfo.amount.toString(), "20");
  });

  it("Check SOL account", async () => {
    // Wait for cooldown (in real tests, you'd need to simulate time passing)
    await new Promise((resolve) => setTimeout(resolve, 1000));

    try {
      await program.methods
        .checkSolAccount()
        .accounts({
          pensionAccount: pensionSolPda,
          user: user.publicKey,
          userSolWallet: user.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
    } catch (error) {
      console.log("Tips: " + error.message);
    }
  });

  it("Check USDC account", async () => {
    // Wait for cooldown (in real tests, you'd need to simulate time passing)
    await new Promise((resolve) => setTimeout(resolve, 1000));

    try {
      await program.methods
        .checkTokenAccount()
        .accounts({
          pensionTokenAccount: pensionTokenAccount,
          userTokenAccount: userUsdcAccount,
          pensionUserInfo: pensionUserInfoPda,
          user: user.publicKey,
          usdcUsdtMint: usdcMint,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();
    } catch (error) {
      console.log("Tips: " + error.message);
    }
  });

  it("Close SOL account", async () => {
    const tx = await program.methods
      .closeSolAccount()
      .accounts({
        pensionAccount: pensionSolPda,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    try {
      await program.account.pension.fetch(pensionSolPda);
      assert.fail("Pension SOL account should be closed");
    } catch (error) {
      assert.include(error.message, "Account does not exist");
    }
  });

    it("Close USDC account", async () => {
      const accountInfo = await provider.connection.getTokenAccountBalance(
        pensionTokenAccount
      );
      console.log(
        "Pension Token Account balance before closing:",
        accountInfo.value.uiAmount
      );

      const tx = await program.methods
        .closeTokenAccount()
        .accounts({
          pensionTokenAccount: pensionTokenAccount,
          userTokenAccount: userUsdcAccount,
          user: user.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();
      const accountInfo2 = await provider.connection.getTokenAccountBalance(
        pensionTokenAccount
      );
      console.log(
        "Pension Token Account balance after closing:",
        accountInfo.value.uiAmount
      );
    });
});
