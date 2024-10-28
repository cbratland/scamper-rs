# scamper-rs

implementation of [scamper](https://github.com/slag-plt/scamper) (scheme) in rust. try it: [https://cbratland.github.io/scamper-rs/](https://cbratland.github.io/scamper-rs/).

made for fun and not up to spec with the original scamper.

## using

there's a repl you can use if you only need to do simple stuff like manipulating numbers:

```bash
cargo run --bin scamper-repl
```

but there's also a clone of the original scamper web interface that compiles the rust code to webassembly. you can run it with:

```bash
bun run serve
```

but you might need to set up [leptos](https://leptos.dev) and trunk first (the leptos documentation book walks through this), and [bun](https://bun.sh) (better than node).

## missing stuff

currently only prelude and some stuff from the image library is implemented. some big things that are missing:

- structs
- `section`
- music functionality
- the debugging features of the original scamper
- all the other scamper built in libraries
