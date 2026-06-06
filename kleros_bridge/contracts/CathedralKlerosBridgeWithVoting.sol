// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./CathedralKlerosBridge.sol";

contract CathedralKlerosBridgeWithVoting is CathedralKlerosBridge {
    mapping(address => uint256) public jurorTheosisWeights;

    event TheosisWeightUpdated(address indexed juror, uint256 weight);

    constructor(address _arbitrator, address _pnkOracle) CathedralKlerosBridge(_arbitrator, _pnkOracle) {}

    function updateJurorTheosisWeight(address juror, uint256 weight) public {
        jurorTheosisWeights[juror] = weight;
        emit TheosisWeightUpdated(juror, weight);
    }

    function getWeightedVote(address juror, uint256 baseVoteWeight) public view returns (uint256) {
        return baseVoteWeight + jurorTheosisWeights[juror];
    }
}
