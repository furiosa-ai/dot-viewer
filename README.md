# dot-viewer
dot debugger in TUI

# 1. Motivations

커다란 graph를 전부 다 visualize 할 수 없음

- graphviz, xdot, netron와 같은 graph visualizer들이 있지만, node 개수가 수천 개가 넘어가면 rendering 시간이 오래 걸림
- rendering에 성공한다 하더라도, graph 크기가 너무 커서 직관적으로 보기 어려움

# 2. Overview

커다란 graph를 효율적으로 다루고 디버깅할 수 있는 dot-viewer 툴 구현

- dot format의 전체 graph를 input으로 받아서,
- TUI로 전체 graph를 탐색하고,
- (visualize 하기에 충분히 작은) 특정 subgraph만 선택하여 dot format으로 export

# 3. Getting Started

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

# 4. Feature Demo

With `dot-viewer`, users may

**traverse the graph in TUI** using,
- goto next/prev node of the currently selected node
- fuzzy search on node name
- regex search on node name and attributes
  

**make and export subgraphs** using,
- subgraph tree selection
- prefix filtering on node names
- neighboring `n` nodes of the currently selected node

## Keybindings

### General

Key | Actions
--- | ---
`q` | quit
`?` | show help
`esc ` | go back to the main screen
`c` | close the current tab (except for the root tab)

### Navigation

Key | Mode | Actions
--- | --- | ---
`tab`/`backtab` | `Nav` | move between tabs
`up` | `Nav`/`Search` | traverse the focused node list
`down` | `Nav`/`Search` | traverse the focused node list
`right` | `Nav` | move focus between lists (highlighted in yellow borders)
`left` | `Nav` | move focus between lists (highlighted in yellow borders)

### Mode Switch

Key | Mode | Actions
--- | --- | ---
`/` | `Nav` | switch to fuzzy `Search` mode (`/[node-id-pattern]`)
`r` | `Nav` | switch to regex `Search` mode (`r[node-attr-regex]`)
`f` | `Nav` | switch to prefix `Filter` mode (`f[node-id-prefix]`)
`s` | `Nav` | switch to subgraph `Popup` mode
`enter` | `Nav`/`Search` | go to the selected node
--- | `Filter` | apply entered prefix (opens a new tab)
--- | `Subgraph` | extract selected subgraph (opens a new tab)

### Exporting

Key | Mode | Actions
--- | --- | ---
`e` | `Nav` | export the current tab to dot
`0-9` | `Nav` | export the subgraph containing neighbors of the current node with given depth
`x` | `Nav` | launch xdot, showing `./exports/current.dot`

All exported files are saved in `exports` directory in the project root.
Most recently exported file is copied in `exports/current.dot`.
