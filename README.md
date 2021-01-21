# Word count

This program takes an arbitrary textual word content as input (pipe) and generate a word count output like below

It splits words and count each of then from the input content.

It can be run like this

```bash
cat src/main.rs | ./wordstat | head -n 10
```

```bash
16	=
14	let
10	words
9	io
9	word
8	std
7	count
6	stdout
6	&mut
6	fn
```

Build it like this :

```bash
cargo build --release
```
