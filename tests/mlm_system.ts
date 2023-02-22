import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MlmSystem } from "../target/types/mlm_system";

describe("mlm_system", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MlmSystem as Program<MlmSystem>;

  let owner = anchor.web3.Keypair.generate();

  it("Is signed up!", async () => {
    const tx1 = await program.methods.signup(owner.publicKey);
    if (tx1) {
        console.log("Users had been signed up!");
    }
  })

  it("Is initialized!", async () => {
    const tx = program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
})