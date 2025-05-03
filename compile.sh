#!/bin/bash

YAAL_DIR="$HOME/Documents/pro/YAAL"
YAAL_PLUGIN_DIR="$HOME/Documents/pro/yaal_plugins/test_plugin/"
CONFIG_DIR="$HOME/.config/yaal"

if [ ! -d "$CONFIG_DIR/plugins" ]; then
    mkdir -p $CONFIG_DIR/plugins
fi

cd $YAAL_PLUGIN_DIR
if [ $? -ne 0 ]; then
    echo "Error: Failed to navigate to $YAAL_PLUGIN_DIR"
    exit 1
fi

echo "Building plugin..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "Error: Failed to build the plugin"
    exit 1
fi


rm -f $CONFIG_DIR/plugins/libtest_plugin.so
cp -v "target/release/libtest_plugin.so" "$CONFIG_DIR/plugins/"


echo "Plugin compiled and copied to $CONFIG_DIR/plugins/"

cd $YAAL_DIR
cargo run

