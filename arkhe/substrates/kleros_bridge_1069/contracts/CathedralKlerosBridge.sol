// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title CathedralKlerosBridge
 * @dev Basic bridge contract between Kleros and Cathedral.
 */
contract CathedralKlerosBridge {
    address public owner;
    address public klerosAddress;
    address public cathedralAddress;

    event BridgeInitialized(address indexed kleros, address indexed cathedral);
    event DataBridged(uint256 indexed disputeId, string data);

    constructor(address _klerosAddress, address _cathedralAddress) {
        owner = msg.sender;
        klerosAddress = _klerosAddress;
        cathedralAddress = _cathedralAddress;
        emit BridgeInitialized(_klerosAddress, _cathedralAddress);
    }

    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner can call this.");
        _;
    }

    function bridgeData(uint256 disputeId, string calldata data) external onlyOwner {
        emit DataBridged(disputeId, data);
    }
}
