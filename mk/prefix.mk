# Rust/GCC 交叉编译器，relibc，libtool的配置文件

###########################
# 目标平台：AArch64架构的嵌入式应用二进制接口（ELF格式），无操作系统的裸机开发环境
# 硬件架构参数：
#		--with-arch-armv8-a 启用 ARMv8-A指令集支持，覆盖64位基础指令和标准扩展
#		--with-abi=lp64 配置LP64数据模型，即long和指针类型位64位（int保持32位）
###########################
PREFIX=prefix/$(TARGET)

PREFIX_INSTALL=$(PREFIX)/sysroot/
PREFIX_PATH=$(ROOT)/$(PREFIX_INSTALL)/bin

BINUTILS_BRANCH=
GCC_BRANCH=
LIBTOOL_VERSION=

export PREFIX_RUSTFLAGS=-L $(ROOT)/$(PREFIX_INSTALL)/$(TARGET)/lib
export RUSTUP_TOOLCHAIN=$(ROOT)/$(PREFIX_INSTALL)
export ASURADA_TOOLCHAIN=$(RUSTUP_TOOLCHAIN)

export CC=
export CXX=

ifeq ($(TARGET),aarch64-unknown-none)
	GCC_ARCH?=--target=aarch64-none-elf --with-arch=armv8-a --with-abi=lp64
else
	GCC_ARCH?=
endif

prefix: $(PREFIX)/sysroot

PREFIX_STRIP=\
		mkdir -p bin libexec "$(GCC_TARGET)/bin" && \
		find bin libexec "$(GCC_TARGET)/bin" "$(GCC_TARGET)/lib" \
				-type f \
				-exec strip --strip-unneeded {} ';' \
				2> /dev/null

$(PREFIX)/relibc: $(ROOT)/reblic
		mkdir -p "$(@D)"
		rm -rf "@.partial" "$@"
		cp -r "$^" "@.partial"
		touch "$@.partial"
		mv "$@.partial" "$@"

$(PREFIX)/relibc-install: $(PREFIX)/relibc | $(PREFIX)/rust-install $(CONTAINER_TAG)
ifeq ($(PODMAN_BUILD),1)
		$(PODMAN_RUN) $(MAKE) $@
else
		rm -rf "$@.partial" "$@"
		cp -r "$(PREFIX)/rust-install" "$@.partial"
		rm -rf "$@.partial/$(TARGET)/include/"*
		cp -r "$(PREFIX)/rust-install/$(GNU_TARGET)/include/c++" "$@.partial/$(GUN_TARGET)/include/c++"
		cp -r "$(PREFIX)/rust-install/lib/rustlib/$(HOST_TARGET)/lib/" "$@.partial/lib/rustlib/$(HOST_TARGET)/"
		cd "$<" && \
		export PATH="$(ROOT)/$@.partial/bin:$$PATH" && \
		export CARGO="env -u CARGO cargo" && \
		$(MAKE) clean && \
		$(MAKE) -j `$(NPROC)` all && \
		$(MAKE) -j `$(NPROC)` install DESTDIR="$(ROOT)/$@.partial/$(GNU_TARGET)"
		cd "$@.partial" && $(PREFIX_STRIP)
		touch "$@.partial"
		mv "$@.partial" "$@"
endif

$(PREFIX)/relibc-install.tar.gz: $(PREFIX)/relibc-install
		tar \
			--create \
			--gzip \
			--file "$@" \
			--directory="$<" \
			.

$(PREFIX)/libtool:

$(PREFIX)/libtool-build: $(PREFIX)/libtool $(CONTANER_TAG)
ifeq ($(PODMAN_BUILD),1)
		$(PODMAN_RUN) $(MAKE) $@
else
		mkdir -p "$@.partial"
		cd "$<" && \
				./bootstrap \
					--skip-po \
					--force \
					--gnulib-srcdir=./gunlib
		cd "$@.partial" && \
				cp -rp $(abspath $<)/. ./ && \
				"$(ROOT)/$</configure" \
						--target="$(TARGET)" \
						--prefix=$(abspath $(PREFIX)/sysroot) && \
				$(MAKE) -j `$(NPROC)`
		touch "$@.partial"
		mv "$@.partial" "$@"
endif

