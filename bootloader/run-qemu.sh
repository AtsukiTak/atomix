#!/bin/bash

# disk.imgが存在しなければ作る
if [ ! -e disk.img ]; then
  qemu-img create -f raw disk.img 200M
  mkfs.fat -n 'ATOMIX' -s 2 -f 2 -R 32 -F 32 disk.img
fi

# mntディレクトリを作成（作成済みの場合はスキップ）
mkdir -p mnt

# disk.imgをsystemにattachしたのち、mntディレクトリにマウントする
hdiutil attach -mountpoint mnt disk.img

# disk.imgのvolume内に/EFI/BOOTディレクトリを作成
mkdir -p mnt/EFI/BOOT

# 対象のファイルを/EFI/BOOT/BOOTx64.EFIにコピー
cp $1 mnt/EFI/BOOT/BOOTx64.EFI

# disk volumeをmntからアンマウントしてdetachする
hdiutil detach mnt

# OVMF_VARS.local.fdが存在しなければOVMF_VARS.fdからコピーする
if [ ! -e OVMF_VARS.local.fd ]; then
  cp OVMF_VARS.fd OVMF_VARS.local.fd
fi

# qemuの実行
ARGS="-drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd -drive if=pflash,format=raw,file=OVMF_VARS.local.fd -drive format=raw,file=disk.img"

if [ "$NOGRAPHIC" = "true" ]
then
  ARGS="-nographic ${ARGS}"
fi

echo $ARGS

qemu-system-x86_64 ${ARGS}
