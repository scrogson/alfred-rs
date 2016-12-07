# Alfred

> A slackbot built in [Rust]

This doesn't really do much at the moment other than log in and log messages to
STDOUT.

[Rust]: https://www.rust-lang.org/en-US/

## Usage

Build the package

```sh
λ cargo build
```

Run it with `cargo`

```sh
λ cargo run -- -h
```

or

```sh
λ ./target/debug/alfred -h
```

```sh
USAGE:
    alfred --token <token>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --token <token>    Slack token
```
