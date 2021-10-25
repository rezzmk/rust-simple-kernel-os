cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64-kernel-os/debug/bootimage-kernel-minimal-rust.bin -L "C:\Program Files\qemu"
