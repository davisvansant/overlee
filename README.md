[![Build Status](https://cloud.drone.io/api/badges/davisvansant/overlee/status.svg)](https://cloud.drone.io/davisvansant/overlee)

~ o v e r l e e  

A [Rentals](https://en.wikipedia.org/wiki/The_Rentals) Discography built with `gRPC` and `Rust`

#### Testing

Start the Server - `cargo run --bin overlee_server --release`

Run the Client - `cargo run --bin overlee --release`

- Albums Service
```
grpcurl -plaintext -import-path ./overlee_build/proto -proto the_rentals.proto -format "text"  localhost:8080 the_rentals.Discography/Albums
```
- Eps Service
```
grpcurl -plaintext -import-path ./overlee_build/proto -proto the_rentals.proto -format "text"  localhost:50052 the_rentals.Discography/Eps
```

- Singles Service
```
grpcurl -plaintext -import-path ./overlee_build/proto -proto the_rentals.proto -format "text"  localhost:8080 the_rentals.Discography/Singles
```
