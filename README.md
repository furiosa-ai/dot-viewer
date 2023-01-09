# dot-viewer

## Prerequisites

### xdot

[xdot](https://github.com/jrfonseca/xdot.py) should be installed beforehand, such that it can be executed in command-line via

```console
$ xdot *.dot
```

## Usage

### Launch dot-viewer with,

```
$ cargo run -- examples/small.dot
```

### REPL commands

dot-parser traverses a given graph defined in `CenterGraph`.

`CenterGraph` is a subgraph of a given graph such that,
 - it has a `center` node
 - and contains nearby nodes, less than `depth` edges away from the `center`.
 
 Users can traverse and visualize the `CenterGraph` using,

1. `show`

```console
dot-viewer> show
```

Visualize the current `CenterGraph`. Like,

```
dot-viewer> show
(-1) tensor0

/\ prevs /\

subgraph0_operator0

\/ nexts \/

(1) subgraph0_tensor1

center : subgraph0_operator0
depth: 1
```

2. `goto`

```console
dot-viewer> goto subgraph0_tensor27
```

Change the `center` node of the current `CenterGraph`.

3. `depth`

```console
dot-viewer> depth 5
```

Change the `depth` limit of the current `CenterGraph`.

4. `render`

```console
dot-viewer> render
```

Launches xdot, visualizing the current `CenterGraph` in 2D format.

```console
dot-viewer> render all
```

Launches xdot, visualizes the full graph.

5. `export`

```console
dot-viewer> export graph.dot
```

Exports the current `CenterGraph` into dot format in the given filename.
