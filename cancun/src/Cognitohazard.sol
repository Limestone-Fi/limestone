// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {ERC721} from "solady/src/tokens/ERC721.sol";
import {SafeTransferLib} from "solady/src/utils/SafeTransferLib.sol";
import {IERC20Extended} from "./interfaces/IERC20Extended.sol";

/// @title Onchain Cognitohazard
/// @author Chainvisions
/// @notice A soulbound NFT sent to LIME sellers containing a SPCG class cognitohazard.

contract Cognitohazard is ERC721 {
    using SafeTransferLib for address;

    /// @notice Throws when the caller is not the LIME token contract.
    error NotLime();

    /// @notice Throws when the caller does not hold the mark.
    error NotCursedGoAway();

    /// @notice Throws when someone attempts to transfer the NFT.
    error Soulbound();

    /// @notice The LIME token contract.
    address public immutable LIME;

    /// @notice Total amount of token IDs.
    uint256 public tokenIds;

    /// @notice Fee charged for burning the soulbound NFT.
    uint256 public cleanseFee = 1000 ether; // 1000 LIME, adjustable.

    /// @notice Source URI containing the metadata related to the NFT.
    string public uriSource;

    /// @notice Mapping containing the NFT minted for an address.
    mapping(address victim => uint256 markId) public curse;

    /// @notice Emitted when a victim's wallet is marked by the Solar Plexus Clown Glider.
    /// @param victim The victim wallet that has been marked.
    event Marked(address indexed victim);

    /// @notice Emitted when a victim's soul is cleansed of the Solar Plexus Clown Glider.
    /// @param soul Address of the wallet that has been cleansed. No longer a victim.
    event Cleansed(address indexed soul);

    /// @notice Cognitohazard constructor.
    /// @param _lime Limestone token contract that is permitted to mint NFTs.
    constructor(address _lime) {
        LIME = _lime;

        // Some people are just cursed from the start...
        tokenIds++;
        _mint(0x5e334FC2eEc978478E84d17446d842bBd8C5Af7D, tokenIds); // @dev Bellokim
        curse[0x5e334FC2eEc978478E84d17446d842bBd8C5Af7D] = tokenIds;
        tokenIds++;
        _mint(0x34833AB677F5CC40A44A36330Fb18fEf2aC4F03B, tokenIds); // @dev Nicolas
        curse[0x34833AB677F5CC40A44A36330Fb18fEf2aC4F03B] = tokenIds;
        tokenIds++;
        _mint(0x1f2B0633BB0623dCCebE57932d6731Ae93f5213E, tokenIds); // @dev Burt Rock
        curse[0x1f2B0633BB0623dCCebE57932d6731Ae93f5213E] = tokenIds;
        tokenIds++;
        _mint(0xEceE5497b9dbB82E1804E3224F67D00d8d891c69, tokenIds); // @dev Lono
        curse[0xEceE5497b9dbB82E1804E3224F67D00d8d891c69] = tokenIds;
    }

    /// @notice Mark a victim's wallet with the Solar Plexus Clown Glider.
    /// @param _victim Address of the victim wallet to mark.
    function mark(address _victim) external {
        if (curse[_victim] != 0) return;
        require(msg.sender == LIME, NotLime());

        // Place the mark.
        tokenIds++;
        _mint(_victim, tokenIds);

        // Broadcast for the entire world to see.
        emit Marked(msg.sender);
    }

    /// @notice Cleanses the soul (wallet) of a victim in exchange for a small fee in LIME tokens.
    function makeItStop() external {
        uint256 curseId = curse[msg.sender];
        uint256 fee = cleanseFee;
        require(curseId != 0, NotCursedGoAway());

        // Cleanse the soul.
        _burn(curseId);
        curse[msg.sender] = 0;
        LIME.safeTransferFrom(msg.sender, address(this), fee);
        IERC20Extended(LIME).burn(fee);

        // They are now free. Let us all rejoice.
        emit Cleansed(msg.sender);
    }

    /// @notice Set the fee in LIME tokens that must be burnt to cleanse.
    /// @param _cleanseFee The new cleanse fee.
    function setCleanseFee(uint256 _cleanseFee) external {
        cleanseFee = _cleanseFee;
    }

    /// @notice Sets the image URI source for the NFT.
    /// @param _uri New image URI.
    function setUriSource(string calldata _uri) external {
        uriSource = _uri;
    }

    /// @notice An overriden version of `safeTransferFrom` that turns the token into a Soulbound NFT.
    function safeTransferFrom(address, address, uint256) public payable override {
        revert Soulbound();
    }

    /// @notice Cognitohazard name.
    function name() public pure override returns (string memory) {
        return "The Mark of the Solar Plexus Clown Glider";
    }

    /// @notice Cognitohazard symbol.
    function symbol() public pure override returns (string memory) {
        return "SPCG";
    }

    /// @notice Solar Plexus Clown Glider.
    function tokenURI(uint256) public view override returns (string memory) {
        return uriSource;
    }
}
