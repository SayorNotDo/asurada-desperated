# QEMU 配置

QEMU=SDL_VIDEO_X11_DGAMOUSE=0 qemu-system-$(QEMU_ARCH)
QEMUFLAGS=-d guest_errors -name "Asurada OS $(ARCH)"

ifeq ($(ARCH),aarch64)
	uefi?=yes
	live?=yes
	QEMU_ARCH=aarch64
	QEMU_MACHINE?=virt
	QEMU_CPU=max
	QEMU_SMP?=1
	QEMU_MEM?=2048
	ifeq ($(BOARD),raspi3bp)
			QEMU_KERNEL=$(BUILD)/raspi3bp_uboot.rom
			disk?=sdcard
			QEMU_MACHINE:=raspi3b
			QEMU_MEM:=1024
			net:=usb-net
			audio:=no
			ifneq ($(usb),no)
				QEMUFLAGS+=-usb -device usb-kbd -device usb-tablet
			endif
	else
		ifeq ($(uefi),yes)
				FIRMWARE=/usr/share/AAVMF/AAVMF_CODE.fd
		else
				FIRMWARE=$(BUILD)/qemu_uboot.rom
		endif
		ifneq ($(gpu),no)
				QEMUFLAGS+=-device ramfb
		endif
		ifneq ($(usb),no)
				QEMUFLAGS+=-device qemu-xhci -device usb-kbd -device usb-tablet
		endif
	endif
else
$(error Unsupported ARCH for QEMU "$(ARCH)")
endif

QEMUFLAGS+=-smp $(QEMU_SMP) -m $(QEMU_MEM)
ifneq ($(ARCH),$(HOST_ARCH))
		kvm?=no
endif

ifneq ($(FIRMWARE),)
		QEMUFLAGS+=-bios $(FIRNWARE)
endif

ifneq ($(QEMU_KERNEL),)
		QEMUFLAGS+=-kernel $(QEMU_KERNEL)
endif