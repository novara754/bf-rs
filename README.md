# bf

Brainfuck interpreter written in Rust.

## Usage

Using cargo:
```
$ cargo run -- sample.bf
```

Using the executable:
```
$ bf sample.bf
```

You can also add the `-c` flag to generate a native binary:
```
$ bf -c sample.bf
```

**Note:** Generating a native binary currently relies on a C compiler called `gcc` to be in
the system path. On Windows you can install GCC using Mingw-w64.

## Todo

* Better error handling.

## License

Licensed under the [MIT License](LICENSE.md).
