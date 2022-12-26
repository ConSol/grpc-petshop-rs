# grpc-petshop-rs

**Showcasing minimalistic full-stack project using gRPC** with a Rust backend and a Browser frontend.

More specific:

- **Rust** using `tokio`, `tonic`
- **Browser** using `solid-js`

<img src="assets/rust-logo.png" width="100" />
<img src="assets/grpc-logo.png" width="100" />
<img src="assets/solidjs-logo.png" width="100" />

## Installation

- [server README](./server/README.md)
- [client README](./client/README.md)

## About the stack

Technologies in the stack are targeting:

- **fast and simple development cycle**
  - client (hot-)reloading: ~1s
    - website
    - API contract changes
  - server auto-compiling and restarting: ~3s
    - implementation
    - API contract changes
- **contract-first** API approach
- **resource-saving** in any way
  - server
    - binary ~4MB
    - memory ~3MB
    - high-performance native application
  - client
    - packaged ~90kB
    - high-performance reactive framework

## Support matrix

- primarily tested on MacOS, the setup should also work for Linux
- as the setup is using a bash script it will not run on Windows natively, but maybe on WSL

| Support | Client | Server |
|----|----|----|
|Linux|yes|yes|
|MacOS|yes|yes|
|Windows|unknown|yes|