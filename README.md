# sog-loader

!wip: loader packages for SOG file

## Developing Rust crates

### Environments

- Rust/Cargo 1.94.1
- wasm-bindgen wasm-bindgen 0.2.108
- cargo-release

### Build libraries

```sh
# build core crate
cargo build --package sog-decoder --release
```

### Update and publish rust crates

```sh
# bump the crates versions
# TYPE: rc | minor | major | ...
cargo release version <TYPE> --execute
```

1. bump versions (sync all over the crates)
2. create PR titled and merge it
3. create tag named `crates_vX.X.X` push it
   1. then release workflow will run
4. create GitHub Release titled "crates vX.X.X (short summery, optinally)"

## Developing npm packages

### Environments

- pnpm 10
- Vite 7

### Build libraries

```sh
# build wasm
cargo build --package sog-decoder-wasm --target wasm32-unknown-unknown --release

# exec wasm-bindgen
# for dev build: `wasm:build`
pnpm --filter @sog-loader/core run wasm:build:release

# build individual packages
pnpm --filter @sog-loader/core run build
pnpm --filter @sog-loader/babylonjs run build

# or build all
pnpm -r build
```

## Release npm packages

```sh
# bump versions by @lerna-lite/version
pnpm run version:bump
```

1. bump versions
2. create PR and merge it
3. create tag named `web_vX.X.X` and push it
   1. then release workflow run
4. create GitHub Release titled "web vX.X.X (short summery, optionally)
