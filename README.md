# docker-volume-rs

## Overview
A helper package to create docker volumes in Rust, inspired by https://github.com/docker/go-plugins-helpers

## Usage 
1. Implement the `VolumeDriver`	async_trait
```rust
#[async_trait]
impl VolumeDriver for XetDriver {
```
2. Initialize a `VolumeHandler` with either TCP or Unix Sockets
```rust
let driver = XetDriver::new(args.mount_root);
let handler = VolumeHandler::new(driver);
```
3. Call either `run_tcp` or `run_unix_socket` from the `VolumeHandler`
```rust
handler.run_tcp(tcp_args.port).await?;
```

## Installation
Add the following to your Cargo.tom
```toml
[dependencies]
docker-volume = "0.1.0"

