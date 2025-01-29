# iced

## contents

- [iced](#iced)
  - [contents](#contents)
  - [reference](#reference)
  - [docs](#docs)
  - [examples](#examples)
  - [adding canvas to toml](#adding-canvas-to-toml)

## reference

examples

https://github.com/iced-rs/iced/tree/master/examples#examples

## docs

pocket guide https://docs.rs/iced/latest/iced/#the-pocket-guide

unnoficial iced guide https://jl710.github.io/iced-guide/app_structure/app_structure.html

online community https://discourse.iced.rs

networking and async with tokio https://docs.rs/tokio/latest/tokio/index.html#cpu-bound-tasks-and-blocking-code 

## examples

chat app https://github.com/airstrike/iced_openai 

receipts https://github.com/airstrike/iced_receipts 

download progress https://github.com/iced-rs/iced/tree/0.13/examples/download_progress 

api calls https://jl710.github.io/iced-guide/runtime/custom_task/custom_task.html 

modal popup window https://github.com/iced-rs/iced/blob/0.13/examples/modal/src/main.rs 

sample joke app https://jl710.github.io/iced-guide/app_structure/app_structure.html 

styling https://github.com/iced-rs/iced/blob/0.13/examples/styling/src/main.rs 

## adding canvas to toml

notice that this command is required in order to add canvas to the projectx

```rs
cargo add iced --features canvas
```


