# slide
Generate templates for competitive programming

## How to use
Create a new file with `slide problem.cpp init`. Then, in the `!slide config` section, define the input format and generate the template with `slide problem.cpp gen`.

To define the input, specify the variables to read, separated by a space or newlines. By default, the variables are integers. You can use suffixes to change this: use `:i` for integers, `:s` for strings, and `:f` for floating points. To define a pair, use `:pab` where `a` and `b` are the types of the pair. Use `:t3abc` to define a tuple with 3 elements. Use `:v` as a suffix for vectors and append the variable with the length at the end. You have to specify the type of the vector by appending it after the initial `v` (e.g. `words:vs:m` for a vector of strings of length `m`).

You can define structs which you want to read. To define a struct, start a line with `}`, followed by the short form (a single letter) of the struct, then the types of the elements, and finally separated by `:` the names of the elements. You can then use the short form as any other type.

Furthermore, there are a few plugins available. You can add them with `+plugin` where `plugin` is the name of the plugin (e.g. `graph` or `dfs`). Using `+flag` and `-flag` you can toggle settings in the plugin and you can set options with `option=value`. You can disable a plugin from reading input using `-input`. To see the available plugins, take a look into the `template/include` folder.

A possible configuration might be:

```
}eiii:edge:from:to:weight
n m
v:vt2is:n
edges:ve:m
t:pfi
vv:vvvpii:n:m:n

+graph +weight var=graph
```

## Install
Run `./install.sh`. This will copy the binary to `~/.local/bin/` and the templates to `~/.local/share/slide/`. There you can adjust your custom templates.

## TODO
Add some more algorithms:
 - BFS
 - DFS
 - TopoSort
 - Dijkstra
 - MST
 - DP Template
 - Binary Search
 - Bridges/Articulation Points
 - SCC
 - Nicer Lazy Segtree
