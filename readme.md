# Ascii Tree

A command line tool for drawing tree structures with ascii characters.

- [Ascii Tree](#ascii-tree)
  - [Usage](#usage)
    - [Input From File](#input-from-file)
    - [Horizontal Tree](#horizontal-tree)
    - [Vertical Tree](#vertical-tree)
      - [Virtual Tree Styles](#virtual-tree-styles)
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

astree 0.2.3
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

For example, we can use it like this:

```
astree horizontal -i "$(cat << 'EOF'
# Root
## Child 1
### Grandchild 1
### Grandchild 2
EOF
)"
```

Here, each additional `#` indicates a nested child. 

Output:

```
Root
└─ Child 1
   ├─ Grandchild 1
   └─ Grandchild 2
```

### Input From File

Alternatively, we can also save the markdown file, such as `tree.md`.

```
# Root
## Child 1
### Grandchild 1
### Grandchild 2
```

And invoke the command like so:

```
astree horizontal -i tree.md
```

Output:

```
Root
└─ Child 1
   ├─ Grandchild 1
   └─ Grandchild 2
```

### Horizontal Tree

Example of drawing a horizontal tree:

```
$ astree horizontal -i examples/with_grandchildren_0.md
Root
├─ Child 1
│  ├─ Grandchild 1.1
│  └─ Grandchild 1.2
├─ Child 2
│  └─ Child 2.1
└─ Child 3
```

Example of drawing a tree with multiple root nodes:
```
$ astree horizontal -i examples/multi_tree.md          
.
├─ Root 1
│  ├─ Child 1.1
│  │  ├─ Grandchild 1.1.1
│  │  └─ Grandchild 1.1.2
│  └─ Child 1.2
└─ Root 2
   └─ Child 2.1
```
### Vertical Tree

Example of drawing a vertical tree:

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

Example of drawing a forest with multiple root nodes:

```
$ astree vertical -i examples/multi_tree.md    
                               ┌────────┐                
                               │ Root 1 │                
                               └───┬────┘                
                    ┌──────────────┴──────────────┐      
              ┌─────┴─────┐                 ┌─────┴─────┐
              │ Child 1.1 │                 │ Child 1.2 │
              └─────┬─────┘                 └───────────┘
         ┌──────────┴──────────┐                         
┌────────┴─────────┐  ┌────────┴─────────┐               
│ Grandchild 1.1.1 │  │ Grandchild 1.1.2 │               
└──────────────────┘  └──────────────────┘               
  ┌────────┐ 
  │ Root 2 │ 
  └───┬────┘ 
┌─────┴─────┐
│ Child 2.1 │
└───────────┘
```

#### Virtual Tree Styles

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
astree vertical --input examples/with_children_2.md --style balloon2
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

See details at https://www.rust-lang.org/tools/install.

### Build & Run

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

### Install Astree

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
