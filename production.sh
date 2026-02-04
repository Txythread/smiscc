#!/bin/bash


env | grep CC_ENABLE
cargo build --release 1>/dev/null 2>/dev/null

if [ $? -ne 0 ]; then
	echo "smiscc ./production.sh: failed to compile smiscc" 1>&2
fi


sudo mv target/release/smiscc /usr/local/bin
