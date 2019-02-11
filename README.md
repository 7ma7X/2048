# 2048

<img src="https://user-images.githubusercontent.com/36184621/52550849-43ec4e00-2e1d-11e9-90b0-2384aab813ca.gif" width="600">

## How to play

```
cargo build
./target/debug/rust2048
```

Or simply

```
cargo run
```

|  KEY  |  manipulation  |
| ---- | ---- |
|  i / I |  move up  |
|  m / M |  move down  |
|  j / J |  move left  |
|  k / K |  move right  |

if you want to change this keybind, please modify the conditions in `src/main.rs` listed below...

```rs
  if input.starts_with("i") || input.starts_with("I") {
    // move up

  } else if input.starts_with("m") || input.starts_with("M") {
    // move down

  } else if input.starts_with("j") || input.starts_with("J") {
    // move left

  } else if input.starts_with("k") || input.starts_with("K") {
    // move right
```

## How to test

```
cargo test
```