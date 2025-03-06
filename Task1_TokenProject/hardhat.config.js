require("@nomicfoundation/hardhat-toolbox");

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: "0.8.28",
  networks: {
    sepolia: {
      url: "https://eth-sepolia.g.alchemy.com/v2/22nBZwHn9UIuCSNscrGPRL1D70Hy5Qcj", // From Alchemy/Infura
      accounts: ["26a5dc75d8f066ce4cd5ec1175ea13de90249939d9a1f74ec022c188dd074730","2f8e05d95a308692c35201d3de46c819031af546b3be06adba97fe9ec787ddf5"] // From MetaMask (never share this!)
    }
  }
};

