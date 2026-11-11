const note = web3.Keypair.generate();

const tx = await pg.program.methods
  .initialize("hello world!")
  .accounts({
    note: note.publicKey,
    signer: pg.wallet.publicKey,
    systemProgram: web3.SystemProgram.programId,
  })
  .signers([note])
  .rpc();

await pg.connection.confirmTransaction(tx, "confirmed");

console.log("Note address:", note.publicKey.toString());
console.log(
  `Explorer: https://explorer.solana.com/address/${note.publicKey}?cluster=devnet`
);

const saved = await pg.program.account.note.fetch(note.publicKey, "confirmed");
console.log("Message:", saved.message);
