export PARTED?=parted
export QEMU?=qemu-system-aarch64

all:

$(BUILD)/bootloader.efi: $(SOURCE)/Cargo.toml $(SOURCE)/Cargo.lock $(shell find $(SOURCE)/src -type f)
		mkdir -p "$(BUILD)"
		cargo rustc \
				--manifest-path="$<"

$(BUILD)/esp.bin: $(BUILD)/bootloader.efi
		rm -f "$@.partial"
		$(MAKEFILE)  -n 64m "$@.partial"

$(BUILD)/harddrive.bin: $(BUILD)/esp.bin $(BUILD)/filesystem.bin
		rm -f "$@.partial"
		$(MAKEFILE)  -n 320m "$@.partial"

$(BUILD)/firmware.rom: /opt/homebrew/share/qemu/edk2-aarch64-code.fd
		cp "$<" "$@"

qemu: $(BUILD)/harddrive.bin $(BUILD)/firmware.rom
		$(QEMU) \
				-d cpu_reset \
				-no-reboot \
				-smp 4 -m 2048 \
				-chardev stdio,id=debug,signal=off,mux=on \
				-serial chardev:debug \
				-device virtio-gpu-pci \
				-machine virt \
				-net none \
				-cpu max \
				-bios "$(BUILD)/firmware.rom" \

