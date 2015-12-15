# broadcast

[![Build Status](https://travis-ci.org/softprops/broadcast.svg?branch=master)](https://travis-ci.org/softprops/broadcast) [![Coverage Status](https://coveralls.io/repos/softprops/broadcast/badge.svg?branch=master&service=github)](https://coveralls.io/github/softprops/broadcast?branch=master)

A rustlang adapter for writing to multiple sources, adapted from the standard library's `std::io::Write#broadcast` which has since been deprecated.

## api docs

rustdoc api documentation can be found [here](https://softprops.github.io/broadcast)

## examples

The currently unstable/deprecated std library function looks like this

```rust
let broadcaster = writera.broadcast(writerb);
```

In broadcast this looks like

```rust
let broadcaster = broadcast::BroadcastWriter::new(
  writera, writerb
);
```

Doug Tangren (softprops) 2015
