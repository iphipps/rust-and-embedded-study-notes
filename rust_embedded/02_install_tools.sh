#!/usr/local/bin/bash

cargo install itm
rustup component add llvm-tools-preview
cargo install cargo-binutils
# from https://github.com/Homebrew/homebrew-cask/pull/56802
brew cask install https://raw.githubusercontent.com/Homebrew/homebrew-cask/b88346667547cc85f8f2cacb3dfe7b754c8afc8a/Casks/gcc-arm-embedded.rb
brew install minicom openocd
