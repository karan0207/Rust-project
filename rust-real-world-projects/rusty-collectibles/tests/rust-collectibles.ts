const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;

describe("rusty_collectibles", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.local();
    anchor.setProvider(provider);
  
    const program = anchor.workspace.RustyCollectibles;
  
    it("Mints an NFT!", async () => {
      const mint = anchor.web3.Keypair.generate();
      const userTokenAccount = await anchor.utils.token.associatedAddress({
        mint: mint.publicKey,
        owner: provider.wallet.publicKey,
      });
  
      await program.rpc.mintNft("https://example.com/metadata.json", {
        accounts: {
          mint: mint.publicKey,
          tokenAccount: userTokenAccount,
          payer: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        },
        signers: [mint],
      });
  
      console.log("NFT Minted successfully!");
    });
  });
  