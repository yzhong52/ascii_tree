# Ascii Tree

A command line tool for drawing tree structures with ascii characters.

## Usage

The input file follows the markdown syntax:

```
#Root
##Child 1
###Grandchild 1
###Grandchild 2
```

Here, an extra `#` indicates a nested child.

With the above content saved in an input file `examples/with_grandchildren.md`, we can render the tree like this:

```
ascii_tree --input examples/with_grandchildren.md
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

### Other Styles

With thick lines:

```
ascii_tree --input examples/with_grandchildren.md --style thick
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
ascii_tree --input examples/with_grandchildren.md --style double
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

## Development

### Install Rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

See details in https://www.rust-lang.org/tools/install.
### Build & Run

```
cargo run -- --input examples/with_grandchildren.md
```

### Install

```
cargo install --path .
```

By default, it will be installed under `/Users/<YOUR_USERNAME>/.cargo/bin/ascii_tree`.

### Unittest

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
