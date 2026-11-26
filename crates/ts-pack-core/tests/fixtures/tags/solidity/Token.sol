// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IToken {
    function balanceOf(address owner) external view returns (uint256);
}

contract Token is IToken {
    mapping(address => uint256) private balances;

    function balanceOf(address owner) external view returns (uint256) {
        return balances[owner];
    }
}
