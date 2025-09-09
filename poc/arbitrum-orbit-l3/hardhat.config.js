require("@nomicfoundation/hardhat-toolbox");

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: {
    version: "0.8.19",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200,
      },
    },
  },
  networks: {
    // L1 Ethereum (Anvil)
    ethereum: {
      url: "http://localhost:8545",
      chainId: 31337,
      accounts: [
        // Default Anvil private key
        "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
      ],
      gas: 30000000,
      gasPrice: 1000000000, // 1 gwei
    },
    
    // L2 Arbitrum One (local)
    "arbitrum-l2": {
      url: "http://localhost:8547", 
      chainId: 42161,
      accounts: [
        "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
      ],
      gas: 30000000,
      gasPrice: 100000000, // 0.1 gwei
    },
    
    // L3 Bunkerverse Orbit
    "bunkerverse-l3": {
      url: "http://localhost:8549",
      chainId: 33701,
      accounts: [
        "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
      ],
      gas: 30000000,
      gasPrice: 10000000, // 0.01 gwei - very low for L3
    }
  },
  paths: {
    sources: "./contracts",
    tests: "./test",
    cache: "./cache",
    artifacts: "./artifacts"
  }
};