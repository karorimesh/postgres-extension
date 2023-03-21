#!/bin/bash
apt-get -y update
apt-get -y install curl
apt-get -y install build-essential
apt-get -y install libssl-dev
apt-get -y install pkg-config
apt-get -y install libreadline-dev
apt-get -y install zlib1g-dev
apt-get -y install libclang-dev
apt-get -y install git
apt-get -y install protobuf-compiler libprotobuf-dev

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
(echo; echo 'eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"') >> /root/.profile
eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"

brew install gcc
brew install bufbuild/buf/buf

cargo install --locked cargo-pgx
cargo install cargo-make
cargo pgx init
cargo make build
cargo pgx package