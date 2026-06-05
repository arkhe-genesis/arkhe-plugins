// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title PNKTheosisOracle
 * @dev Connects on-chain metrics of qualified jurors from Kleros to the Cathedral.
 */
contract PNKTheosisOracle {
    address public owner;
    mapping(address => uint256) public jurorTheosis;

    event JurorTheosisUpdated(address indexed juror, uint256 theosisScore);

    constructor() {
        owner = msg.sender;
    }

    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner can call this.");
        _;
    }

    function updateTheosis(address juror, uint256 theosisScore) external onlyOwner {
        jurorTheosis[juror] = theosisScore;
        emit JurorTheosisUpdated(juror, theosisScore);
    }

    function getTheosis(address juror) external view returns (uint256) {
        return jurorTheosis[juror];
    }
}
