// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

// Template for RollupBridge
contract RollupBridge {
    address public rollupContract;
    address public owner;

    constructor(address _rollupContract) {
        rollupContract = _rollupContract;
        owner = msg.sender;
    }

    function submitTransactions(bytes memory _transactions) public {
        require(msg.sender == owner, "Owner only");
        // TODO: compress & validate transactions

        (bool success, bytes memory result) =  rollupContract.call(_transactions);
        require(success, "Transaction submission failed");
        // post updated rollup state
    }
}

// Template for Rollup
// TODO: This can be used as the template for a contract that we autogenerate from the SDK. We may not need it.
contract Rollup {
    mapping(address => uint256) public balances;
    uint256 public nonce;

    function deposit() public payable {
        balances[msg.sender] += msg.value;
    }

    // This is basically a standard withdraw function - the only difference is that it involves proofs
    function withdraw(uint256 amount, bytes32[] calldata proof) public {
        bytes32 root = generateMerkleRoot(amount, proof);
        require(verifyProof(root, amount, proof), "Invalid proof");
        require(balances[msg.sender] >= amount, "not enough funds");
        balances[msg.sender] -= amount;
        (bool success, ) = msg.sender.call{value: amount}("");
        require(success, "failure to withdraw funds");
    }

    function generateMerkleRoot(uint256 amount, bytes32[] calldata proof) public pure returns (bytes32) {
        // encodePacked is used to prepare data for hashing - it tightly packs the data into a byte array, removes padding, etc.
        // It produces a compact byte array which is then hashed.
        bytes32 root = keccak256(abi.encodePacked(amount));
        for (uint256 i = 0; i < proof.length; i++) {
            root = keccak256(abi.encodePacked(root, proof[i]));
        }
        return root;
    }

    function verifyProof(bytes32 root, uint256 amount, bytes32[] calldata proof) public pure returns (bool) {
        bytes32 hash = keccak256(abi.encodePacked(amount));
        for (uint256 i = 0; i < proof.length; i++) {
            if (proof[i] < hash) {
                hash = keccak256(abi.encodePacked(hash, proof[i]));
            } else {
                hash = keccak256(abi.encodePacked(proof[i], hash));
            }
        }
        return root == hash;
    }
}
