const anchor = require('@project-serum/anchor');
const solanaWeb3 = require('@solana/web3.js');

const { SystemProgram } = anchor.web3;

describe('test_payment', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.local();
  anchor.setProvider(provider);
  const program = anchor.workspace.TestPayment
  const account = anchor.web3.Keypair.generate();
  const authority = anchor.web3.Keypair.generate();


  it('Creates an account', async () => {
    const signature = await program.provider.connection.requestAirdrop(authority.publicKey, 2000000000);
    await program.provider.connection.confirmTransaction(signature);
    await program.rpc.initialize(10,{
      accounts: {
        lockAccount: account.publicKey,
        owner: authority.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [authority,account]
    });
  });

  it('pays in solana', async () => {
    let connection = new solanaWeb3.Connection('http://localhost:8899', 'confirmed');
    let walletBalance = await connection.getBalance(
      new solanaWeb3.PublicKey(account.publicKey)
    );
    console.log(walletBalance)

    await program.rpc.payin({
      accounts: {
        lockAccount: account.publicKey,
        owner: authority.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [authority.Keypair]
    });

    walletBalance = await connection.getBalance(
      new solanaWeb3.PublicKey(account.publicKey)
    );
    console.log(walletBalance)
  });

});
