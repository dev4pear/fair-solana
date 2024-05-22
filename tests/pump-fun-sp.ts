import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PumpFunSp } from "../target/types/pump_fun_sp";
import { BN } from "bn.js";
import { Keypair, PublicKey } from "@solana/web3.js";
import { getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

describe("pump-fun-sp", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();

  const connection = provider.connection;

  anchor.setProvider(provider);

  const program = anchor.workspace.PumpFunSp as Program<PumpFunSp>;

  const owner = provider.wallet as NodeWallet;



  it("Create & Mint token!", async () => {
    // Add your test here.
    const mint = Keypair.generate();
    const userToken = getAssociatedTokenAddressSync(
      mint.publicKey,
      owner.publicKey
    )
    const tx = await program.methods.createToken(6, new BN(100 * 10 ** 6)).accounts({
      mint: mint.publicKey,
      feeAcc: new PublicKey("JCsE6i2uDDiTkjPozHpQakp4CceHahXMWEWVcdfoEnC8"),
      userToken
    }).signers([mint]).rpc().catch(e => console.log(e));
    console.log("Your transaction signature", tx);
  });
});
