const hre = require("hardhat");

async function main() {
  const [deployer] = await hre.ethers.getSigners();
  console.log("Deploying contracts with the account:", deployer.address);

  // Deploy PNKTheosisOracle
  const PNKTheosisOracle = await hre.ethers.getContractFactory("PNKTheosisOracle");
  const pnkOracle = await PNKTheosisOracle.deploy();
  await pnkOracle.waitForDeployment();
  const pnkOracleAddress = await pnkOracle.getAddress();
  console.log("PNKTheosisOracle deployed to:", pnkOracleAddress);

  // Placeholder addresses for Kleros and Cathedral
  const klerosAddress = "0x0000000000000000000000000000000000000001";
  const cathedralAddress = "0x0000000000000000000000000000000000000002";

  // Deploy CathedralKlerosBridge
  const CathedralKlerosBridge = await hre.ethers.getContractFactory("CathedralKlerosBridge");
  const bridge = await CathedralKlerosBridge.deploy(klerosAddress, cathedralAddress);
  await bridge.waitForDeployment();
  console.log("CathedralKlerosBridge deployed to:", await bridge.getAddress());

  // Deploy CathedralKlerosBridgeWithVoting
  const CathedralKlerosBridgeWithVoting = await hre.ethers.getContractFactory("CathedralKlerosBridgeWithVoting");
  const bridgeWithVoting = await CathedralKlerosBridgeWithVoting.deploy(pnkOracleAddress, klerosAddress);
  await bridgeWithVoting.waitForDeployment();
  console.log("CathedralKlerosBridgeWithVoting deployed to:", await bridgeWithVoting.getAddress());
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
