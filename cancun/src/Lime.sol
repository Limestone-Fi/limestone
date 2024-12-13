// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;
/*
import {ERC20} from "@openzeppelin/token/ERC20/ERC20.sol";
import {Ownable} from "@openzeppelin/access/Ownable.sol";
import {OFT} from "@layerzerolabs/oft-evm/contracts/OFT.sol";
import {Cognitohazard} from "./Cognitohazard.sol";

/// @title LIME Token contract.
/// @author Chainvisions
/// @notice Limestone OFT contract.

contract Lime is OFT, Ownable {
    /// @notice Thrown when `mintingComplete` is true.
    error MintingComplete();

    /// @notice Flag for whether or not LIME token minting is complete.
    bool public mintingComplete;

    /// @notice Cognitohazard contract.
    address public hazard;

    /// @notice Contracts that when LIME are sent to, invoke the mark.
    mapping(address => bool) public contained;

    /// @notice LIME token constructor.
    /// @param _name Token name.
    /// @param _symbol Token symbol.
    /// @param _endpoint Layer Zero endpoint.
    /// @param _delegate Contract owner/delegate.
    constructor(string memory _name, string memory _symbol, address _endpoint, address _delegate)
        OFT(_name, _symbol, _endpoint, _delegate)
        Ownable(_delegate)
    {}

    /// @notice Burns LIME tokens from the caller.
    /// @param _amount Amount of LIME tokens to burn.
    function burn(uint256 _amount) external {
        _burn(msg.sender, _amount);
    }

    /// @notice Mints new LIME tokens.
    /// @param _to Address to mint tokens to.
    /// @param _amount Amount of LIME tokens to mint.
    function mint(address _to, uint256 _amount) external onlyOwner {
        _mint(_to, _amount);
    }

    /// @notice Used to disable LIME minting once it is complete.
    function completeMinting() external onlyOwner {
        mintingComplete = true;
    }

    /// @notice Sets the current cognitohazard contract.
    /// @param _hazard Hazard contract address.
    function setHazard(address _hazard) external onlyOwner {
        hazard = hazard;
    }

    /// @notice Updates the containment status of a contract.
    /// @param _contained Address of the contract being put in containment.
    /// @param _status Status of the contained contract.
    function updateContainment(address _contained, bool _status) external onlyOwner {
        contained[_contained] = _status;
    }

    /// @dev Transfer hook. Used for marking addresses that sell LIME.
    function _beforeTokenTransfer(address _from, address _to, uint256 _amount) internal override {
        if (contained[_to]) {
            try Cognitohazard(hazard).mark(_from) {} catch {}
        }
    }
}*/
