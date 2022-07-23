# Ascii Tree

A command line tool for drawing tree structures with ascii characters.

## Usage

The input file has the following format:

```
Root
#Child 1
##Grandchild 1
##Grandchild 2
```

`#` indicates a nested child.

With the above content in a file `examples/with_grandchildren.txt`, we can render the tree like this:

```
cargo run -- --input examples/with_grandchildren.txt

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