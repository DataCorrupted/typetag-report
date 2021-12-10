# typetag-0.1.8 report

Typetag 0.1.8 need user to explicitly include `inventory = "0.2"` in [Cargo.toml:12](./Cargo.toml#L12).
Otherwise the compile failes. 
This is a minimal version of the problem.

The complain info is:

```
   Compiling typetag-report v0.1.0 (/home/peter/typetag-report)
error[E0433]: failed to resolve: could not find `inventory` in `{{root}}`
   --> /home/peter/.cargo/registry/src/github.com-1ecc6299db9ec823/inventory-0.2.0/src/lib.rs:393:13
    |
393 |             #[$crate::ctor]
    |             ^^^^^^^^^^^^^^^ could not find `inventory` in `{{root}}`

error[E0433]: failed to resolve: could not find `inventory` in `{{root}}`
   --> /home/peter/.cargo/registry/src/github.com-1ecc6299db9ec823/inventory-0.2.0/src/lib.rs:393:13
    |
393 |             #[$crate::ctor]
    |             ^^^^^^^^^^^^^^^ not found in `inventory::core::cell`
    |
help: consider importing one of these items
    |
1   | use core::cell::UnsafeCell;
    |
1   | use std::cell::UnsafeCell;
    |

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0433`.
error: could not compile `typetag-report`
```


