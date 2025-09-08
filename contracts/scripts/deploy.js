const { ethers } = require("hardhat");

async function main() {
  console.log("🚀 Deploying BunkerverseNFT contract...");
  
  // Get the deployer account
  const [deployer] = await ethers.getSigners();
  console.log(`📝 Deploying with account: ${deployer.address}`);
  
  // Get account balance
  const balance = await ethers.provider.getBalance(deployer.address);
  console.log(`💰 Account balance: ${ethers.formatEther(balance)} ETH`);
  
  // Deploy the contract
  const BunkerverseNFT = await ethers.getContractFactory("BunkerverseNFT");
  const nft = await BunkerverseNFT.deploy(deployer.address);
  
  await nft.waitForDeployment();
  const contractAddress = await nft.getAddress();
  
  console.log(`✅ BunkerverseNFT deployed to: ${contractAddress}`);
  
  // Test minting functionality
  console.log("\n🎨 Testing NFT minting...");
  
  const mintTx = await nft.mint(
    deployer.address, 
    "https://ipfs.io/ipfs/QmTestHash1/metadata.json"
  );
  await mintTx.wait();
  
  const tokenId = await nft.getNextTokenId() - 1n;
  console.log(`✅ Minted NFT with token ID: ${tokenId}`);
  
  // Test batch minting
  console.log("\n🔢 Testing batch minting...");
  const tokenURIs = [
    "https://ipfs.io/ipfs/QmTestHash2/metadata.json",
    "https://ipfs.io/ipfs/QmTestHash3/metadata.json", 
    "https://ipfs.io/ipfs/QmTestHash4/metadata.json"
  ];
  
  const batchMintTx = await nft.batchMint(deployer.address, tokenURIs);
  await batchMintTx.wait();
  
  const totalSupply = await nft.totalSupply();
  console.log(`✅ Total NFTs minted: ${totalSupply}`);
  
  // Test transfer
  console.log("\n🔄 Testing NFT transfer...");
  const [, recipient] = await ethers.getSigners();
  
  const transferTx = await nft.transferFrom(
    deployer.address, 
    recipient.address, 
    1n
  );
  await transferTx.wait();
  
  const newOwner = await nft.ownerOf(1n);
  console.log(`✅ Token 1 transferred to: ${newOwner}`);
  
  // Network info
  const network = await ethers.provider.getNetwork();
  console.log(`\n🌐 Network: ${network.name} (Chain ID: ${network.chainId})`);
  
  console.log("\n🎉 Contract deployment and testing completed successfully!");
  
  return {
    contractAddress,
    totalSupply: totalSupply.toString(),
    network: {
      name: network.name,
      chainId: network.chainId.toString()
    }
  };
}

if (require.main === module) {
  main()
    .then(() => process.exit(0))
    .catch((error) => {
      console.error("❌ Deployment failed:", error);
      process.exit(1);
    });
}

module.exports = main;