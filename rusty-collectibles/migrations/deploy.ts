const anchor = require("@project-serum/anchor");

module.exports = async function (provider) {
  // Configure the client to use the provider.
  anchor.setProvider(provider);

  const program = anchor.workspace.RustyCollectibles;
  console.log("Deploying program...");
  console.log("Program deployed with ID:", program.programId.toBase58());
};
