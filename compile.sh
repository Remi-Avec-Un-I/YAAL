#!/bin/bash

YAAL_DIR=$(pwd)
YAAL_PLUGIN_DIR="$HOME/Documents/pro/yaal_plugins"
YAAL_CONFIG_DIR="$HOME/.config/yaal"

for plugin in $YAAL_PLUGIN_DIR/*; do
    if [[ $@ == *"$(basename $plugin)"* ]]; then
        echo "Skipping $plugin"
        continue
    fi
    cd $plugin
    echo $plugin
    if [ -f Cargo.toml ]; then
        cargo build --release
        path=$(realpath $(find target/release -type f -name "*.so" | head -n 1))
    fi
    cp $path $YAAL_CONFIG_DIR/plugins/
done

cd $YAAL_DIR
cargo run
