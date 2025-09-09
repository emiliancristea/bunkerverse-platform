const { ethers } = require("hardhat");

async function main() {
    console.log("\n🚀 BUNKERVERSE ORBIT L3 POC - SMART CONTRACT DEPLOYMENT");
    console.log("========================================================");
    
    const [deployer] = await ethers.getSigners();
    const network = await ethers.provider.getNetwork();
    
    console.log(`🌐 Network: ${network.name} (Chain ID: ${network.chainId})`);
    console.log(`👤 Deployer: ${deployer.address}`);
    
    // Get deployer balance
    const balance = await ethers.provider.getBalance(deployer.address);
    console.log(`💰 Balance: ${ethers.formatEther(balance)} ETH`);
    
    // Validate we're on L3
    if (network.chainId !== 33701n) {
        throw new Error(`❌ Expected Bunkerverse L3 (Chain ID: 33701), got ${network.chainId}`);
    }
    
    console.log("\n📋 Deploying BunkerverseNFT contract...");
    
    // Deploy the contract
    const BunkerverseNFT = await ethers.getContractFactory("BunkerverseNFT");
    const contract = await BunkerverseNFT.deploy(deployer.address);
    
    await contract.waitForDeployment();
    const contractAddress = await contract.getAddress();
    
    console.log(`✅ BunkerverseNFT deployed to: ${contractAddress}`);
    
    // Validate deployment
    console.log("\n🔍 Validating contract deployment...");
    
    const contractInfo = await contract.getContractInfo();
    console.log(`📛 Name: ${contractInfo[0]}`);
    console.log(`🏷️  Symbol: ${contractInfo[1]}`);
    console.log(`📊 Total Supply: ${contractInfo[2]}`);
    console.log(`🔢 Version: ${contractInfo[3]}`);
    console.log(`⛓️  Chain Name: ${contractInfo[4]}`);
    console.log(`🆔 Chain ID: ${contractInfo[5]}`);
    
    // Validate L3 chain characteristics
    const l3Validation = await contract.validateL3Chain();
    console.log("\n🔗 L3 Chain Validation:");
    console.log(`   Chain ID: ${l3Validation[0]}`);
    console.log(`   Block Number: ${l3Validation[1]}`);
    console.log(`   Block Timestamp: ${l3Validation[2]}`);
    console.log(`   Block Coinbase: ${l3Validation[3]}`);
    console.log(`   Gas Limit: ${l3Validation[4]}`);
    
    console.log("\n🎉 L3 Smart Contract Deployment SUCCESSFUL!");
    console.log(`📄 Contract Address: ${contractAddress}`);
    console.log(`⛓️  Confirmed on Bunkerverse L3 (Chain ID: ${network.chainId})`);
    
    return {
        contractAddress,
        deployer: deployer.address,
        chainId: network.chainId
    };
}

// Execute deployment
main()
    .then((result) => {
        console.log("\n✅ Deployment completed successfully!");
        process.exit(0);
    })
    .catch((error) => {
        console.error("\n❌ Deployment failed:");
        console.error(error);
        process.exit(1);
    });