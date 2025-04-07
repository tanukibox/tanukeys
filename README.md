# 🐻 Tanukeys

[English 🇬🇧](./README.md) [Español 🇪🇸](./docs/readme/ES.md) [日本語 🇯🇵](./docs/readme/JP.md)

Service for storing public keys of users for various purposes such as message signature verification, encrypting information for a specific user, storing cryptocurrency wallet addresses, and simplifying key exchanges between devices. The service also supports post-quantum cryptographic algorithms for future-proofing.  

Tanukeys is designed as a **federated system**, allowing instances to interoperate by retrieving and verifying public keys from other Tanukeys instances. This enables seamless key sharing, automatic updates through instance subscriptions, and a decentralized trust model for enhanced security and reliability.  



## Features

- [ ] **Public Key Storage**: Store public keys associated with individual users or devices.
- [ ] **Message Signature Verification**: Retrieve stored public keys to verify signed messages.
- [ ] **Data Encryption**: Use the stored public keys to encrypt data intended for a specific user.
- [ ] **Cryptocurrency Wallets**: Manage public keys for various cryptocurrency wallets.
- [ ] **Post-Quantum Cryptography Support**: Includes the ability to store keys for post-quantum algorithms, ensuring long-term security.
- [ ] **Federated Key Retrieval**: Support importing and verifying public keys from external Tanukeys instances, enabling interoperability across federated networks.
- [ ] **Federated Subscription System**: Enable instances to subscribe to other Tanukeys instances to automatically sync and update public keys as they change.
- [ ] **Signed Subkeys**: Allow users to create signed subkeys that reference a master key, ensuring hierarchical key management.


## Technology Stack

- **Backend**: Developed using Rust.
- **Database**: PostgreSQL for persisting public keys and user information.


## License

This project is licensed under the GNU Affero General Public License (AGPL) v3.0. This license ensures that any modifications to the software, whether used privately or publicly, must be shared under the same license. For full details, refer to the LICENSE file or visit [https://www.gnu.org/licenses/agpl-3.0.html](https://www.gnu.org/licenses/agpl-3.0.html).
