# Lexodus

# Deps
## Install spin
```bash
brew install brew/spin
```
## Install Leptos
```bash
cargo install --locked leptos
```
## Add WASM target
```bash
rustup target add wasm32-unknown-unknown &&\
rustup target add wasm32-wasi
```

## Run the project
```bash
spin up --build
```

