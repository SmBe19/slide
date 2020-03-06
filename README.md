# slide
Generate templates for competitive programming

## How to use
Create a new file with `slide problem.cpp init`. Then, in the `!slide config` section, define the input format and generate the template with `slide problem.cpp gen`.

To define the input, specify the variables to read, separated by a space or newlines. By default, the variables are integers. You can use prefixes to change this: use `i_` for integers, `s_` for strings, and `f_` for floating points. To define a pair, use `pab_` where `a` and `b` are the types of the pair. Use `t3abc` to define a tuple with 3 elements. Use `v_` as a prefix for vectors and append the variable with the length at the end (e.g. `v_edges_m`). You can specify the type of the vector by appending it after the initial `v` (e.g. `vs_words_m` for a vector of strings of length `m`).

You can define structs which you want to read. To define a struct, start a line with `}`, followed by the short form (a single letter) of the struct, then the types of the elements, and finally separated by `_` the names of the elements. You can then use the short form as any other type.

A possible configuration might be:

```
}eiii_edge_from_to_weight
n m
vt2is_v_n
ve_edges_m
```

## Install
Run `./install.sh`. This will copy the binary to `~/.local/bin/` and the templates to `~/.local/share/slide/`. There you can adjust your custom templates.

## TODO
 - config parsing
 - generate input reading
 - generate structs
 - parameters for includes
