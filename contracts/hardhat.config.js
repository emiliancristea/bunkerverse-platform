require("@nomicfoundation/hardhat-toolbox");

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: {
    version: "0.8.19",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200
      }
    }
  },
  networks: {
    localhost: {
      url: "http://127.0.0.1:8545",
      chainId: 31337,
      accounts: {
        mnemonic: "test test test test test test test test test test test junk"
      }
    },
    bunkerverse: {
      url: "http://127.0.0.1:8549",
      chainId: 33701,
      accounts: {
        mnemonic: "test test test test test test test test test test test junk"
      },
      gasPrice: 1000000000, // 1 gwei
      gas: 8000000
    },
    arbitrum_local: {
      url: "http://127.0.0.1:8547", 
      chainId: 42161,
      accounts: {
        mnemonic: "test test test test test test test test test test test junk"
      }
    }
  },
  paths: {
    sources: "./solidity",
    tests: "./test",
    cache: "./cache",
    artifacts: "./artifacts"
  }
};