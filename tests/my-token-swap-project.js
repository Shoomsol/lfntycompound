const anchor = require("@project-serum/anchor");
const assert = require("assert");
const { SystemProgram, PublicKey } = anchor.web3;
const { Token, TOKEN_PROGRAM_ID } = require("@solana/spl-token");

// Configure the client to use the local cluster.
const provider = anchor.Provider.env();
anchor.setProvider(provider);

const program = anchor.workspace.MyTokenSwapProject;

const { Token, TOKEN_PROGRAM_ID } = require("@solana/spl-token");

    async function createMint(provider, authority, decimals) {
      const mint = anchor.web3.Keypair.generate();
      const lamports = await provider.connection.getMinimumBalanceForRentExemption(Token.MINIMUM_BALANCE);

      let transaction = new anchor.web3.Transaction().add(
        anchor.web3.SystemProgram.createAccount({
          fromPubkey: provider.wallet.publicKey,
          newAccountPubkey: mint.publicKey,
          space: Token.MINIMUM_BALANCE,
          lamports: lamports,
          programId: TOKEN_PROGRAM_ID,
        }),
        Token.createInitMintInstruction(
          TOKEN_PROGRAM_ID, 
          mint.publicKey, 
          decimals, 
          authority, 
          null // Freeze authority (null if not used)
        )
      );

      await provider.sendAndConfirm(transaction, [mint]);
      return mint.publicKey;
    }

    async function createTokenAccount(provider, mint, owner) {
      const account = anchor.web3.Keypair.generate();
      const lamports = await provider.connection.getMinimumBalanceForRentExemption(Token.MINIMUM_BALANCE);
    
      let transaction = new anchor.web3.Transaction().add(
        anchor.web3.SystemProgram.createAccount({
          fromPubkey: provider.wallet.publicKey,
          newAccountPubkey: account.publicKey,
          space: Token.MINIMUM_BALANCE,
          lamports: lamports,
          programId: TOKEN_PROGRAM_ID,
        }),
        Token.createInitAccountInstruction(
          TOKEN_PROGRAM_ID, 
          mint, 
          account.publicKey, 
          owner
        )
      );
    
      await provider.sendAndConfirm(transaction, [account]);
      return account.publicKey;
    }
    
    async function mintToAccount(provider, mint, destination, authorityKeypair, amount) {
      const token = new Token(provider.connection, mint, TOKEN_PROGRAM_ID, authorityKeypair);
      await token.mintTo(destination, authorityKeypair.publicKey, [], amount);
    }

describe("my-token-swap-project", () => {
  let user, xMint, yMint, userXAccount, userYAccount, vaultXAccount;

  before(async () => {
    // Generate a new keypair for the user
    user = anchor.web3.Keypair.generate();

    // Fund the user account with some SOL
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(user.publicKey, 1000000000),
      "confirmed"
    );

    

    // Create a mint for X tokens
    xMint = await createMint(provider, user.publicKey, 9);

    // Create token accounts for the user for X tokens
    userXAccount = await createTokenAccount(provider, xMint, user.publicKey);

    // Mint some initial X tokens to the user's X token account
    await mintToAccount(provider, xMint, userXAccount, user, 1000 * 10 ** 9);

    // Create a mint for Y tokens controlled by the smart contract
    yMint = await createMint(provider, program.programId, 9);

    // Create a token account for the user for Y tokens
    userYAccount = await createTokenAccount(provider, yMint, user.publicKey);

    // Create a vault account for X tokens controlled by the smart contract
    vaultXAccount = await createTokenAccount(provider, xMint, program.programId);
    

  });

  it("Deposits X token and receives Y token", async () => {
    const amount = new anchor.BN(100);

    await program.rpc.depositXReceiveY(amount, {
      accounts: {
        user: user.publicKey,
        xMint: xMint.publicKey,
        yMint: yMint.publicKey,
        userXAccount: userXAccount.publicKey,
        userYAccount: userYAccount.publicKey,
        vaultXAccount: vaultXAccount.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [user],
    });

    // Add assertions here
  });

  it("Burns Y token and receives X token", async () => {
    const amount = new anchor.BN(50);

    await program.rpc.burnYReceiveX(amount, {
      accounts: {
        user: user.publicKey,
        xMint: xMint.publicKey,
        yMint: yMint.publicKey,
        userXAccount: userXAccount.publicKey,
        userYAccount: userYAccount.publicKey,
        vaultXAccount: vaultXAccount.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [user],
    });

    // Add assertions here
  });

  // Define the helper functions here
});
