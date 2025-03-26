// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

/// @title IMultiModalWorkerPrivilege
/// @author Chainvisions
/// @notice Interface for the privileges expansion to MultiModalWorker.

interface IMultiModalWorkerPrivilege {
    function authorize(address _executor, uint256 _positionId, address _permittee) external;
    function authorizeMany(address _executor, uint256 _positionId, address[] calldata _permittees) external;
    function configurePermissionsForOne(
        address _executor,
        uint256 _positionId,
        address _permittee,
        uint256 _authorizationRoleBits
    ) external;
    function configurePermissionsForMany(
        address _executor,
        uint256 _positionId,
        address[] calldata _permittees,
        uint256[] calldata _authorizationRoleBits
    ) external;
}
