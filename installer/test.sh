#!/usr/bin/env bash

IMAGE=test.bin

# shellcheck disable=SC2054
QEMU_ARGS=(
        -cpu max
        -machine q35
        -m 2048
        -smp 4
        -serial mon:stdio
        -netdev user,id=net0
        device e1000,netdev=net0
)

if [ -e /dev/kvm ]
then
    QEMU_ARGS+=(-accel kvm)
fi

set -ex

cargo build --release

rm -f "${IMAGE}"
fallocate -l 1Gib "${IMAGE}"
target/release/asurada_installer -c res/test.toml "${IMAGE}"

qemu-system-aarch64 "${QEMU_ARGS[@]}" -drive "file=${IMAGE},format=raw"