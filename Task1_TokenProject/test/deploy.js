const hre = require("hardhat");

async function main() {
  const [deployer] = await hre.ethers.getSigners();
  console.log("Deploying with account:", deployer.address);

  const MyToken = await hre.ethers.getContractFactory("MyToken");
  const token = await MyToken.deploy(1000); // 1000 tokens initial supply
  await token.waitForDeployment(); // Updated for Ethers.js v6

  console.log("MyToken deployed to:", token.target); // 'target' is the address in v6
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});