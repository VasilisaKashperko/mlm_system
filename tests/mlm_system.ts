import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MlmSystem } from "../target/types/mlm_system";

describe("mlm_system", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MlmSystem as Program<MlmSystem>;

  let owner = anchor.web3.Keypair.generate();

  it("Is signed up!", async () => {
    const tx = await program.methods.signup(owner.publicKey);
    if (tx) {
        console.log("Users had been signed up!");
    }
  })

  it("Is initialized!", async () => {
    let percentage: number = 5;
    const tx = await program.methods.initialize(5);
    console.log("Your transaction signature", tx);
  });
})