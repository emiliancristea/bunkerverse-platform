// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

/**
 * @title BunkerverseNFT
 * @dev ERC721 NFT contract for Bunkerverse Orbit L3 PoC validation
 * Demonstrates smart contract deployment and interaction on custom L3 chain
 */
contract BunkerverseNFT is ERC721, ERC721URIStorage, Ownable {
    using Counters for Counters.Counter;

    Counters.Counter private _tokenIdCounter;
    
    // Contract metadata
    string public constant CONTRACT_VERSION = "1.0.0";
    string public constant CHAIN_NAME = "Bunkerverse L3";
    
    // Events for PoC validation
    event NFTMinted(
        uint256 indexed tokenId, 
        address indexed to, 
        string tokenURI, 
        uint256 timestamp
    );
    
    event NFTTransferred(
        uint256 indexed tokenId,
        address indexed from,
        address indexed to,
        uint256 timestamp
    );

    constructor(address initialOwner) 
        ERC721("Bunkerverse PoC NFT", "BVNFT") 
        Ownable(initialOwner)
    {
        // Start token IDs at 1
        _tokenIdCounter.increment();
    }

    /**
     * @dev Mint NFT to specified address with metadata URI
     * Used for PoC validation of L3 chain functionality
     */
    function safeMint(address to, string memory uri) public onlyOwner returns (uint256) {
        uint256 tokenId = _tokenIdCounter.current();
        _tokenIdCounter.increment();
        
        _safeMint(to, tokenId);
        _setTokenURI(tokenId, uri);
        
        emit NFTMinted(tokenId, to, uri, block.timestamp);
        
        return tokenId;
    }

    /**
     * @dev Batch mint multiple NFTs for testing
     */
    function batchMint(
        address[] memory recipients, 
        string[] memory uris
    ) public onlyOwner returns (uint256[] memory) {
        require(recipients.length == uris.length, "Arrays length mismatch");
        
        uint256[] memory tokenIds = new uint256[](recipients.length);
        
        for (uint256 i = 0; i < recipients.length; i++) {
            tokenIds[i] = safeMint(recipients[i], uris[i]);
        }
        
        return tokenIds;
    }

    /**
     * @dev Get current total supply
     */
    function totalSupply() public view returns (uint256) {
        return _tokenIdCounter.current() - 1;
    }

    /**
     * @dev Get contract info for PoC validation
     */
    function getContractInfo() public view returns (
        string memory name,
        string memory symbol,
        uint256 totalSupply,
        string memory version,
        string memory chainName,
        uint256 chainId
    ) {
        return (
            name(),
            symbol(),
            totalSupply(),
            CONTRACT_VERSION,
            CHAIN_NAME,
            block.chainid
        );
    }

    /**
     * @dev Validate L3 chain characteristics
     */
    function validateL3Chain() public view returns (
        uint256 chainId,
        uint256 blockNumber,
        uint256 blockTimestamp,
        address blockCoinbase,
        uint256 gasLimit
    ) {
        return (
            block.chainid,
            block.number,
            block.timestamp,
            block.coinbase,
            block.gaslimit
        );
    }

    // Override required functions
    function _update(address to, uint256 tokenId, address auth)
        internal
        override
        returns (address)
    {
        address from = _ownerOf(tokenId);
        address previousOwner = super._update(to, tokenId, auth);
        
        // Emit transfer event for PoC validation
        if (from != address(0) && to != address(0)) {
            emit NFTTransferred(tokenId, from, to, block.timestamp);
        }
        
        return previousOwner;
    }

    function tokenURI(uint256 tokenId)
        public
        view
        override(ERC721, ERC721URIStorage)
        returns (string memory)
    {
        return super.tokenURI(tokenId);
    }

    function supportsInterface(bytes4 interfaceId)
        public
        view
        override(ERC721, ERC721URIStorage)
        returns (bool)
    {
        return super.supportsInterface(interfaceId);
    }
}