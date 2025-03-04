# 构建系统环境配置

-include .config

HOST_ARCH?=$(shell uname -m)

# 系统的目标架构
ifeq ($(HOST_ARCH),arm64)
	ARCH?=aarch64
else
	ARCH?=$(HOST_ARCH)
endif
## AArch64架构的设备类型
BOARD?=
## 启动二进制前缀
PREFIX_BINARY?=1
## 启用二进制包
REPO_BINARY?=0
## 镜像名
CONFIG_NAME?=desktop
REPO_NONSTOP?=
ifeq ($(BOARD),)
FILESYSTEM_CONFIG?=config/$(ARCH)/$(CONFIG_NAME).toml
else
FILESYSTEM_CONFIG?=config/$(ARCH)/$(BOARD)/$(CONFIG_NAME).toml
endif
HOST_CARGO=env -u RUSTUP_TOOLCHAIN -u CC -u TARGET cargo
## Flags to pass to asurada-mkfs
ASURADA_MKFS_FLAGS?=
## Set to 1 to enable Podman build, any other value will disable it
PODMAN_BUILD?=1
## The containerfile to user for the Podman base image
CONTAINERFILE?=podman/asurada-base-containerfile

export NPROC=nproc
export ASURADA_MAKE=make
HOST_TARGET := $(shell env -u RUSTUP_TOOLCHAIN rustc -vV | grep host | cut -d: -f2 | tr -d " ")

UNAME:=$(shell uname)
ifeq ($(UNAME),Darwin)
		FUMOUNT=umount
		export NPROC=sysctl -n hw.ncpu
		VB_AUDIO=coreaudio
		VBM=/Applications/VirtualBox.app/Contents/MacOS/VBoxManage
else
	ifneq (, $(shell which fusermount3))
			FUMOUNT=fusermount3 -u
	else
			FUMOUNT=fusermount -u
	endif
	VB_AUDIO=pulse
	VBM=VBoxManage
endif

# Automatic variables
ROOT=$(CURDIR)
export RUST_COMPILER_RT_ROOT=$(ROOT)/rust/src/llvm-project/compiler-rt

## userspace variables
export TARGET=$(ARCH)-unknown-none
ifeq ($(ARCH),riscv64gc)
export GNU_TARGET=riscv64-unknown-none
else
export GUN_TARGET=$(TARGET)
endif

# 构建输出目录
BUILD=build/$(ARCH)/$(CONFIG_NAME)
INSTALLER=installer/target/release/asurada_installer
INSTALLER_OPTS=
LIST_PACKAGES=installer/target/release/list_packages
LIST_PACKAGES_OPTS=


export BOARD


## 交叉编译器变量
AR=$(GNU_TARGET)-gcc-ar
AS=$(GNU_TARGET)-as
CC=$(GNU_TARGET)-gcc
CXX=$(GNU_TARGET)-g++
LD=$(GNU_TARGET)-ld
NM=$(GNU_TARGET)-gcc-nm
OBJCOPY=$(GNU_TARGET)-objcopy
OBJDUMP=$(GNU_TARGET)-objdump
RANLIB=$(GNU_TARGET)-gcc-ranlib
READELF=$(GNU_TARGET)-readelf
STRIP=$(GNU_TARGET)-strip

## Rust 交叉编译变量
export AR_$(subst -,_,$(TARGET)):=$(AR)
export CC_$(subst -,_,$(TARGET)):=$(CC)
export CXX_$(subst -,_,$(TARGET)):=$(CXX)

## If Podman is being used, a container is required
ifeq ($(PODMAN_BUILD),1)
CONTAINER_TAG=build/container.tag
else
CONTAINER_TAG=
endif