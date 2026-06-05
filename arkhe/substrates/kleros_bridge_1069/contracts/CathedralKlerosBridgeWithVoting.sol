// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title CathedralKlerosBridgeWithVoting
 * @dev Integrates Theosis-weighted voting into Kleros courts via the Cathedral Substrate 1069.
 */
contract CathedralKlerosBridgeWithVoting {
    address public owner;
    address public pnkOracle;
    address public klerosCourt;

    mapping(address => uint256) public theosisWeights;

    event TheosisWeightUpdated(address indexed juror, uint256 newWeight);
    event VoteCast(address indexed juror, uint256 disputeId, uint256 choice, uint256 weight);

    constructor(address _pnkOracle, address _klerosCourt) {
        owner = msg.sender;
        pnkOracle = _pnkOracle;
        klerosCourt = _klerosCourt;
    }

    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner can call this.");
        _;
    }

    function updateTheosisWeight(address juror, uint256 newWeight) external {
        // In a real implementation, this would verify a ZK proof or check the oracle
        require(msg.sender == owner || msg.sender == pnkOracle, "Unauthorized");
        theosisWeights[juror] = newWeight;
        emit TheosisWeightUpdated(juror, newWeight);
    }

    function castWeightedVote(uint256 disputeId, uint256 choice) external {
        uint256 weight = theosisWeights[msg.sender];
        require(weight > 0, "No Theosis weight assigned");

        // Here we would interact with the Kleros court
        // klerosCourt.castVote(disputeId, choice, weight);

        emit VoteCast(msg.sender, disputeId, choice, weight);
    }
}
