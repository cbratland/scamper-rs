# scamper-rs

implementation of [scamper](https://github.com/slag-plt/scamper) (scheme) in rust. try it: [https://cbratland.github.io/scamper-rs/](https://cbratland.github.io/scamper-rs/).

made for fun and not up to spec with the original scamper.

## using

there's a repl you can use if you only need to do simple stuff like manipulating numbers:

```bash
cargo run --bin scamper-repl
```

but there's also a clone of the original scamper web interface that compiles the rust code to webassembly, which you can run with:

```bash
cargo install trunk
bun install
bun run serve
```

after installing [bun](https://bun.sh) (better than node).

## checklist

we're still missing some things from the original scamper, but here's the progress:

- [x] language features (`define`, `let`, `match`, etc.)
- [x] prelude
- [x] built-in libraries
	- [x] image
	- [x] lab
	- [x] music
	- [ ] test
	- [ ] audio
	- [ ] canvas
	- [ ] html
- [x] web editor
- [x] multiple files in web editor
- [ ] non-functional stuff
	- [ ] mutable vectors (e.g. `vector-set!`)
	- [ ] music playing and instrument loading (`play-composition`, `load-instrument`, etc.)
- [ ] reactive image files
- [ ] debugging features of the original scamper
