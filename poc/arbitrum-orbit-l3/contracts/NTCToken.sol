// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title NTCToken
 * @dev Native Token Contract for Netchain L3 - Gas Currency
 * 
 * NTC (Netchain Token) serves as the native gas currency for the Netchain L3 blockchain.
 * Features:
 * - 100 billion initial supply (100,000,000,000 NTC)
 * - Unlimited supply capability (like ETH) through owner minting
 * - Deflationary mechanics through burning
 * - Gas rebate system integration
 * - Standard ERC20 compatibility for cross-chain operations
 */
contract NTCToken is ERC20, ERC20Burnable, Ownable {
    
    // Initial supply: 100 billion tokens (100 * 10^9 * 10^18)
    uint256 public constant INITIAL_SUPPLY = 100_000_000_000 * 10**18;
    
    // No maximum supply cap - allows unlimited minting like ETH
    
    // Gas rebate configuration
    uint256 public gasRebateRate = 1000; // 0.1% rebate (1000 basis points = 10%)
    mapping(address => uint256) public gasRebateEarned;
    
    // Minting and burning events for transparency
    event TokensMinted(address indexed to, uint256 amount, string reason);
    event TokensBurned(address indexed from, uint256 amount, string reason);
    event GasRebateProcessed(address indexed user, uint256 rebateAmount, uint256 gasUsed);
    
    constructor() ERC20("Netchain Token", "NTC") {
        // Mint initial supply to contract deployer
        _mint(msg.sender, INITIAL_SUPPLY);
        emit TokensMinted(msg.sender, INITIAL_SUPPLY, "Initial Supply");
    }
    
    /**
     * @dev Mint new tokens to specified address
     * @param to Address to receive minted tokens
     * @param amount Amount of tokens to mint
     * @param reason Reason for minting (for transparency)
     */
    function mint(address to, uint256 amount, string calldata reason) external onlyOwner {
        require(to != address(0), "NTC: mint to zero address");
        require(amount > 0, "NTC: mint amount must be positive");
        
        _mint(to, amount);
        emit TokensMinted(to, amount, reason);
    }
    
    /**
     * @dev Batch mint to multiple addresses
     * @param recipients Array of addresses to receive tokens
     * @param amounts Array of amounts corresponding to each recipient
     * @param reason Reason for batch minting
     */
    function batchMint(
        address[] calldata recipients, 
        uint256[] calldata amounts, 
        string calldata reason
    ) external onlyOwner {
        require(recipients.length == amounts.length, "NTC: arrays length mismatch");
        require(recipients.length > 0, "NTC: empty arrays");
        
        for (uint256 i = 0; i < recipients.length; i++) {
            require(recipients[i] != address(0), "NTC: mint to zero address");
            require(amounts[i] > 0, "NTC: amount must be positive");
            
            _mint(recipients[i], amounts[i]);
            emit TokensMinted(recipients[i], amounts[i], reason);
        }
    }
    
    /**
     * @dev Burn tokens from specified address (owner only)
     * @param from Address to burn tokens from
     * @param amount Amount of tokens to burn
     * @param reason Reason for burning
     */
    function burnFrom(address from, uint256 amount, string calldata reason) external onlyOwner {
        require(from != address(0), "NTC: burn from zero address");
        require(amount > 0, "NTC: burn amount must be positive");
        require(balanceOf(from) >= amount, "NTC: insufficient balance to burn");
        
        _burn(from, amount);
        emit TokensBurned(from, amount, reason);
    }
    
    /**
     * @dev Process gas rebate for user after transaction
     * @param user Address to receive gas rebate
     * @param gasUsed Amount of gas used in transaction
     */
    function processGasRebate(address user, uint256 gasUsed) external onlyOwner {
        require(user != address(0), "NTC: rebate to zero address");
        require(gasUsed > 0, "NTC: gas used must be positive");
        
        // Calculate rebate: gasUsed * gasPrice * rebateRate / 10000
        uint256 rebateAmount = (gasUsed * tx.gasprice * gasRebateRate) / 10000;
        
        if (rebateAmount > 0) {
            gasRebateEarned[user] += rebateAmount;
            _mint(user, rebateAmount);
            emit GasRebateProcessed(user, rebateAmount, gasUsed);
        }
    }
    
    /**
     * @dev Update gas rebate rate (owner only)
     * @param newRate New rebate rate in basis points (10000 = 100%)
     */
    function setGasRebateRate(uint256 newRate) external onlyOwner {
        require(newRate <= 1000, "NTC: rebate rate too high"); // Max 10%
        gasRebateRate = newRate;
    }
    
    /**
     * @dev Emergency pause functionality for critical situations
     */
    bool public paused = false;
    
    function pause() external onlyOwner {
        paused = true;
    }
    
    function unpause() external onlyOwner {
        paused = false;
    }
    
    /**
     * @dev Override transfer functions to add pause functionality
     */
    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 amount
    ) internal virtual override {
        require(!paused || from == address(0) || to == address(0), "NTC: token transfers paused");
        super._beforeTokenTransfer(from, to, amount);
    }
    
    /**
     * @dev Get total NTC supply in circulation
     */
    function getCirculatingSupply() external view returns (uint256) {
        return totalSupply();
    }
    
    /**
     * @dev Get gas rebate earned by address
     */
    function getGasRebateEarned(address user) external view returns (uint256) {
        return gasRebateEarned[user];
    }
    
    /**
     * @dev Check if address has sufficient balance for gas payment
     */
    function canPayGas(address user, uint256 gasAmount) external view returns (bool) {
        return balanceOf(user) >= gasAmount;
    }
    
    /**
     * @dev Simulate gas cost for transaction (for frontend estimation)
     */
    function estimateGasCost(uint256 gasLimit, uint256 gasPrice) external pure returns (uint256) {
        return gasLimit * gasPrice;
    }
    
    /**
     * @dev Get token metrics for monitoring and analytics
     */
    function getTokenMetrics() external view returns (
        uint256 currentSupply,
        uint256 totalMinted,
        uint256 totalBurned,
        uint256 currentRebateRate,
        bool isPaused
    ) {
        currentSupply = totalSupply();
        totalMinted = INITIAL_SUPPLY; // Would track total minted in production
        totalBurned = 0; // Would track total burned in production
        currentRebateRate = gasRebateRate;
        isPaused = paused;
    }
}