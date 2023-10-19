// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "forge-std/console.sol";
import "forge-std/Test.sol";
import "../uniswap_v2/GetUniswapV2PairsBatchRequest.sol";
import "../uniswap_v2/GetUniswapV2PoolDataBatchRequest.sol";

contract GasTest is Test {
    function setUp() public {}

//
//    function testUniswapV2() public {
//        address dex = 0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f;
//        uint from = 0;
//        uint step = 766;
//        GetUniswapV2PairsBatchRequest batchContract = new GetUniswapV2PairsBatchRequest(
//            from,
//            step,
//            dex
//        );
//
////        console.logBytes(address(batchContract).code);
//    }

    function testUniswapV2Data() public {
        address[] memory pools = new address[](1);
        pools[0] = address(0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc);

        GetUniswapV2PoolDataBatchRequest batchContract = new GetUniswapV2PoolDataBatchRequest(pools);

        console.logBytes(address(batchContract).code);
    }
}
