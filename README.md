# dot-viewer

`dot-viewer` is a dot-format graph debugger in TUI, inspired by Vim.

# 1. Getting Started

## a. Prerequisites

### i. Graphviz

`dot-viewer` parses a dot format file using C bindings to [Graphviz (v7.0.6)](https://gitlab.com/graphviz/graphviz/-/tree/7.0.6/lib).

The system environment should be able to find and include the following header files.

```C
#include <gvc.h>
#include <cgraph.h>
```

#### Option 1. Installing Graphviz from Package Manager

Coming from Linux,
```console
$ sudo apt install graphviz-dev
```

Coming from Mac,
```console
$ brew install graphviz
```

And coming from Apple Silicon Mac, and [add an environment variable](https://apple.stackexchange.com/questions/414622/installing-a-c-c-library-with-homebrew-on-m1-macs),
```shell
export CPATH=/opt/homebrew/include
```

#### Option 2. Building Graphviz from Source

Or, try building from the source code following the [guide](https://graphviz.org/download/source/).

### ii. xdot.py

`dot-viewer` renders a subgraph with `xdot.py`, an interactive dot visualizer.

It is required that [xdot is executable in command-line](https://github.com/jrfonseca/xdot.py) beforehand such that the following works.
```console
$ xdot *.dot
```

### iii. Others

Coming from Linux, the followings are necessary for `bindgen` to make bindings to Graphviz.
```console
$ sudo apt install build-essentials cmake
$ sudo apt install clang
```

## b. Installation

### i. Initialize

First initialize and update the submodule `dot-graph`.

```console
$ git submodule init
$ git submodule update
```

### ii. Run

Then run crate.

```console
$ cargo run --release [path-to-dot-file]
```

This will open a TUI screen on the terminal.

# 2. Features

With `dot-viewer`, users may

**traverse the graph in TUI** using,
- goto next/prev node of the currently selected node
- fuzzy search on node name
- regex search on node name and attributes
 

**make and export subgraphs** using,
- subgraph tree selection
- applying filter on search matches
- neighboring `n` nodes of the currently selected node

## Keybindings

### General

Key | Command | Actions
--- | --- | ---
`q` | | quit `dot-viewer`
. | `:help<CR>` | show help
`esc` | . | go back to the main screen

### Mode Switches

Key | From | To
--- | --- | ---
`esc` | All | Normal
`/` | Normal | Fuzzy Search
`r` | Normal | Regex Search
`:` | Normal | Command

### Normal

Key | Actions
--- | ---
`c` | close the current tab(view)
`h/l` | move focus between current, prevs, nexts list
`j/k` | traverse in focused list
`n/N` | move between matched nodes
`tab`/`backtab` | move between tabs

### Search
Key | Actions
--- | ---
`tab` | autocomplete search keyword
`enter` | apply search

e.g., in fuzzy search mode, `/g1_s14_t100` and in regex search mode, `r\(H: ., D: .\)`

### Command

Key | Command | Actions
--- | --- | ---
. | `filter` | apply filter on current matches, opening a new tab(view)
. | `neighbors [depth]` | get up to `depth` neighbors of the current node in a new tab(view)
. | `export [(opt) filename]` | export the current tab(view) to dot
. | `xdot [(opt) filename]` | launch `xdot` with the filename or `exports/current.dot` by default
. | `subgraph` | open a popup showing subgraph tree
`enter` | . | execute command

All exported files are saved in `exports` directory in the project root.

Most recently exported file is copied in `exports/current.dot`.

### Subgraph Popup

Key | Actions
--- | ---
`h/j/k/l` | traverse the tree
`enter` | change root to the selected subgraph, opening a new tab(view)

### Help Popup

Key | Actions
--- | ---
`h/j/k/l` | traverse help messages
