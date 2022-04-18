import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Testing } from "../target/types/testing";

describe("testing", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Testing as Program<Testing>;

  it("Is initialized!", async () => {
    await program.methods.initializeA().rpc();
    await program.methods.initializeAPubkeys().rpc();
    await program.methods.initializeB().rpc();
    await program.methods.initializeBPubkeys().rpc();
  });
});
