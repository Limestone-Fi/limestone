// SPDX-License-Identifier: Unlicense
pragma solidity >=0.5.0;

interface IUniswapV2Pair {
    event Approval(address indexed owner, address indexed spender, uint256 value);
    event Transfer(address indexed from, address indexed to, uint256 value);

    function name() external view returns (string memory);

    function symbol() external view returns (string memory);

    function decimals() external view returns (uint8);

    function totalSupply() external view returns (uint256);

    function balanceOf(address owner) external view returns (uint256);

    function allowance(address owner, address spender) external view returns (uint256);

    function approve(address spender, uint256 value) external returns (bool);

    function transfer(address to, uint256 value) external returns (bool);

    function transferFrom(address from, address to, uint256 value) external returns (bool);

    function mint(address) external;

    function burn(address) external returns (uint256, uint256);

    function token0() external view returns (address);

    function token1() external view returns (address);

    function tokens() external view returns (address, address);

    function getReserves() external view returns (uint256 reserve0, uint256 reserve1, uint32 blockTimestampLast);

    function price0CumulativeLast() external view returns (uint256);
}
