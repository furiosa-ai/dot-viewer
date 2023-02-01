# dot-viewer
Dot debugger in TUI

## Prerequisites

### Graphviz

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

### xdot.py

`dot-viewer` renders a subgraph with xdot.py, an interactive dot visualizer.

It is required that [xdot is executable in command-line](https://github.com/jrfonseca/xdot.py) beforehand such that the following works.
```console
$ xdot *.dot
```

## Usage

### Initialize

First initialize and update the submodule `dot-graph`.

```console
$ git submodule init
$ git submodule update
```

### Run

Then run crate.

```console
$ cargo run --release -- [path-to-dot-file]
```

This will open a TUI screen on the terminal.

### Interactions

Users may interact with dot-viewer in TUI to traverse the graph.

Key | Effect
--- | ---
`q` | quit
`c` | close the current tab (except for the root tab)
`e` | export the current tab to dot
`x` | launch xdot, showing `./exports/current.dot`
`0-9` | export the subgraph containing neighbors of the currently selected node with given depth (in digits)
`up`/`k` | traverse the focused node list
`down`/`j` | traverse the focused node list
`right`/`l` | move focus between lists (highlighted in yellow borders)
`left`/`h` | move focus between lists (highlighted in yellow borders)
`tab`/`backtab` | navigate tabs or autocomplete input keyword
`enter` | when traversing in prev/next/search-match node list, goto the selected node
`/[node-id-pattern]` (e.g. `/g1s35t`) | search for node by fuzzy matcher
`r[node-prefix]` (e.g. `r(H: ., D: .)`) | search for node by regex matcher (matched on raw dot file)
`f[node-prefix]` (e.g. `fgraph1_subgraph34`) | apply filter with prefix

All exported files are saved in `exports` directory in the project root.
Most recently exported file is copied in `exports/current.dot`.
