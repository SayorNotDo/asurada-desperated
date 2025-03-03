SOURCE:=$(dir $(realpath $(lastword $(MAKEFILE_LIST))))
BUILD?=$(CURDIR)
export RUST_TARGET_PATH=$(SOURCE)/targets

ifeq ($(TARGET),)
	ARCH?=$(shell uname -m)
else
	ARCH?=$(shell echo "$(TARGET)" | cut -d - -f1)
endif

ifeq ($(ARCH),riscv64gc)
	ARCH:=riscv64
endif

ifeq ($(ARCH),arm64)
	ARCH:=aarch64
endif

GNU_TARGET=$(ARCH)-unknown-lora

all: $(BUILD)/kernel $(BUILD)/kernel.sym

# 特定架构的链接脚本
LD_SCRIPT=$(SOURCE)/linkers/$(ARCH).ld
# 目标规格文件，指定构建时使用的目标平台
TARGET_SPEC=$(RUST_TARGET_PATH)/$(ARCH)-unknown-none.json


# 编译内核
$(BUILD)/kernel.all: $(LD_SCRIPT) $(TARGET_SPEC) $(shell find $(SOURCE) -name "*.rs" -type f)
	cargo rustc \
		--bin kernel \
		--manifest-path "$(SOURCE)/Cargo.toml" \
		--target "$(TARGET_SPEC)" \
		--release \
		-Z build-std=core,alloc \
		-- \
		-C link-arg=-T -Clink-arg="$(LD_SCRIPT)" \
		-C link-arg=-z -Clink-arg=max-page-size=0x1000 \
		--emit link="$(BUILD)/kernel.all"

# 生成符号文件，通过objcopy从kernel.all文件中提取符号信息，生成调试符号文件kernel.sym
$(BUILD)/kernel.sym: $(BUILD)/kernel.all
	$(GNU_TARGET)-objcopy \
		--only-keep-debug \
		"$(BUILD)/kernel.all" \
		"$(BUILD)/kernel.sym"

# 生成最终内核二进制，通过objcopy去掉调试信息，生成干净的内核镜像kernel
$(BUILD)/kernel: $(BUILD)/kernel.all
	$(GNU_TARGET)-objcopy \
	   --strip-debug \
		"$(BUILD)/kernel.all" \
		"$(BUILD)/kernel"
