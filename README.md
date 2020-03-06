# slide
Generate templates for competitive programming

## How to use
Create a new file with `slide problem.cpp init`. Then, in the `!slide config` section, define the input format and generate the template with `slide problem.cpp gen`.

To define the input, specify the variables to read, separated by a space or newlines. By default, the variables are integers. You can use prefixes to change this: use `i:` for integers, `s:` for strings, and `f:` for floating points. To define a pair, use `pab:` where `a` and `b` are the types of the pair. Use `t3abc` to define a tuple with 3 elements. Use `v:` as a prefix for vectors and append the variable with the length at the end (e.g. `v:edges:m`). You can specify the type of the vector by appending it after the initial `v` (e.g. `vs:words:m` for a vector of strings of length `m`).

You can define structs which you want to read. To define a struct, start a line with `}`, followed by the short form (a single letter) of the struct, then the types of the elements, and finally separated by `:` the names of the elements. You can then use the short form as any other type.

A possible configuration might be:

```
}eiii:edge:from:to:weight
n m
vt2is:v:n
ve:edges:m
pfi:t
vvvpii:vv:n:m:n
```

## Install
Run `./install.sh`. This will copy the binary to `~/.local/bin/` and the templates to `~/.local/share/slide/`. There you can adjust your custom templates.

## TODO
 - parameters for includes
