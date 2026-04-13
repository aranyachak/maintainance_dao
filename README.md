# maintainance_dao
# 🛠️ Maintenance DAO (Stellar Soroban)
<img width="2879" height="1415" alt="image" src="https://github.com/user-attachments/assets/ac42ac45-76a9-4531-828d-9cff21852015" />


## Project Description
The **Maintenance DAO** is a decentralized autonomous organization built on the Stellar network using Soroban smart contracts. It is designed to decentralize the management, funding, and execution of shared maintenance tasks—whether for a physical community (like a housing cooperative or shared workspace) or a digital infrastructure project. By leveraging blockchain technology, the DAO ensures total transparency in how maintenance funds are requested, approved, and deployed.

## What it Does
This smart contract allows community members to democratically manage a shared treasury dedicated to upkeep and repairs. Instead of relying on a centralized property manager or administrator to decide where funds go, community members can:
1. **Submit Proposals:** Request funds for specific maintenance tasks (e.g., "Repair the community roof," "Upgrade server infrastructure").
2. **Vote:** Community members can review pending proposals and cast their votes on-chain.
3. **Execute:** Once a proposal reaches a predetermined consensus threshold, the contract allows the proposal to be executed, which will ultimately trigger the release of funds to the contractor or service provider.

## Features
* **Decentralized Proposal Creation:** Any authenticated user can submit a detailed proposal, including a text description and the exact requested budget (`amount`).
* **On-Chain Voting:** Transparent, immutable vote tracking ensuring that every community member's voice is accurately recorded.
* **Automated Threshold Execution:** Proposals cannot be executed until they meet a strict voting threshold (currently set to a demo threshold, configurable for production).
* **State Persistence:** Built on Soroban's efficient storage models (`instance` and `persistent` storage) to ensure proposals live securely on the Stellar network.
* **Secure Authorization:** Utilizes Soroban's native `require_auth()` to ensure only the actual wallet owners can propose tasks or cast votes.

## Deployed Smart Contract Link
**Network:** Stellar Testnet / Futurenet  
**Contract ID:** `CCCFVIKRQA66HGOLLCIVAZFI6BYQHJXJ7MECGUT7K225VDHBJ5UKGG3D`  
**Link**: https://stellar.expert/explorer/testnet/contract/CCCFVIKRQA66HGOLLCIVAZFI6BYQHJXJ7MECGUT7K225VDHBJ5UKGG3D



