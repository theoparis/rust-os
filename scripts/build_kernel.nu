#!/usr/bin/env nu

RUSTC_BOOTSTRAP=1 cargo rustc -p kernel --target=x86_64-unknown-none -- -Clink-args=-Tkernel/x86_64-qemu.ld -Clink-args="-no-pie"
(
    qemu-system-x86_64
    -machine q35
    -serial mon:stdio
    -kernel ./target/x86_64-unknown-none/debug/kernel
    -device virtio-gpu-pci
    -vga none
    -device virtio-net-pci,netdev=net0
    -netdev user,id=net0,hostfwd=tcp::5555-:5555
)
