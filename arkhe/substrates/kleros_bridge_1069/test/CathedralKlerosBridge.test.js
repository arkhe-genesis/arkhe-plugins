const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("CathedralKlerosBridgeWithVoting", function () {
  let pnkOracle;
  let bridgeWithVoting;
  let owner;
  let juror1;
  let juror2;
  const klerosAddress = "0x0000000000000000000000000000000000000001";

  beforeEach(async function () {
    [owner, juror1, juror2] = await ethers.getSigners();

    const PNKTheosisOracle = await ethers.getContractFactory("PNKTheosisOracle");
    pnkOracle = await PNKTheosisOracle.deploy();
    await pnkOracle.waitForDeployment();
    const pnkOracleAddress = await pnkOracle.getAddress();

    const CathedralKlerosBridgeWithVoting = await ethers.getContractFactory("CathedralKlerosBridgeWithVoting");
    bridgeWithVoting = await CathedralKlerosBridgeWithVoting.deploy(pnkOracleAddress, klerosAddress);
    await bridgeWithVoting.waitForDeployment();
  });

  it("Should set the right owner", async function () {
    expect(await bridgeWithVoting.owner()).to.equal(owner.address);
  });

  it("Should update Theosis weight correctly", async function () {
    await bridgeWithVoting.updateTheosisWeight(juror1.address, 100);
    expect(await bridgeWithVoting.theosisWeights(juror1.address)).to.equal(100);
  });

  it("Should cast weighted vote if weight is greater than 0", async function () {
    await bridgeWithVoting.updateTheosisWeight(juror1.address, 100);

    await expect(bridgeWithVoting.connect(juror1).castWeightedVote(1, 2))
      .to.emit(bridgeWithVoting, "VoteCast")
      .withArgs(juror1.address, 1, 2, 100);
  });

  it("Should revert vote if weight is 0", async function () {
    await expect(
      bridgeWithVoting.connect(juror2).castWeightedVote(1, 2)
    ).to.be.revertedWith("No Theosis weight assigned");
  });
});
