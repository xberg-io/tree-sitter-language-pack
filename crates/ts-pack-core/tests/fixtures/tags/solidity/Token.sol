// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./IERC20.sol";

uint256 constant MAX_SUPPLY = 1_000_000;

error Unauthorized(address caller);

type Price is uint256;

function clamp(uint256 value) pure returns (uint256) {
    return value > MAX_SUPPLY ? MAX_SUPPLY : value;
}

interface IToken {
    event Transfer(address indexed from, address indexed to, uint256 amount);

    function balanceOf(address owner) external view returns (uint256);
}

library SafeMath {
    function add(uint256 a, uint256 b) internal pure returns (uint256) {
        return a + b;
    }
}

contract Token is IToken {
    using SafeMath for uint256;

    struct Account {
        uint256 balance;
        bool frozen;
    }

    enum Status { Active, Paused }

    uint256 public constant DECIMALS = 18;
    address public immutable owner;
    uint256 public totalSupply;
    mapping(address => Account) private accounts;

    modifier onlyOwner() {
        if (msg.sender != owner) revert Unauthorized(msg.sender);
        _;
    }

    constructor() {
        owner = msg.sender;
    }

    function balanceOf(address holder) external view returns (uint256) {
        return accounts[holder].balance;
    }

    function mint(address to, uint256 amount) external onlyOwner {
        uint256 capped = clamp(amount);
        accounts[to].balance = accounts[to].balance.add(capped);
        totalSupply += capped;
        emit Transfer(address(0), to, capped);
    }
}
