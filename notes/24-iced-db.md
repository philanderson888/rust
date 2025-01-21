# iced db

## contents

- [iced db](#iced-db)
  - [contents](#contents)
  - [intro](#intro)
  - [walkthrough](#walkthrough)

## intro

now we are looking at building basic iced apps with a local database

## walkthrough

```bash
cargo new iced-db
```

```yaml
[dependencies]
iced = { version = "0.10", features = ["tokio"] }
sled = "0.34" # Lightweight embedded database
```