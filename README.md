# secret-leptos

This is a Client-Side-Rendered App showing how it's possible to interact with
[Keplr](https://github.com/chainapsis/keplr-wallet) and
[secretjs](https://github.com/scrtlabs/secret.js) from WebAssembly,
using the [Leptos](https://leptos.dev/) framework.

## Getting Started

If you don’t already have it installed, you can install Trunk by running

```bash
cargo install trunk
```

Using nightly Rust and the nightly feature in Leptos enables the function-call
syntax for signal getters and setters used in this example.

To use nightly Rust, you can run

```bash
rustup toolchain install nightly
rustup default nightly
```

Make sure you've added the `wasm32-unknown-unknown` target so that Rust can
compile your code to WebAssembly to run in the browser.

```bash
rustup target add wasm32-unknown-unknown
```

## Tailwind

`trunk.toml` is configured to build the CSS automatically.

You can install Tailwind using `npm`:

```bash
npm install -D tailwindcss
```

If you'd rather not use npm, you can install the Tailwind binary
[here](https://github.com/tailwindlabs/tailwindcss/releases).

## Developing

Start a development server:

```bash
trunk serve

# or start the server and open the app in a new browser tab
trunk serve --open
```

## Building

To create a production version of your app:

```bash
trunk build --release
```

`trunk build` will create a number of build artifacts in a `dist/` directory.
Publishing `dist` somewhere online should be all you need to deploy your app.
This should work very similarly to deploying any JavaScript application.
