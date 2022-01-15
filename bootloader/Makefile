.PHONY: run
run: image/EFI/BOOT/BOOTx64.EFI
	qemu-system-x86_64 -bios OVMF.fd -drive format=raw,file=fat:rw:image
