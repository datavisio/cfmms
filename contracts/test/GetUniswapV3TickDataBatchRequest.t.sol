// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "forge-std/console.sol";
import "forge-std/Test.sol";
import "../uniswap_v3/GetUniswapV3TickDataBatchRequest.sol";
import "../uniswap_v3/SyncUniswapV3PoolBatchRequest.sol";

contract GasTest is Test {
    function setUp() public {}

    function testUniswapV3() public {
        address pool = 0x6c6Bc977E13Df9b0de53b251522280BB72383700;
        bool zeroForOne = true;
        (, int24 currentTick, , , , , ) = IUniswapV3Pool(pool).slot0();
        uint16 numTicks = 10;
        int24 tickSpacing = 10;
        GetUniswapV3TickDataBatchRequest batchContract = new GetUniswapV3TickDataBatchRequest(
            pool,
            zeroForOne,
            currentTick,
            numTicks,
            tickSpacing
        );

//        console.logBytes(address(batchContract).code);
    }
}
