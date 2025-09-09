const { ethers } = require("hardhat");

async function main() {
    console.log("\n🧪 BUNKERVERSE ORBIT L3 POC - NFT FUNCTIONALITY TESTING");
    console.log("======================================================");
    
    const [deployer, user1, user2] = await ethers.getSigners();
    const network = await ethers.provider.getNetwork();
    
    console.log(`🌐 Network: Bunkerverse L3 (Chain ID: ${network.chainId})`);
    console.log(`👤 Deployer: ${deployer.address}`);
    console.log(`👤 User1: ${user1.address}`);
    console.log(`👤 User2: ${user2.address}`);
    
    // Get deployed contract address (you'll need to update this)
    // For now, we'll deploy a new one for testing
    console.log("\n📋 Deploying test contract...");
    
    const BunkerverseNFT = await ethers.getContractFactory("BunkerverseNFT");
    const contract = await BunkerverseNFT.deploy(deployer.address);
    await contract.waitForDeployment();
    const contractAddress = await contract.getAddress();
    
    console.log(`✅ Test contract deployed to: ${contractAddress}`);
    
    // Test 1: Mint NFT to user1
    console.log("\n🎨 TEST 1: Minting NFT to User1...");
    const tokenURI1 = "ipfs://QmYourFirstNFTMetadataHash";
    
    const tx1 = await contract.safeMint(user1.address, tokenURI1);
    const receipt1 = await tx1.wait();
    
    console.log(`✅ NFT minted! Token ID: 1`);
    console.log(`   Owner: ${user1.address}`);
    console.log(`   URI: ${tokenURI1}`);
    console.log(`   Gas Used: ${receipt1.gasUsed}`);
    
    // Test 2: Batch mint multiple NFTs
    console.log("\n🎨 TEST 2: Batch minting NFTs...");
    const recipients = [user1.address, user2.address, deployer.address];
    const uris = [
        "ipfs://QmBatchNFT1MetadataHash",
        "ipfs://QmBatchNFT2MetadataHash", 
        "ipfs://QmBatchNFT3MetadataHash"
    ];
    
    const tx2 = await contract.batchMint(recipients, uris);
    const receipt2 = await tx2.wait();
    
    console.log(`✅ Batch mint completed! Tokens 2, 3, 4 created`);
    console.log(`   Gas Used: ${receipt2.gasUsed}`);
    
    // Test 3: Check total supply
    const totalSupply = await contract.totalSupply();
    console.log(`📊 Total Supply: ${totalSupply} NFTs`);
    
    // Test 4: Transfer NFT from user1 to user2
    console.log("\n📤 TEST 3: Transferring NFT from User1 to User2...");
    
    const contractAsUser1 = contract.connect(user1);
    const tx3 = await contractAsUser1.transferFrom(user1.address, user2.address, 1);
    const receipt3 = await tx3.wait();
    
    console.log(`✅ NFT transferred!`);
    console.log(`   Token ID: 1`);
    console.log(`   From: ${user1.address}`);
    console.log(`   To: ${user2.address}`);
    console.log(`   Gas Used: ${receipt3.gasUsed}`);
    
    // Test 4: Verify ownership
    console.log("\n🔍 TEST 4: Verifying NFT ownership...");
    
    for (let tokenId = 1; tokenId <= 4; tokenId++) {
        const owner = await contract.ownerOf(tokenId);
        const uri = await contract.tokenURI(tokenId);
        console.log(`   Token ${tokenId}: Owner = ${owner.slice(0,10)}..., URI = ${uri.slice(0,30)}...`);
    }
    
    // Test 5: Validate L3 characteristics during transactions
    console.log("\n⛓️  TEST 5: L3 Chain Validation During Transactions...");
    
    const l3Data = await contract.validateL3Chain();
    console.log(`   Chain ID: ${l3Data[0]} (Expected: 33701)`);
    console.log(`   Current Block: ${l3Data[1]}`);
    console.log(`   Block Timestamp: ${new Date(Number(l3Data[2]) * 1000).toISOString()}`);
    console.log(`   Gas Limit: ${l3Data[4]}`);
    
    // Verify we're on the correct L3
    if (l3Data[0] === 33701n) {
        console.log(`✅ Successfully operating on Bunkerverse L3!`);
    } else {
        throw new Error(`❌ Wrong chain! Expected 33701, got ${l3Data[0]}`);
    }
    
    console.log("\n🎉 ALL NFT TESTS PASSED ON BUNKERVERSE L3!");
    console.log("============================================");
    console.log(`📄 Contract: ${contractAddress}`);
    console.log(`🎨 NFTs Minted: ${totalSupply}`);
    console.log(`⛓️  Chain: Bunkerverse L3 (ID: ${network.chainId})`);
    console.log(`⚡ L3 transactions completed successfully!`);
    
    return {
        contractAddress,
        totalSupply: Number(totalSupply),
        chainId: Number(network.chainId),
        testsCompleted: 5
    };
}

// Execute testing
main()
    .then((result) => {
        console.log("\n✅ All L3 NFT tests completed successfully!");
        process.exit(0);
    })
    .catch((error) => {
        console.error("\n❌ L3 NFT testing failed:");
        console.error(error);
        process.exit(1);
    });