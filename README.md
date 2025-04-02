# Pokémon GraphQL API (Rust)

## Overview
This is a **high-performance GraphQL API** built with **Rust, Warp, and Async-GraphQL**.  
It fetches data from the [Pokémon API](https://pokeapi.co/) and serves it as a **GraphQL service**.

## Features
- 🚀 **Asynchronous & High Throughput** (Uses **Tokio** for concurrency)
- ⚡ **GraphQL API** with `async-graphql`
- 🌍 **Fetches Pokémon Data** from an external REST API
- ♻️ **Uses Connection Pooling** with Reqwest for efficiency
- 📡 **Fast & Scalable** with Warp

## Tech Stack
- **Rust** 🦀
- **Tokio** (Async Runtime)
- **Warp** (Web Framework)
- **Async-GraphQL** (GraphQL Library)
- **Reqwest** (HTTP Client)
- **Serde** (JSON Serialization)

## Run Locally
```sh
# Clone the repository
git clone https://github.com/vickky06/pokemon-graphql-rust.git
cd pokemon-graphql-rust

# Run the server
cargo run
