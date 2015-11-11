rust-touptek
============

rust-touptek is a binding for the [ToupLite][] library, which allows to grab images from CMOS instrumentation cameras such as those used in AmScope microscopes.

[touplite]: http://www.touptek.com/download/showdownload.php?lang=en&id=2

Prerequisites
-------------

The ToupLite SDK, installable as a component of ToupLite for [Linux][touplitelinux] or [OS X][toupliteosx].

[touplitelinux]: http://www.touptek.com/download/showdownload.php?lang=en&id=28
[toupliteosx]: http://www.touptek.com/download/showdownload.php?lang=en&id=29

Usage
-----

See the [capture_png.rs example](examples/capture_png.rs). You can run
it from a source tree using `cargo run --example capture_png`.

Note that the example code requires the following dependencies
to be specified in `Cargo.toml`:

```
[dependencies]
png  = { git = "https://github.com/servo/rust-png" }
simd = { git = "https://github.com/huonw/simd" }
```

License
-------

[MIT license](LICENSE.txt)
