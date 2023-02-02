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
- prefix filtering on node names
- neighboring `n` nodes of the currently selected node

## Keybindings

### Navigation

Key | Effect
--- | ---
`q` | quit
`c` | close the current tab (except for the root tab)
`tab`/`backtab` | navigate tabs or autocomplete input keyword
`up`/`k` | traverse the focused node list
`down`/`j` | traverse the focused node list
`right`/`l` | move focus between lists (highlighted in yellow borders)
`left`/`h` | move focus between lists (highlighted in yellow borders)
`enter` | goto the selected (prev/next) node

### Search

Key | Effect
--- | ---
`/[node-id-pattern]` (e.g. `/g1s35t`) | search for node by fuzzy matcher
`r[node-prefix]` (e.g. `r(H: ., D: .)`) | search for node by regex matcher (matched on raw dot file)
`enter` | goto the selected (searched) node

### Making Subgraphs

Key | Effect
--- | ---
`f[node-prefix]` (e.g. `fgraph1_subgraph34`) | apply filter with prefix
`enter` | apply filter with the given prefix (opens a new tab)
`e` | export the current tab to dot
`0-9` | export the subgraph containing neighbors of the currently selected node with given depth (in digits)
`x` | launch xdot, showing `./exports/current.dot`

All exported files are saved in `exports` directory in the project root.
Most recently exported file is copied in `exports/current.dot`.
