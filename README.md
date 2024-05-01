# Receipt-Project

## Introduction

Rust project for creating receipts and the corresponding receiptItems

## Installing Rust

Install Rust via:
```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

For more information go to the [***official Rust documentation***](https://www.rust-lang.org/learn/get-started)

## Environment variables

Before the application can be started an `.env` file needs to be created in the root directory.
You can either do it manually or automatically by running following command:
```bash
    cp .env.example .env
```

Note: The `.env.example` already contains every configuration necessary for local development.
For production these values need to be adjusted.

## Starting the application locally

1. Make sure [***Docker***](https://www.docker.com/) is installed & running
2. Run:
    ```bash
    docker-compose up --build
    ```

