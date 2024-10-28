# Krust

## Installation de la version adéquate de rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustup install nightly

rustup default nightly

## Installation de la cible
rustup target add thumbv7em-none-eabihf

## Simple cargo build
cargo build --target thumbv7em-none-eabihf
