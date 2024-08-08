# Rust and Go Server with Redis Integration

This project demonstrates a simple integration between Rust, Go, and Redis. The system consists of a Rust server that handles incoming requests, triggers the creation of short-lived Goroutines in a Go server, and stores data sent from these Goroutines into a Redis database.

## Project Overview

### Architecture

1. **Rust Server**:
   - Exposes two endpoints:
     - `/trigger`: Triggers the Go server to create a random number of Goroutines.
     - `/receive`: Receives payload data from Goroutines and stores it in Redis.
   - Uses the Rocket framework for the web server.
   - Connects to Redis to store Goroutine data.

2. **Go Server**:
   - Exposes an endpoint `/create-goroutines` to create short-lived Goroutines.
   - Each Goroutine sends a unique identifier and UTC timestamp to the Rust server.

3. **Redis**:
   - Used to store Goroutine data. Each Goroutine's unique identifier is used as the Redis key, and the data received (timestamps) is stored as a list under this key.

## Prerequisites

- Rust (with `cargo` installed)
- Go
- Redis Server (running on port `6380`)

## Setup

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/your-repo.git
cd your-repo
