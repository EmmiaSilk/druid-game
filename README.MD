# Druid Game

## About
This repository the source for a game about druids, written in Rust.

## Usage
use `git clone` to clone this repository

```
git clone https://github.com/EmmiaSilk/druid-game.git
```

## WebAssembly front-end 
### Building 
```
wasm-pack build wasm-frontend --out-dir 'pkg'
```

### Testing in Headless Browsers
```
wasm-pack test --headless --firefox wasm-frontend
```

### Running
Build the front end, then run

```
cd wasm-frontend/www/
npm install
npm run start 
```