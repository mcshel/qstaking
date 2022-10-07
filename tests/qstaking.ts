import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Qstaking } from "../target/types/qstaking";

describe("qstaking", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Qstaking as Program<Qstaking>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
