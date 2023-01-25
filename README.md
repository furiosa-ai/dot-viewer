# dot-viewer
Dot debugger in TUI

## Prerequisites

dot-viewer parses a dot format file using C bindings to Graphviz.

Thus, it is required that [Graphviz is installed (compiled)](https://graphviz.org/download/source/) beforehand such that the followings can be included.
```C
#include <graphviz/gvc.h>
#include <graphviz/cgraph.h>
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
`up` | traverse the focused node list
`down` | traverse the focused node list
`right` | move focus between lists (highlighted in yellow borders)
`left` | move focus between lists (highlighted in yellow borders)
`tab`/`backtab` | navigate tabs
`enter` | when traversing in prev/next/search-match node list, goto the selected node
`/[node-id-pattern]` (e.g. `/g1s35t`) | search for node by fuzzy matcher
`f[node-prefix]` (e.g. `fgraph1_subgraph34`) | apply filter with prefix
