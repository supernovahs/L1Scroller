// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console, console2} from "forge-std/Test.sol";
import {L1Scroller} from "../src/L1Scroller.sol";

contract L1ScrollerTest is Test {
    L1Scroller public l1Scroller;

    function setUp() public {
        l1Scroller = new L1Scroller();
    }

    function fake_read_uint(bytes memory result) public view returns (uint256) {
        return l1Scroller.bytesToUint(result);
    }

    function fake_read_string(bytes memory result) public pure returns (string memory) {
        return string(result);
    }

    function test_get_uint() public view {
        uint256 fake_uint_result = fake_read_uint(hex"0000000000000000000000000000000000000000000000000000000000000005");
        assertEq(fake_uint_result, 5);
    }

    function test_get_string() public pure {
        string memory fake_string_result = fake_read_string(hex"68656c6c6f");
        assertEq("hello", fake_string_result);
    }
}
