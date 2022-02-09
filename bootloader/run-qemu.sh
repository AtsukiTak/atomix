#!/bin/bash

cp $1 mnt/EFI/BOOT/BOOTx64.EFI

ARGS="-bios OVMF.fd"
ARGS="-drive format=raw,file=fat:rw:mnt ${ARGS}"

if [ "$NOGRAPHIC" = "true" ]
then
  ARGS="-nographic ${ARGS}"
fi

echo $ARGS

qemu-system-x86_64 ${ARGS}
