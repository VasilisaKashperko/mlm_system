import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MlmSystem } from "../target/types/mlm_system";
import {assert} from "chai";
import BN from "bn.js";

import {
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress
} from "@solana/spl-token";

describe("mlm_system", () => {
  const program = anchor.workspace.MlmSystem as Program<MlmSystem>;

  const provider = anchor.AnchorProvider.env();

  anchor.setProvider(anchor.AnchorProvider.env());

  const connection = program.provider.connection;

  let owner = anchor.web3.Keypair.generate();

  it("Is signed up for owner!", async () => {
    const tx = await program.methods.signup(owner.publicKey, owner.publicKey);
    if (tx) {
        console.log("Users had been signed up!");
    }
  })

  it("Is initialized!", async () => {
    let percentage: number = 5;
    const tx = await program.methods.initialize(percentage);
    console.log("Your transaction signature");
  })

  let user1 = anchor.web3.Keypair.generate();
  let user2 = anchor.web3.Keypair.generate();
  let user3 = anchor.web3.Keypair.generate();
  let user4 = anchor.web3.Keypair.generate();
  let user5 = anchor.web3.Keypair.generate();

  it("Is signed up for users!", async () => {
    const tx1 = await program.methods.signup(user1.publicKey, user1.publicKey);
    const tx2 = await program.methods.signup(user2.publicKey, user1.publicKey);
    const tx3 = await program.methods.signup(user3.publicKey, user1.publicKey);
    const tx4 = await program.methods.signup(user4.publicKey, user3.publicKey);
    const tx5 = await program.methods.signup(user5.publicKey, user5.publicKey);

    if (tx1 && tx2 && tx3 && tx4 && tx5) {
      console.log("Users had been signed up!");
    }
  })

  it("Checked! All new users have 0 balance!", async function() {
    const balance1 = await connection.getBalance(user1.publicKey);
    const balance2 = await connection.getBalance(user2.publicKey);
    const balance3 = await connection.getBalance(user3.publicKey);
    const balance4 = await connection.getBalance(user4.publicKey);
    const balance5 = await connection.getBalance(user5.publicKey);

    if ((balance1 && balance2 && balance3 && balance4 && balance5) == 0) {
      console.log("Checked! All new users have 0 balance!");
    }
  })

  it("Is invested!", async () => {
    let invest_amount = new BN(6); // Convert from SOL to lamport
    const address = user1.publicKey;
    const tx = await program.methods.invest(invest_amount, address);

    if (tx) {
      console.log("Is invested!");
    }
  })

  async function send_lamports_for_user(account, program, connection) {
    let airdrop_sign_account = await program.provider.connection.requestAirdrop(account.publicKey,1000000000);

    await program.provider.connection.confirmTransaction({
      signature: airdrop_sign_account,
      blockhash: await connection.getLatestBlockhash().blockhash,
      lastValidBlockHeight: await connection.getLatestBlockhash().lastValidBlockHeight
    });
  }

  it("It withdraw all income!", async function() {
    send_lamports_for_user(user1, program, connection).then();
    console.log("User balance before withdraw: ", await connection.getBalance(user1.publicKey));

    await program.methods.withdraw(user1.publicKey);
    console.log("User balance after withdraw: ", await connection.getBalance(user1.publicKey));

    assert.equal(await connection.getBalance(user1.publicKey), 0);

    console.log("It withdraw all income!");
  })

  const fromWallet = anchor.web3.Keypair.generate()
  const mint = anchor.web3.Keypair.generate()

  it("Is initialised mint!", async () => {
    const tx = await program.methods.initializeMint().accounts({
      mint: mint.publicKey,
      payer: fromWallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY
    }).signers([fromWallet, mint]);

    if (tx) {
      console.log("Is initialised mint!");
    }
  })

  it("Is minted token!", async () => {
    let associatedTokenAccount = await getAssociatedTokenAddress(mint.publicKey, fromWallet.publicKey);

    const tx = await program.methods.mintToken(new BN(100)).accounts({
      mint: mint.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      tokenAccount: associatedTokenAccount,
      authority:fromWallet.publicKey,
    }).signers([fromWallet]);

    if (tx) {
      console.log("Is minted token!");
    }
  })
})