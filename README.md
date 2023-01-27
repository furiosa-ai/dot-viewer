# dot-viewer
Dot debugger in TUI

## Prerequisites

### Graphviz

dot-viewer parses a dot format file using C bindings to Graphviz.

Thus, it is required that [Graphviz is installed (compiled)](https://graphviz.org/download/source/) beforehand such that the followings can be included.
```C
#include <graphviz/gvc.h>
#include <graphviz/cgraph.h>
```

### Xdot.py

dot-viewer renders a subgraph with xdot.py, an interactive dot visualizer.

It is required that [xdot is executable in command-line](https://github.com/jrfonseca/xdot.py) beforehand such that the following works.
```console
$ xdot *.dot
```

## Usage

### Initialize

First initialize and update the submodule dot-graph.

```console
$ git submodule init
$ git submodule update
```

### Run

Then run crate.

```console
$ cargo run -- --path [path-to-dot-file]
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
`up` | traverse the focused node list
`down` | traverse the focused node list
`right` | move focus between lists (highlighted in yellow borders)
`left` | move focus between lists (highlighted in yellow borders)
`tab`/`backtab` | navigate tabs
`enter` | when traversing in prev/next/search-match node list, goto the selected node
`/[node-id-pattern]` (e.g. `/g1s35t`) | search for node by fuzzy matcher
`f[node-prefix]` (e.g. `fgraph1_subgraph34`) | apply filter with prefix

All exported files are saved in `exports` directory in the project root.
Most recently exported file is copied in `exports/current.dot`.
