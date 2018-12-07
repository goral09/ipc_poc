# Inter process communication in Rust using proto models

## Generate models from protos

```
cd protos
cargo run
```
## Run server
```
cd server
cargo run -e __path_to_socket_file__
```

## Send data
```
cd client
cargo run __path_to_socket_file__ __name__ __age__
```
