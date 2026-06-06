// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract CathedralKlerosBridge {
    address public arbitrator;
    address public pnkOracle;

    event DisputeCreated(uint256 disputeID, string courtInfo);

    constructor(address _arbitrator, address _pnkOracle) {
        arbitrator = _arbitrator;
        pnkOracle = _pnkOracle;
    }

    function createDispute(string memory courtInfo) public payable returns (uint256) {
        // Mock Kleros arbitrator interface
        uint256 disputeID = 1;
        emit DisputeCreated(disputeID, courtInfo);
        return disputeID;
    }
}
