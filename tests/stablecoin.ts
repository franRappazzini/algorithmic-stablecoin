import * as anchor from "@coral-xyz/anchor";

import { Program } from "@coral-xyz/anchor";
import { PythSolanaReceiver } from "@pythnetwork/pyth-solana-receiver";
import { Stablecoin } from "../target/types/stablecoin";
import { expect } from "chai";

/* 

  1 SOL = 150 USD when this test was written.

*/

describe("stablecoin", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  const wallet = provider.wallet as anchor.Wallet;

  anchor.setProvider(provider);

  const program = anchor.workspace.Stablecoin as Program<Stablecoin>;

  const pythSolanaReceiver = new PythSolanaReceiver({ connection, wallet });

  const SOL_USD_FEED_ID = "ef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
  const solUsdPriceFeedAccount = pythSolanaReceiver
    .getPriceFeedAccountAddress(0, SOL_USD_FEED_ID)
    .toBase58();

  const [collateralAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("collateral"), wallet.publicKey.toBuffer()],
    program.programId
  );

  it("Should initialize Config account!", async () => {
    const tx = await program.methods.initializeConfig().accounts({}).rpc({ skipPreflight: true });
    console.log("Initialize Config account tx signature:", tx);
  });

  it("Should deposit collateral and mint stablecoin", async () => {
    const collateralSolAmount = 100_000_000_000; // 100 SOL
    const stablecoinToMint = 1_000_000_000_000; // 1000 stablecoin

    const tx = await program.methods
      .depositAndMint(bn(collateralSolAmount), bn(stablecoinToMint))
      .accounts({ priceUpdate: solUsdPriceFeedAccount })
      .rpc({ skipPreflight: true });

    console.log("Deposit and mint tx signature:", tx);
  });

  it("Should redeem collateral and burn stablecoin", async () => {
    const redeemableSolAmount = 50_000_000_000; // 50 SOL
    const stablecoinToBurn = 500_000_000_000; // 500 stablecoin

    const tx = await program.methods
      .redeemAndBurn(bn(redeemableSolAmount), bn(stablecoinToBurn))
      .accounts({
        priceUpdate: solUsdPriceFeedAccount,
      })
      .rpc({ skipPreflight: true });

    console.log("Redeem and burn tx signature:", tx);
  });

  it("Should update Config account", async () => {
    const healthFactor = 10;
    const liquidationThreshold = 49;
    const liquidationBonus = 9;

    const tx = await program.methods
      .updateConfig(bn(healthFactor), bn(liquidationThreshold), bn(liquidationBonus))
      .accounts({})
      .rpc({ skipPreflight: true });

    console.log("Update Config account tx signature:", tx);

    const configAccount = await program.account.config.fetch(program.programId);
    console.log("Config account:", configAccount);

    expect(configAccount.healthFactor.toNumber()).to.equal(healthFactor);
    expect(configAccount.liquidationThreshold.toNumber()).to.equal(liquidationThreshold);
    expect(configAccount.liquidationBonus.toNumber()).to.equal(liquidationBonus);
  });

  it("Should liquidate collateral", async () => {
    const liquidableAmount = 50_000_000_000; // 50 stablecoin

    const tx = await program.methods
      .liquidate(bn(liquidableAmount))
      .accounts({
        collateral: collateralAccount,
        priceUpdate: solUsdPriceFeedAccount,
      })
      .rpc({ skipPreflight: true });

    console.log("Liquidate tx signature:", tx);
  });

  it("Should update Config account again", async () => {
    const healthFactor = 1;
    const liquidationThreshold = 50;
    const liquidationBonus = 10;

    const tx = await program.methods
      .updateConfig(bn(healthFactor), bn(liquidationThreshold), bn(liquidationBonus))
      .accounts({})
      .rpc({ skipPreflight: true });

    console.log("Update Config account again tx signature:", tx);

    const configAccount = await program.account.config.fetch(program.programId);
    console.log("Config account:", configAccount);

    expect(configAccount.healthFactor.toNumber()).to.equal(healthFactor);
    expect(configAccount.liquidationThreshold.toNumber()).to.equal(liquidationThreshold);
    expect(configAccount.liquidationBonus.toNumber()).to.equal(liquidationBonus);
  });
});

function bn(n: number) {
  return new anchor.BN(n);
}
