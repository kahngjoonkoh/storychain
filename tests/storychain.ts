import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Storychain } from "../target/types/storychain";

describe("storychain", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Storychain as Program<Storychain>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initializeProgram().rpc();
    console.log("Your transaction signature", tx);
  });
});
