const Voting = artifacts.require("Voting");

contract('Voting', (accounts) => {
  let instance;

  before(async () => {
    instance = await Voting.deployed();
  });

  it('should add candidates', async () => {
    await instance.addCandidate("Alice", { from: accounts[0] });
    await instance.addCandidate("Bob", { from: accounts[0] });
    const candidates = await instance.getCandidates();
    assert.equal(candidates.length, 2, "Candidates should be 2");
  });

  it('should authorize and vote', async () => {
    await instance.authorizeVoter(accounts[1], { from: accounts[0] });
    await instance.vote(0, { from: accounts[1] });
    const totalVotes = await instance.totalVotes();
    assert.equal(totalVotes, 1, "There should be 1 vote");
  });

  it('should return the correct winner', async () => {
    const result = await instance.endElection({ from: accounts[0] });
    assert.equal(result[0], "Alice", "The winner should be Alice");
  });
});
