# Blockchain Overview

## Transition from PoW to PoS with Schnorr Signatures, Blake3 Hashing, and zk-STARKs for Confidentiality

### Proof of Work (PoW) Phase
In the initial phase of our blockchain, the network employs a Proof of Work (PoW) consensus mechanism. Miners compete to solve computational puzzles, and the first to solve the puzzle appends a new block to the chain. This process is underpinned by Schnorr signatures for cryptographic security and Blake3 for efficient hashing. A unique feature of our approach is that miners earn a block reward equal to the network's difficulty level, ensuring a direct correlation between work performed and reward earned. This equitable system eliminates the typical arbitrary work versus reward ratio.

### Proof of Stake (PoS) Phase
Once the blockchain reaches the transition block, it switches to a Proof of Stake (PoS) consensus mechanism. In this phase, validators are chosen based on their stake in the network. The selection process is secure and efficient, utilizing zero-knowledge proofs (zk-STARKs) to ensure confidentiality and integrity of transactions without revealing sensitive information. After the transition block, validators no longer receive block rewards but earn transaction fees, ensuring continued incentivization without inflationary pressures.

### Security and Efficiency
- **Schnorr Signatures**: Our use of Schnorr signatures offers provable security and efficiency, enabling fast and secure cryptographic operations.
- **Blake3 Hashing**: Blake3 provides a highly efficient and secure hashing algorithm, ensuring fast processing times and strong cryptographic guarantees.
- **zk-STARKs**: Zero-Knowledge Scalable Transparent ARguments of Knowledge (zk-STARKs) are integrated to provide scalable and efficient zero-knowledge proofs, ensuring transaction confidentiality and enhancing overall security.

### Proof of Stake Highlights
Based on the paper "A Scalable Proof-of-Stake Blockchain in the Open Setting," our PoS implementation introduces several novel security properties:
1. **Chain Soundness**: Ensures new players can join the protocol securely and their best local chain is consistent with existing honest players.
2. **Greedy Strategies**: Investigates and addresses greedy strategies where players might attempt to extend multiple positions in the blockchain simultaneously, ensuring fair participation.
3. **Adaptive Security**: Achieves adaptive security without assuming secure erasure, enhancing practical deployment in real-world settings.

### Key Properties
- **Common Prefix**: Ensures that the blockchain maintained by honest players shares a common prefix, providing consistency across the network.
- **Chain Quality**: Guarantees a minimum proportion of blocks in the chain are contributed by honest players, ensuring the integrity of the blockchain.
- **Chain Growth**: Ensures the blockchain grows at a reasonable rate, maintaining network efficiency and responsiveness.

### Conclusion
Our blockchain seamlessly transitions from PoW to PoS, leveraging state-of-the-art cryptographic techniques to ensure security, efficiency, and scalability. By combining Schnorr signatures, Blake3 hashing, and zk-STARKs, we create a robust framework for future-proof blockchain technology, addressing key challenges in both PoW and PoS paradigms.

## References
- [A Scalable Proof-of-Stake Blockchain in the Open Setting](https://eprint.iacr.org/2017/656.pdf)
- [Best-Possible Unpredictable Proof-of-Stake: An Impossibility and a Practical Design](https://eprint.iacr.org/2021/660.pdf)
