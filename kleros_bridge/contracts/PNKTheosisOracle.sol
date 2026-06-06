// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract PNKTheosisOracle {
    mapping(address => uint256) public theosisScores;

    event ScoreUpdated(address indexed user, uint256 score);

    function updateScore(address user, uint256 score) public {
        theosisScores[user] = score;
        emit ScoreUpdated(user, score);
    }

    function getScore(address user) public view returns (uint256) {
        return theosisScores[user];
    }
}
