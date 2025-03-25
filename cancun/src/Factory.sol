// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Factory} from "@solidstate/factory/Factory.sol";

/// @title Limestone Factory
/// @author Chainvisions
/// @notice A CREATE2 factory used for deploying Limestone contracts.
contract LimestoneFactory {
    error NotDeployer();

    /// @notice Addresses permitted to make deployments.
    mapping(address => bool) public deployers;

    /// @notice Factory constructor.
    constructor() {
        deployers[msg.sender] = true;
    }

    /// @notice Deploys a contract.
    /// @param _bytecode Contract bytecode.
    /// @param _salt Contract salt.
    /// @return Contract address.
    function deploy(bytes calldata _bytecode, bytes32 _salt) external returns (address) {
        require(deployers[msg.sender], NotDeployer());
        return Factory.deploy(_bytecode, _salt);
    }

    /// @notice Deploys a contract.
    /// @param _bytecode Contract bytecode.
    /// @param _salt Contract salt.
    /// @return Contract address.
    function deployWithMemory(bytes memory _bytecode, bytes32 _salt) external returns (address) {
        require(deployers[msg.sender], NotDeployer());
        return Factory.deploy(_bytecode, _salt);
    }

    /// @notice Adds a permitted deployer.
    /// @param _deployer Deployer to add.
    function addDeployer(address _deployer) external {
        require(deployers[msg.sender], NotDeployer());
        deployers[_deployer] = true;
    }

    /// @notice Removes a permitted deployer.
    /// @param _deployer Deployer to add.
    function removeDeployer(address _deployer) external {
        require(deployers[msg.sender], NotDeployer());
        deployers[_deployer] = true;
    }
}
