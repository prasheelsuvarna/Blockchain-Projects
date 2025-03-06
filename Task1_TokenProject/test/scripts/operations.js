const hre = require("hardhat");

async function main() {
  const [admin, addr1] = await hre.ethers.getSigners();
  console.log("Admin account:", admin.address);
  console.log("Second account:", addr1.address);

  const contractAddress = "0x064D4EdF291F69AccA6d94AA3f7b9F8D7A52F3C9";
  const MyToken = await hre.ethers.getContractFactory("MyToken");
  const token = await MyToken.attach(contractAddress);
  console.log("Connected to MyToken at:", contractAddress);

  const adminBalance = await token.balanceOf(admin.address);
  const addr1Balance = await token.balanceOf(addr1.address);
  console.log("Admin initial balance:", hre.ethers.formatEther(adminBalance), "MTK");
  console.log("Addr1 initial balance:", hre.ethers.formatEther(addr1Balance), "MTK");

  const mintTx = await token.mint(addr1.address, hre.ethers.parseEther("500"));
  await mintTx.wait();
  console.log("Minted 500 tokens to", addr1.address);
  console.log("Addr1 balance:", hre.ethers.formatEther(await token.balanceOf(addr1.address)), "MTK");

  const burnTx = await token.connect(addr1).burn(hre.ethers.parseEther("200"));
  await burnTx.wait();
  console.log("Burned 200 tokens from", addr1.address);
  console.log("Addr1 balance:", hre.ethers.formatEther(await token.balanceOf(addr1.address)), "MTK");

  const transferTx = await token.connect(addr1).transfer(admin.address, hre.ethers.parseEther("150"));
  await transferTx.wait();
  console.log("Transferred 150 tokens from", addr1.address, "to", admin.address);
  console.log("Admin balance:", hre.ethers.formatEther(await token.balanceOf(admin.address)), "MTK");
  console.log("Addr1 balance:", hre.ethers.formatEther(await token.balanceOf(addr1.address)), "MTK");

  const grantTx = await token.grantMinterRole(addr1.address);
  await grantTx.wait();
  console.log("Granted Minter role to", addr1.address);
  console.log("Addr1 has Minter role:", await token.hasRole(await token.MINTER_ROLE(), addr1.address));

  const mintTx2 = await token.connect(addr1).mint(admin.address, hre.ethers.parseEther("300"));
  await mintTx2.wait();
  console.log("Addr1 minted 300 tokens to", admin.address);
  console.log("Admin balance:", hre.ethers.formatEther(await token.balanceOf(admin.address)), "MTK");

  const revokeTx = await token.revokeMinterRole(addr1.address);
  await revokeTx.wait();
  console.log("Revoked Minter role from", addr1.address);
  console.log("Addr1 has Minter role:", await token.hasRole(await token.MINTER_ROLE(), addr1.address));
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});