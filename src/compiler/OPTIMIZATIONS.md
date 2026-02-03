# Optimizations

## Fundamentals
Optimizations are assigned by the optimization level, which can be specified using the `--ol` flag. If a user wishes to, they can set or unset specific optimizations by specifying the name of the operation and adding a `!`as a prefix if they wish to turn it off.  
All kinds of optimizations are declared in `src/compiler/optimizations.rs` in the `OptimizationKind` enum. 

## List of Optimizations

| Name                     | Description                                                      | Level |
|:-------------------------|:-----------------------------------------------------------------|:-----:|
| removeIdentityOperations | Removes any operations with the identity number of an operation. |   1   |