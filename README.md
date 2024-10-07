
# IDONK – Decentralized Knowledge Verification Platform
**"You know what I don't know."**

![Screenshot of IDONK – Decentralized Knowledge Verification Platform](https://github.com/user-attachments/assets/7b33b537-2d83-4a3e-9de0-1408bd31fcb0)

**IDONK** aims to address misinformation by offering a decentralized platform where users can post, validate, and authenticate knowledge. By leveraging blockchain technology and a staking mechanism, the platform ensures that only verified, authentic information is rewarded and preserved, creating a trustworthy library. Users are incentivized through the IDONK token to actively participate in communities, vote on the accuracy of ideas, and contribute knowledge.

### Repository Overview:
This repository contains all the smart contracts related to this project:

- **Communities**: Handles computations for manipulating and viewing concepts related to the community platform.
- **Connector**: The major view function, connecting to other contracts via Interface Call to create a safe view platform.
- **ContentConnector**: A safe contract used for users to submit their content or ideas.
- **ContentWork**: The raw contract dealing with content submission and manipulation (unsafe).
- **CustomERC20**: Manages the IDONK token.
- **Reward**: Contract used for claiming rewards from content.
- **SafeUserRegistration**: Ensures safe user registration.
- **UserMetadata**: Manages, stores, and retrieves users' unique names and metadata.
- **UserProfiles**: (Unsafe) Contract managing system state data of users.
- **Voters**: (Unsafe) Contract managing voting, redirection, and laws regarding voting.
- **VotingConnector**: A safe contract that gets the stake and votes of users concerning content.

![Communities Overview](https://github.com/user-attachments/assets/dfd8b3ea-96bc-4bef-9a27-92cbcbd26088)

---
