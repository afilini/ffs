# ffs - FakeFS

This is a very, very, basic FUSE fs written in Rust which writes to stdout all the files written to the filesystem.

It was designed to be used in conjunction with Bitcoin Core, because some RPC commands (like `dumptxoutset`) only write the result to
a file, not to stdout. 

By mounting this fs to a directory and asking Core to write the file somewhere within that directory, the content will be printed to
the stdout of this binary.

Run with:

```
cargo run -- mnt/ > out
```

This will mount the fs to `mnt/` and anything written under that directory will be printed out and redirected to `out`.

Naturally more complex pipelines can be constructed, to for example compress streams on the go, etc.
