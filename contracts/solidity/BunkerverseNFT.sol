// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title BunkerverseNFT
 * @dev Simple ERC-721 NFT contract for testing Bunkerverse L3 functionality
 */
contract BunkerverseNFT is ERC721, ERC721URIStorage, Ownable {
    uint256 private _nextTokenId;
    mapping(uint256 => string) private _tokenMetadata;
    
    // Events for testing
    event NFTMinted(address indexed to, uint256 indexed tokenId, string tokenURI);
    event NFTTransferred(address indexed from, address indexed to, uint256 indexed tokenId);
    
    constructor(address initialOwner) 
        ERC721("BunkerverseNFT", "BNFT") 
        Ownable(initialOwner) 
    {
        _nextTokenId = 1;
    }
    
    /**
     * @dev Mint a new NFT to the specified address
     * @param to Address to mint the NFT to
     * @param tokenURI Metadata URI for the NFT
     * @return tokenId The ID of the minted token
     */
    function mint(address to, string memory tokenURI) public onlyOwner returns (uint256) {
        uint256 tokenId = _nextTokenId++;
        _safeMint(to, tokenId);
        _setTokenURI(tokenId, tokenURI);
        _tokenMetadata[tokenId] = tokenURI;
        
        emit NFTMinted(to, tokenId, tokenURI);
        return tokenId;
    }
    
    /**
     * @dev Batch mint multiple NFTs
     * @param to Address to mint the NFTs to
     * @param tokenURIs Array of metadata URIs
     * @return tokenIds Array of minted token IDs
     */
    function batchMint(address to, string[] memory tokenURIs) public onlyOwner returns (uint256[] memory) {
        uint256[] memory tokenIds = new uint256[](tokenURIs.length);
        
        for (uint256 i = 0; i < tokenURIs.length; i++) {
            tokenIds[i] = mint(to, tokenURIs[i]);
        }
        
        return tokenIds;
    }
    
    /**
     * @dev Get the next token ID that will be minted
     * @return The next token ID
     */
    function getNextTokenId() public view returns (uint256) {
        return _nextTokenId;
    }
    
    /**
     * @dev Get the total number of tokens minted
     * @return The total supply
     */
    function totalSupply() public view returns (uint256) {
        return _nextTokenId - 1;
    }
    
    /**
     * @dev Override transfer to emit custom event for testing
     */
    function _update(address to, uint256 tokenId, address auth) internal override returns (address) {
        address from = _ownerOf(tokenId);
        address previousOwner = super._update(to, tokenId, auth);
        
        if (from != address(0) && to != address(0)) {
            emit NFTTransferred(from, to, tokenId);
        }
        
        return previousOwner;
    }
    
    /**
     * @dev Required override for ERC721URIStorage
     */
    function tokenURI(uint256 tokenId) public view override(ERC721, ERC721URIStorage) returns (string memory) {
        return super.tokenURI(tokenId);
    }
    
    /**
     * @dev Required override for ERC721URIStorage
     */
    function supportsInterface(bytes4 interfaceId) public view override(ERC721, ERC721URIStorage) returns (bool) {
        return super.supportsInterface(interfaceId);
    }
    
    /**
     * @dev Get metadata for testing purposes
     */
    function getTokenMetadata(uint256 tokenId) public view returns (string memory) {
        return _tokenMetadata[tokenId];
    }
    
    /**
     * @dev Emergency function for testing - allows owner to transfer any token
     */
    function emergencyTransfer(address from, address to, uint256 tokenId) public onlyOwner {
        _update(to, tokenId, from);
    }
}