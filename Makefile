TARGET = x86_64-unknown-uefi

#clean:
#	@rm target
#	@rm img
build:
	@cargo build --target $(TARGET)

test: build
	@mkdir img/EFI/BOOT
	@cp ./target/$(TARGET)/debug/ruefis.efi ./img/EFI/BOOT/BOOTX64.EFI
	@qemu-system-x86_64 -bios OVMF.fd -fda fat:floppy:/img