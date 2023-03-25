// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

contract Rollup {

    struct RollupBlock {
        bytes32 merkleRoot;
        uint256 timestamp;
        uint256 blockNumber;
        uint256 fee;
    }

    uint256 public totalBlocks;
    mapping (uint256 => RollupBlock) public rollupBlocks;

    function submitBlock(bytes32 _merkleRoot, uint256 _fee) public returns (uint256) {
        require(_merkleRoot != bytes32(0), "Invalid merkle root");
        require(_fee > 0, "Invalid fee");

        uint256 currentBlockNumber = block.number;
        RollupBlock storage newBlock = rollupBlocks[currentBlockNumber];
        newBlock.merkleRoot = _merkleRoot;
        newBlock.timestamp = block.timestamp;
        newBlock.blockNumber = currentBlockNumber;
        newBlock.fee = _fee;

        totalBlocks++;

        return currentBlockNumber;
    }

    function getBlock(uint256 _blockNumber) public view returns (bytes32, uint256, uint256, uint256) {
        RollupBlock storage blockData = rollupBlocks[_blockNumber];
        return (blockData.merkleRoot, blockData.timestamp, blockData.blockNumber, blockData.fee);
    }
}
