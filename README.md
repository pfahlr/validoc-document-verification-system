# Validoc

**Validoc** is a secure document verification and storage system that uses IPFS for decentralized storage and Ethereum for cryptographic proof of authenticity. It is designed to operate as a command-line tool, a web service, and a frontend application, with extensibility for mobile platforms in the future.

## Features

- **Document Hashing**: Generate cryptographic hashes of documents.
- **Decentralized Storage**: Upload documents to IPFS.
- **Blockchain Proof**: Store and verify document authenticity on Ethereum.
- **JWT-Based Authentication**: Secure API access via short-lived tokens.
- **Cross-platform**: Designed for use on CLI, web, and mobile.
- **Microservice Architecture**: Each service (IPFS, Ethereum, Auth) runs in its own container and scales independently.

---

## Command Line Interface

```sh
validoc upload [options] <file>
validoc hash [options] <file>
validoc verify [options] <file>
```

### Options

- `--api-key <KEY>`: Authenticate using your API key.
- `--ipfs-url <URL>`: Override default IPFS endpoint.
- `--eth-network <NAME>`: Specify Ethereum network (e.g., mainnet, goerli).

---

## Architecture

- **Rust-based CLI** for interacting with the system locally or through scripting.
- **Microservices**:
  - `ipfs-service`: Handles decentralized document storage.
  - `ethereum-service`: Posts document hashes and metadata to the Ethereum blockchain.
  - `auth-service`: Issues and validates JWTs using a shared signing key.
  - `verification-service`: Validates documents against stored hash and blockchain entries.
- **Web Application**:
  - React frontend interacting with services via RESTful APIs.
  - Optionally proxying to the CLI commands for internal use.
- **Deployment**:
  - Dockerized services.
  - Orchestrated using Kubernetes with appropriate node/pod separation.

---

## Authentication

- **API Key**: Used by CLI to authenticate.
- **JWT Tokens**: Issued by `auth-service`, expire after 1–5 minutes.
- **Service Verification**: JWTs are signed with a shared private key and validated statelessly by each service.

---

## Tech Stack

- **Rust**: Primary language for CLI and service logic.
- **PostgreSQL**: Central database for user and key management.
- **IPFS**: Distributed storage for document content.
- **Ethereum**: Blockchain for storing verifiable document hashes.
- **React**: Frontend web application.
- **Docker + Kubernetes**: Containerization and orchestration.

---

## Future Plans

- Mobile clients (Android/iOS)
- Role-based access controls
- Support for multiple blockchain backends
- Integration with legal and academic institutions

---

## Getting Started

### Requirements

- Rust
- Docker
- PostgreSQL
- IPFS node
- Ethereum node or Infura access

### Build CLI

```bash
cargo build --release
```

### Run Services (Dev)

```bash
docker-compose up --build
```
