// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../contracts/CathedralKlerosBridgeWithVoting.sol";
import "../contracts/PNKTheosisOracle.sol";

contract CathedralKlerosBridgeTest is Test {
    CathedralKlerosBridgeWithVoting bridge;
    PNKTheosisOracle oracle;

    function setUp() public {
        oracle = new PNKTheosisOracle();
        bridge = new CathedralKlerosBridgeWithVoting(address(0), address(oracle));
    }

    function testVotingWeight() public {
        bridge.updateJurorTheosisWeight(address(this), 100);
        assertEq(bridge.getWeightedVote(address(this), 50), 150);
    }
}
