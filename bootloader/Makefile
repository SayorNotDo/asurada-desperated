TARGET?=aarch64-unknown-uefi
SOURCE:=$(dir $(realpath $(lastword $(MAKEFILE_LIST))))
BUILD:=$(CURDIR)
export RUST_TARGET_PATH?=$(SOURCE)/targets
ifeq ($(shell uname -m),arm64)
MAKEFILE?=mkfile
else
MAKEFILE?=makefile
endif

include $(SOURCE)/mk/$(TARGET).mk

clean:
		rm -rf build target

$(BUILD)/filesystem:
		mkdir -p $(BUILD)
		rm -f $@.partial
		mkdir $@.partial
		$(MAKEFILE) -n 1m  $@.partial/kernel
		mv $@.partial $@

$(BUILD)/filesystem.bin: $(BUILD)/filesystem
		mkdir -p $(BUILD)
		rm -f $@.partial
		$(MAKEFILE) -n 254m $@.partial