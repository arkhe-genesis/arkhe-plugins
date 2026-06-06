const hre = require("hardhat");
const fs = require("fs");

async function main() {
  const [deployer] = await hre.ethers.getSigners();
  console.log("Deploying contracts with the account:", deployer.address);

  // Deploy PNKTheosisOracle
  const Oracle = await hre.ethers.getContractFactory("PNKTheosisOracle");
  const oracle = await Oracle.deploy();
  await oracle.waitForDeployment();
  const oracleAddress = await oracle.getAddress();
  console.log("PNKTheosisOracle deployed to:", oracleAddress);

  // Deploy CathedralKlerosBridgeWithVoting
  const mockArbitrator = "0x" + "3".repeat(40);
  const Bridge = await hre.ethers.getContractFactory("CathedralKlerosBridgeWithVoting");
  const bridge = await Bridge.deploy(mockArbitrator, oracleAddress);
  await bridge.waitForDeployment();
  const bridgeAddress = await bridge.getAddress();
  console.log("CathedralKlerosBridgeWithVoting deployed to:", bridgeAddress);

  const deployments = {
      PNKTheosisOracle: oracleAddress,
      CathedralKlerosBridgeWithVoting: bridgeAddress
  };
  fs.writeFileSync("kleros_bridge/deployments.json", JSON.stringify(deployments, null, 2));
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
