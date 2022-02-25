const anchor = require("@project-serum/anchor");
const assert = require("assert");

describe('token_program1', () => {

  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.TokenProgram1;
  const phantomAcc = new anchor.web3.PublicKey("GwLPwf7zLxyDEotinzBEpsy1krdv165AtpkGfAmg3fVP");
  const {SystemProgram} = anchor.web3


  let mint = null;
  let from = null;
  let to = null;

  it("Initializes test state", async () => {
    mint = await createMint(provider);
    console.log(mint.toBase58());
    from = await createTokenAccount(provider, mint, provider.wallet.publicKey);
    to = await createTokenAccount(provider, mint, provider.wallet.publicKey);
  });

  it("Mints a token", async () => {
    await program.rpc.proxyMintTo(new anchor.BN(1000), {
      accounts: {
        authority: provider.wallet.publicKey,
        mint,
        to: from,
        tokenProgram: TokenInstructions.TOKEN_PROGRAM_ID,
      },
    });

    const fromAccount = await getTokenAccount(provider, from);
    assert.ok(fromAccount.amount.eq(new anchor.BN(1000)));

  });

  it("Transfers a token", async () => {
    
    const tx = await program.rpc.proxyTransfer(new anchor.BN(400), {
      accounts: {
        authority: provider.wallet.publicKey,
        to,
        from,
        mint,
        tokenProgram: TokenInstructions.TOKEN_PROGRAM_ID,
      },
    });

    console.log("Transaction hash:",tx);

    const fromAccount = await getTokenAccount(provider, from);
    const toAccount = await getTokenAccount(provider, to);

    console.log(fromAccount.amount.toString())
    console.log(toAccount.amount.toString())
    // assert.ok(fromAccount.amount.eq(new anchor.BN(600)));
    // assert.ok(toAccount.amount.eq(new anchor.BN(400)));
  });

  it("Burns a token", async () => {

    await program.rpc.proxyBurn(new anchor.BN(350), {
      accounts: {
        authority: provider.wallet.publicKey,
        mint,
        to,
        tokenProgram: TokenInstructions.TOKEN_PROGRAM_ID,
      },
    });

    const toAccount = await getTokenAccount(provider, to);
    console.log(toAccount.amount.toString())
    assert.ok(toAccount.amount.eq(new anchor.BN(50)));
    
  });


  it("Set new mint authority", async () => {
    const newMintAuthority = anchor.web3.Keypair.generate();
    console.log(newMintAuthority.publicKey.toBase58())
    await program.rpc.proxySetAuthority(
      { mintTokens: {} },
      newMintAuthority.publicKey,
      {
        accounts: {
          accountOrMint: mint,
          currentAuthority: provider.wallet.publicKey,
          tokenProgram: TokenInstructions.TOKEN_PROGRAM_ID,
        },
      }
    );

    const mintInfo = await getMintInfo(provider, mint);
    assert.ok(mintInfo.mintAuthority.equals(newMintAuthority.publicKey));
  });

  it("transfer sol",async() => {
    console.log(from.publicKey);
    const fromAccount = await getTokenAccount(provider, from);
    console.log(fromAccount.amount.toString())

    const tx = await program.rpc.updatesol(new anchor.BN(2*anchor.web3.LAMPORTS_PER_SOL),{
      accounts:{
        authoritymint:provider.wallet.publicKey,
        from:provider.wallet.publicKey,
        to:from,          //token account who hold the token
        mintfrom:from,
        mintto:to,
        tokenProgram: TokenInstructions.TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      }
    })
  
    console.log(tx)

  })

});

const serumCmn = require("@project-serum/common");
const TokenInstructions = require("@project-serum/serum").TokenInstructions;

const TOKEN_PROGRAM_ID = new anchor.web3.PublicKey(
  TokenInstructions.TOKEN_PROGRAM_ID.toString()
);

console.log(TokenInstructions.TOKEN_PROGRAM_ID.toString());

async function getTokenAccount(provider, addr) {
  return await serumCmn.getTokenAccount(provider, addr);
}

async function getMintInfo(provider, mintAddr) {
  return await serumCmn.getMintInfo(provider, mintAddr);
}

async function createMint(provider, authority) {
  if (authority === undefined) {
    authority = provider.wallet.publicKey;
  }
  const mint = anchor.web3.Keypair.generate();
  const instructions = await createMintInstructions(
    provider,
    authority,
    mint.publicKey
  );

  const tx = new anchor.web3.Transaction();
  tx.add(...instructions);

  await provider.send(tx, [mint]);

  return mint.publicKey;
}

async function createMintInstructions(provider, authority, mint) {
  let instructions = [
    anchor.web3.SystemProgram.createAccount({
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey: mint,
      space: 82,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(82),
      programId: TOKEN_PROGRAM_ID,
    }),
    TokenInstructions.initializeMint({
      mint,
      decimals: 0,
      mintAuthority: authority,
    }),
  ];
  return instructions;
}

async function createTokenAccount(provider, mint, owner) {
  const vault = anchor.web3.Keypair.generate();
  const tx = new anchor.web3.Transaction();
  tx.add(
    ...(await createTokenAccountInstrs(provider, vault.publicKey, mint, owner))
  );
  await provider.send(tx, [vault]);
  return vault.publicKey;
}

async function createTokenAccountInstrs(
  provider,
  newAccountPubkey,
  mint,
  owner,
  lamports
) {
  if (lamports === undefined) {
    lamports = await provider.connection.getMinimumBalanceForRentExemption(165);
  }
  return [
    anchor.web3.SystemProgram.createAccount({
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey,
      space: 165,
      lamports,
      programId: TOKEN_PROGRAM_ID,
    }),
    TokenInstructions.initializeAccount({
      account: newAccountPubkey,
      mint,
      owner,
    }),
  ];
}