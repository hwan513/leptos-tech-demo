# Leptos Tech Demo

This tech demo will go over the basics of development using Leptos, a full stack Rust framework.

Being a full stack framework, Leptos enables two ways to develop web applications:

1. Client-side rendering (CSR): This method is most similar to React which has been introduced in the course CS732/SOFTENG750.
2. Full-stack, server-side rendering (SSR): This method is most similar to Next.js, a popular full stack framework built on React.

Due to the complexity and depth of developing a website in Rust, and for comparison with React, only CSR will be explored in this demo.

# Getting Started

## Rust Toolchain

This section will go over how to install all the tooling related to Leptos, assuming zero prior knowledge.

### Code Editors

There are two common editors to use for Rust:

- [Visual Studio Code](https://code.visualstudio.com) with the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension. If using VS Code, I recommended also enabling Clippy to run within editor for better linting suggestions (see [here](https://code.visualstudio.com/docs/languages/rust#_linting).
- [Rust Rover](https://www.jetbrains.com/rust/). This is JetBrains IDE for Rust.

### Rustup, Cargo and Rust

To get the project running you will need to install the Rust toolchain. The recommended way is to install [`rustup`](https://rustup.rs), which functions as a version manager for the toolchain, similar to the [Node Version Manager](https://github.com/nvm-sh/nvm) (nvm). Follow the instructions [here](https://rustup.rs) or see [other installation methods](https://rust-lang.github.io/rustup/installation/other.html#using-a-package-manager) to install rustup.

Once `rustup` is installed, you will want to use `nightly` Rust to use all the features and syntax shown in this demo. When creating Leptos project you might have to manually set the toolchain to nightly instead of stable, but for this demo, it is handled with the [`rust-toolchain.toml`](./rust-toolchain.toml) file\dots

!> [!NOTE]

> If rustup doesn't work then you might need to restart your shell instance

```sh
rustup toolchain install nightly

# manually set nightly as default globally
rustup default nightly

# or only for your project
cd <into your project>
rustup override set nightly
```

Some of the tools installed are as follows with their equivalents in the JavaScript Ecosystem:

- [Cargo](https://github.com/rust-lang/cargo): Rust package manager, similar to npm/yarn/pnpm.
- [Clippy](https://github.com/rust-lang/rust-clippy): Rust linter, similar to ESLint.
- [rustfmt](https://github.com/rust-lang/rustfmt): Rust formatter, similar to Prettier.

### Trunk and WebAssembly

Trunk is a WASM web application bundler for Rust, working for frameworks such as [Leptos](https://leptos.dev), [Sycamore](https://sycamore-rs.netlify.app) and [Yew](https://yew.rs). It functions similar to how Vite does for the common JavaScript frontend frameworks, being able to create a dev server and allowing for the bundling of the application with a build command. In addition to Trunk, the `wasm32-unknown-unknown` target must also be added to allow for compilation of Rust code into WebAssembly.

To install Trunk and WebAssembly target run:

```sh
cargo install trunk
# wasm target doesn't need to be defined in this project because of rust-toolchain.toml
rustup target add wasm32-unknown-unknown
```

Unfortunately, Trunk falls short of Vite on the dev server side, due to the nature of Rust compilation into WebAssembly. The dev server in Trunk is much slower compared to Vite and it does not support Hot Module Reloading, meaning the page will refresh after a change to the code and that state will not persist.

## Running this demo

### Running for development

To open the app in your default browser at `http://localhost:3000`, make sure that you are in the correct directory and run:

```sh
trunk serve --port 3000 --open
```

### Building for deployment

To build the app for deployment into the `dist` folder, run:

```sh
trunk build --release
```

# Concepts Covered
