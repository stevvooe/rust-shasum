# rust-shasum

Learning rust - walk fs and sum everything, with Go example.

Build the example with the following:

```
cargo build --release
```

Run with the following:

```console
time ./target/release/shasumrust src target
```

Compare with the Go version:

```console
$ go build -o shasumgo shasum.go
$ time ./shasumgo src target
```

Go version runs faster as of Go 1.8.1 and a release build of the binary.
Haven't had time to understand why, but the current guess is that the sha-2
implementation in Go is better optimized.

# FAQ

## What can I use this for?

Nothing. This is just for me to learn rust. You can probably use it that way,
although there are better resources.

## Why are you comparing with Go?

Because, I mostly use Go today and have had a good experience with its IO
model.
