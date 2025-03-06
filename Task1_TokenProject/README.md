# MyToken ERC-20 Project

This project implements an ERC-20 token (`MyToken`) with a customized minting mechanism and role-based access control using Solidity and Hardhat. The token is deployed on the Sepolia testnet and includes scripts to perform operations like minting, burning, transferring tokens, and managing roles.

## Features
- **Token Details**: Name: "MyToken", Symbol: "MTK", Decimals: 18, Initial Supply: 1000 tokens.
- **Minting**: Restricted to Admin and Minter roles.
- **Burning**: Users can burn their own tokens.
- **Transfers**: Standard ERC-20 token transfers between accounts.
- **Role-Based Access**: Admin can grant/revoke Minter roles.

## Prerequisites
- **Node.js**: Version 18 (LTS recommended; v23.7.0 may cause issues).
- **npm**: Node package manager.
- **MetaMask**: For private keys and Sepolia testnet interaction.
- **Alchemy Account**: For Sepolia RPC endpoint.
- **Sepolia ETH**: Test ETH for gas fees.

## Setup Instructions

### 1. Clone the Repository
If this is a new project, create a folder and initialize it:
```bash
mkdir mytoken-project
cd mytoken-project
git init  # Optional: if using git
npm init -y

2. Install Dependencies
Install Hardhat and OpenZeppelin:
npm install --save-dev hardhat @nomicfoundation/hardhat-toolbox
npm install @openzeppelin/contracts
npx hardhat init 

3. Configure Hardhat
Alchemy API Key: Sign up at Alchemy, create a Sepolia app, and copy the HTTP URL (e.g., https://eth-sepolia.g.alchemy.com/v2/abcdefghijklmnopqrstuvwxyz123456).
Private Key(s):
Get from MetaMask: "Account Details" > "Export Private Key".
For two accounts (recommended), add a second key: accounts: ["ADMIN_KEY", "SECOND_KEY"].


Running the Project
1. Install Node.js v18 (Recommended)
Hardhat may not support Node.js v23.7.0:
# Install nvm (if not installed)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
# Restart terminal, then:
nvm install 18
nvm use 18

npx hardhat compile
npx hardhat test
npx hardhat run test/scripts/operations.js --network sepolia
