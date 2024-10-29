// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Voting {
    struct Candidate {
        uint id;
        string name;
        uint voteCount;
    }

    struct Voter {
        bool authorized;
        bool voted;
        uint vote;
    }

    address public owner;
    string public electionName;
    uint public totalVotes;
    
    mapping(address => Voter) public voters;
    Candidate[] public candidates;

    modifier ownerOnly() {
        require(msg.sender == owner, "Caller is not owner");
        _;
    }

    constructor(string memory _name) {
        owner = msg.sender;
        electionName = _name;
    }

    function addCandidate(string memory _name) public ownerOnly {
        candidates.push(Candidate(candidates.length, _name, 0));
    }

    function authorizeVoter(address _voter) public ownerOnly {
        voters[_voter].authorized = true;
    }

    function vote(uint _candidateId) public {
        require(!voters[msg.sender].voted, "You have already voted");
        require(voters[msg.sender].authorized, "You are not authorized to vote");

        voters[msg.sender].voted = true;
        voters[msg.sender].vote = _candidateId;

        candidates[_candidateId].voteCount += 1;
        totalVotes += 1;
    }

    function getCandidates() public view returns (Candidate[] memory) {
        return candidates;
    }

    function endElection() public ownerOnly view returns (string memory winnerName, uint winnerVoteCount) {
        uint maxVoteCount = 0;
        uint winnerIndex = 0;

        for (uint i = 0; i < candidates.length; i++) {
            if (candidates[i].voteCount > maxVoteCount) {
                maxVoteCount = candidates[i].voteCount;
                winnerIndex = i;
            }
        }

        winnerName = candidates[winnerIndex].name;
        winnerVoteCount = candidates[winnerIndex].voteCount;
    }
}
