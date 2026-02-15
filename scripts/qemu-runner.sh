#!/bin/bash
# scripts/qemu-runner.sh

# cargo passes the output path as the first argument
make qemu-for-test KERNEL_ELF="$1"