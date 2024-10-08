# Ascii Tree

A command line tool for drawing tree structures with ascii characters.

  - [Installation](#installation)
    - [Install Via Homebrew](#install-via-homebrew)
    - [Install Via Cargo](#install-via-cargo)
  - [Usage](#usage)
    - [Input From File](#input-from-file)
    - [Horizontal Tree](#horizontal-tree)
    - [Vertical Tree](#vertical-tree)
      - [Vertical Tree Styles](#vertical-tree-styles)
      - [Vertical Tree Maximum Label Width](#vertical-tree-maximum-label-width)
      - [Vertical Tree Horizontal Spacing](#vertical-tree-horizontal-spacing)
      - [Vertical Tree Multi Lines](#vertical-tree-multi-lines)
  - [Development](#development)

## Installation

### Install Via Homebrew

Install homebrew https://brew.sh/:

```
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

Install astree https://github.com/ascii-draw/homebrew-ascii-tree:

```
brew tap ascii-draw/ascii-tree
brew install astree
```

### Install Via Cargo

Install cargo <https://doc.rust-lang.org/cargo/getting-started/installation.html>:

```
curl https://sh.rustup.rs -sSf | sh
```

Install astree from <https://crates.io/crates/astree>:

```
cargo install astree
```

## Usage

Check out the help message:

```
$ astree -h
A command line tool for drawing tree structures with ascii characters

Usage: astree <COMMAND>

Commands:
  vertical    Print the tree virtually. Use 'v' for shorthand
  horizontal  Print the tree horizontally. Use 'h' for shorthand
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information (use `--help` for more detail)
  -V, --version  Print version information
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

#### Vertical Tree Styles

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

#### Vertical Tree Maximum Label Width

To specify the maximum width of the label with `--width <WIDTH>`. For example:

```
astree vertical --input examples/with_long_label.md --width 10
     ┌──────────┐
     │ A Simple │
     │   Root   │
     └────┬─────┘
    ┌─────┴──────┐
┌───┴────┐  ┌────┴────┐
│ A long │  │ Another │
│ child  │  │  long   │
└────────┘  │  child  │
            └─────────┘
```

#### Vertical Tree Horizontal Spacing

By default, there is a two-spaces gap between boxes. This can be set with `--spacing <SPACING>`.

```
astree vertical --input examples/with_many_children.md --width 10 --spacing 0
               ┌──────────┐
               │ A Simple │
               │   Root   │
               └────┬─────┘
    ┌──────────┬────┴─────┬──────────┐
┌───┴────┐┌────┴────┐┌────┴────┐┌────┴─────┐
│ A long ││ Another ││ A third ││ One more │
│ child  ││  long   ││  child  ││  child   │
└────────┘│  child  │└─────────┘└──────────┘
          └─────────┘

astree vertical --input examples/with_many_children.md --width 10 --spacing 10
         ┌──────────┐
         │ A Simple │
         │   Root   │
         └────┬─────┘
    ┌─────────┴──────────┐
┌───┴────┐          ┌────┴────┐
│ A long │          │ Another │
│ child  │          │  long   │
└────────┘          │  child  │
                    └─────────┘
```

#### Vertical Tree Multi Lines

Title lines in your Markdown file define the tree structure. Any content under a title is automatically included as separate lines within the same structural level.

```
astree vertical --input examples/with_content.md
           ┌──────┐
           │ Root │
           └──┬───┘
      ┌───────┴────────┐
┌─────┴──────┐  ┌──────┴──────┐
│ Left Child │  │ Right Child │
│ Quota: 100 │  │ Quota: 200  │
└────────────┘  └─────────────┘
```

## Development

See [development.md](./development.md).
