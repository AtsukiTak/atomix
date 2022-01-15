#!/bin/bash

cp $1 mnt/EFI/BOOT/BOOTx64.EFI
qemu-system-x86_64 -bios OVMF.fd -drive format=raw,file=fat:rw:mnt
