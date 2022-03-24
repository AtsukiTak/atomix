#!/bin/bash

cp $1 mnt/EFI/BOOT/BOOTx64.EFI

# ARGS="-bios OVMF.fd"
SRCDIR="${HOME}/Develop/atsuki-tak/atomix/bootloader"
ARGS="-drive if=pflash,format=raw,readonly=on,file=${SRCDIR}/OVMF_CODE.fd -drive if=pflash,format=raw,file=${SRCDIR}/OVMF_VARS.fd -drive format=raw,file=fat:rw:mnt"

if [ "$NOGRAPHIC" = "true" ]
then
  ARGS="-nographic ${ARGS}"
fi

echo $ARGS

qemu-system-x86_64 ${ARGS}
