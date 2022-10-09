# Ascii Tree

A command line tool for drawing tree structures with ascii characters.

- [Ascii Tree](#ascii-tree)
  - [Usage](#usage)
    - [Horizontal Tree](#horizontal-tree)
    - [Vertical Tree](#vertical-tree)
      - [Other Virtual Tree Styles](#other-virtual-tree-styles)
  - [Development](#development)
    - [Install Rust](#install-rust)
    - [Build & Run](#build--run)
    - [Install Astree](#install-astree)
    - [Unit Tests](#unit-tests)
    - [Add Libraries](#add-libraries)
  - [Release](#release)

## Usage

Install from crates.io <https://crates.io/crates/astree>:

```
cargo install astree
```

Check out the help message:

```
$ astree --help

astree 0.2.2
 A command line tool for drawing tree structures with ascii characters

USAGE:
    astree <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help          Print this message or the help of the given subcommand(s)
    horizontal    Print the tree horizontally
    vertical      Print the tree virtually
```

Create an input file following the markdown syntax, such as:

```
# Root
## Child 1
### Grandchild 1
### Grandchild 2
```

Here, an extra `#` indicates a nested child.

And saved that in a file, such as `tree.md`.

### Horizontal Tree

With an input file `examples/with_grandchildren_0.md`, we can render the tree like this:

```
$ astree horizontal -i examples/with_grandchildren_0.md
.
└─ Root
   ├─ Child 1
   │  ├─ Grandchild 1.1
   │  └─ Grandchild 1.2
   ├─ Child 2
   │  └─ Child 2.1
   └─ Child 3
```

### Vertical Tree

With an input file `examples/with_grandchildren_1.md`, we can render the tree like this:

```
astree vertical --input examples/with_grandchildren_1.md
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

#### Other Virtual Tree Styles

With thick lines:

```
astree vertical --input examples/with_grandchildren_1.md --style thick
             ┏━━━━━━┓
             ┃ Root ┃
             ┗━━┳━━━┛
           ┏━━━━┻━━━━┓
           ┃ Child 1 ┃
           ┗━━━━┳━━━━┛
       ┏━━━━━━━━┻━━━━━━━━┓
┏━━━━━━┻━━━━━━━┓  ┏━━━━━━┻━━━━━━━┓
┃ Grandchild 1 ┃  ┃ Grandchild 2 ┃
┗━━━━━━━━━━━━━━┛  ┗━━━━━━━━━━━━━━┛
```

With double lines:

```
astree vertical --input examples/with_grandchildren_1.md --style double
             ╔══════╗
             ║ Root ║
             ╚══╦═══╝
           ╔════╩════╗
           ║ Child 1 ║
           ╚════╦════╝
       ╔════════╩════════╗
╔══════╩═══════╗  ╔══════╩═══════╗
║ Grandchild 1 ║  ║ Grandchild 2 ║
╚══════════════╝  ╚══════════════╝
```

With chest style:

```
astree vertical --input examples/with_grandchildren_2.md --style chest
                       ╔━━━━━━╗
                       ┃ Root ┃
                       ╚━━┳━━━╝
              ╔━━━━━━━━━━━┻━━━━━━━━━━━╗
          ╔━━━┻━━━╗               ╔━━━┻━━━╗
          ┃ Left  ┃               ┃ Right ┃
          ┃ Child ┃               ┃ Child ┃
          ╚━━━┳━━━╝               ╚━━━┳━━━╝
      ╔━━━━━━━┻━━━━━━━╗         ╔━━━━━┻━━━━━━╗
╔━━━━━┻━━━━━━╗  ╔━━━━━┻━━━━━━╗  ┃   Right    ┃
┃ Grandchild ┃  ┃ Grandchild ┃  ┃ Grandchild ┃
┃    (1)     ┃  ┃    (2)     ┃  ╚━━━━━━━━━━━━╝
╚━━━━━━━━━━━━╝  ╚━━━━━━━━━━━━╝
```

With balloon style:

```
astree vertical --input examples/with_children_2.md --style balloon --top-connection ¤ --bottom-connection ¤ 
   ╭───────────╮    
   │ Root Node │    
   ╰─────¤─────╯    
    ╭────┴─────╮    
╭───¤───╮  ╭───¤───╮
│ Child │  │ Child │
│  (1)  │  │  (2)  │
╰───────╯  ╰───────╯
```

## Development

### Install Rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

See details in https://www.rust-lang.org/tools/install.

### Build & Run

Here are some example build and run commands:

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

### Install Astree

Install local version:

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

### Unit Tests

```
cargo test
```

### Add Libraries

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
