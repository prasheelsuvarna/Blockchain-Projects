const hre = require("hardhat");

async function main() {
  const [signer] = await hre.ethers.getSigners();
  console.log("Interacting with account:", signer.address);

  // Connect to the deployed contract
  const contractAddress = "0x064D4EdF291F69AccA6d94AA3f7b9F8D7A52F3C9";
  const MyToken = await hre.ethers.getContractFactory("MyToken");
  const token = await MyToken.attach(contractAddress);

  // Check initial balance
  const balance = await token.balanceOf(signer.address);
  console.log("Balance:", hre.ethers.formatEther(balance), "MTK");

  // Mint 500 more tokens and wait for transaction confirmation
  const mintTx = await token.mint(signer.address, hre.ethers.parseEther("500"));
  await mintTx.wait(); // Wait for the transaction to be mined
  console.log("Minted 500 tokens");

  // Check updated balance
  const newBalance = await token.balanceOf(signer.address);
  console.log("New Balance:", hre.ethers.formatEther(newBalance), "MTK");
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});