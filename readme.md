# Ascii Tree

A command line tool for drawing tree structures with ascii characters.

## Usage

Install from [crates.io](https://crates.io/crates/astree) with `cargo install astree`.

Create an input file following the markdown syntax:

```
# Root
## Child 1
### Grandchild 1
### Grandchild 2
```

Here, an extra `#` indicates a nested child.

With the above content saved in an input file `examples/with_grandchildren.md`, we can render the tree like this:

```
astree --input examples/with_grandchildren.md
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
astree --input examples/with_grandchildren.md --style thick
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
astree --input examples/with_grandchildren.md --style double
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

With special top connection:

```
astree --input examples/with_grandchildren.md --top-connection ▼
             ┌──────┐             
             │ Root │             
             └──┬───┘             
           ┌────▼────┐            
           │ Child 1 │            
           └────┬────┘            
       ┌────────┴────────┐        
┌──────▼───────┐  ┌──────▼───────┐
│ Grandchild 1 │  │ Grandchild 2 │
└──────────────┘  └──────────────┘
```

With chest style:

```
astree --input examples/with_grandchildren_2.md --style chest
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
astree --input examples/with_children_2.md --style balloon --top-connection '☐' --bottom-connection '┰'`
   ╭───────────╮    
   │ Root Node │    
   ╰─────┰─────╯    
    ╭────┴─────╮    
╭───☐───╮  ╭───☐───╮
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

```
cargo run -- --input examples/with_grandchildren.md
```

### Install

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
