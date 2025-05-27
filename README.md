
# Leuvenshtein: Efficient FHE-based Edit Distance Computation with Single Bootstrap per Cell

This paper presents a novel approach to calculating the Levenshtein (edit) distance within the framework of Fully Homomorphic Encryption (FHE), specifically targeting third-generation schemes like TFHE. Edit distance computations are essential in applications across finance and genomics, such as DNA sequence alignment. We introduce an optimised algorithm that significantly reduces the cost of edit distance calculations called Leuvenshtein. This algorithm specifically reduces the number of programmable bootstraps (PBS) needed per cell of the calculation, lowering it from approximately 99 operations - required by the conventional Wagner-Fisher algorithm - to just 1. Additionally, we propose an efficient method for performing equality checks on characters, reducing ASCII character comparisons to only 2 PBS operations. Finally, we explore the potential for further performance improvements by utilising preprocessing when one of the input strings is unencrypted. Our Leuvenshtein achieves up to $205\times$ faster performance compared to the best available TFHE implementation and up to $39\times$ faster than an optimised implementation of the Wagner-Fisher algorithm. Moreover, when offline preprocessing is possible due to the presence of one unencrypted input on the server side, an additional $3\times$ speedup can be achieved.

## Code repository

This repository contains a Rust implementation of the Leuvenshtein algorithm. 

Requirements: 
- Rustc v1.72
- TFHE-rs v0.7.2