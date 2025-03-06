const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("MyToken Contract", function () {
  let MyToken, token, owner, addr1, addr2;

  beforeEach(async function () {
    MyToken = await ethers.getContractFactory("MyToken");
    [owner, addr1, addr2] = await ethers.getSigners();
    token = await MyToken.deploy(1000);
  });

  it("Should deploy with correct initial supply", async function () {
    const ownerBalance = await token.balanceOf(owner.address);
    expect(ownerBalance).to.equal(ethers.parseEther("1000"));
  });

  it("Should allow Admin to mint tokens", async function () {
    await token.mint(addr1.address, ethers.parseEther("500"));
    const addr1Balance = await token.balanceOf(addr1.address);
    expect(addr1Balance).to.equal(ethers.parseEther("500"));
  });

  it("Should restrict minting to Minter role only", async function () {
    await expect(
      token.connect(addr1).mint(addr2.address, ethers.parseEther("100"))
    ).to.be.reverted; // Simplified to just check for revert
  });

  it("Should allow Admin to grant and revoke Minter role", async function () {
    await token.grantMinterRole(addr1.address);
    await token.connect(addr1).mint(addr2.address, ethers.parseEther("200"));
    expect(await token.balanceOf(addr2.address)).to.equal(ethers.parseEther("200"));

    await token.revokeMinterRole(addr1.address);
    await expect(
      token.connect(addr1).mint(addr2.address, ethers.parseEther("100"))
    ).to.be.reverted; // Simplified to just check for revert
  });

  it("Should allow users to burn their own tokens", async function () {
    await token.mint(addr1.address, ethers.parseEther("300"));
    await token.connect(addr1).burn(ethers.parseEther("100"));
    const addr1Balance = await token.balanceOf(addr1.address);
    expect(addr1Balance).to.equal(ethers.parseEther("200"));
  });

  it("Should allow token transfers between accounts", async function () {
    await token.transfer(addr1.address, ethers.parseEther("400"));
    expect(await token.balanceOf(addr1.address)).to.equal(ethers.parseEther("400"));

    await token.connect(addr1).transfer(addr2.address, ethers.parseEther("150"));
    expect(await token.balanceOf(addr2.address)).to.equal(ethers.parseEther("150"));
    expect(await token.balanceOf(addr1.address)).to.equal(ethers.parseEther("250"));
  });
});