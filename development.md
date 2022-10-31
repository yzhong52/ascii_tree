
- [Development](#development)
  - [Install Rust](#install-rust)
  - [Build & Run](#build--run)
  - [Install astree](#install-astree)
  - [Unit Tests](#unit-tests)
  - [Add Libraries](#add-libraries)
  - [Release](#release)

# Development

## Install Rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

See details at https://www.rust-lang.org/tools/install.

## Build & Run

Here are some examples of building and running:

```
$ cargo run -- horizontal -i examples/with_grandchildren_0.md
.
└─ Root
   ├─ Child 1
   │  ├─ Grandchild 1.1
   │  └─ Grandchild 1.2
   ├─ Child 2
   │  └─ Child 2.1
   └─ Child 3
```

```
$ cargo run -- vertical -i examples/with_grandchildren_1.md
             ┌──────┐
             │ Root │
             └──┬───┘
           ┌────┴────┐
           │ Child 1 │
           └────┬────┘
       ┌────────┴────────┐
┌──────┴───────┐  ┌──────┴───────┐
│ Grandchild 1 │  │ Grandchild 2 │
└──────────────┘  └──────────────┘
```

## Install astree

Install the local version:

```
git clone git@github.com:yzhong52/ascii_tree.git
cd ascii_tree
cargo install --path .
```

Install from <https://crates.io/crates/astree>:

```
cargo install astree
```

By default, it will be installed under `/Users/<YOUR_USERNAME>/.cargo/bin/astree`.

## Unit Tests

```
cargo test
```

## Add Libraries

```
cargo add <DEP>[@<VERSION>] ...
```

e.g.

```
cargo add num
```

## Release

Step 1: bump version in `Cargo.toml`.

Step 2: publish.

```
cargo publish --token <TOEKN>
```

or

```
cargo login
cargo publish
```
