#######################################################
# 操作系统构建
# 通过 include mk/*.mk 语句加载模块化配置文件，涵盖容器构建（podman.mk）、磁盘工具（fstools.mk）、交叉编译（prefix.mk）等
# 环境变量 PODMAN_BUILD 控制是否启用容器化构建，实现构建环境隔离
# 基础目标链：fetch.tag >> repo.tag >> harddrive.img 构成主构建流程，实现从获取依赖到生成磁盘镜像的全链路
# 特殊场景：
# 	live/popsicle：生成liveCD镜像并通过USB烧录工具部署
# 	rebuild：强制全量重建，清除中间产物确保构建纯净性
# 	distclean: 深度清理，移除所有下载的依赖项
# 调试支持：
# 	集成 GDB 远程调试功能，配置符号文件路径 GDB_KERNEL_FILE
#	特殊调试模式 gdb-userspace 支持用户态程序与内核的交互调试
#	网络抓取模块通过 wireshark 目标直接分析 network.pcap 数据包
#######################################################

include mk/config.mk	# 全局配置（编译器路径、构建目录等）
include mk/depends.mk	# 定义构建依赖关系

all: $(BUILD)/harddrive.img			# 默认目标，生成镜像

live:								# 清理旧的挂载点，调用生成Live ISO的规则（livedisk.iso）
	-$(FUMOUNT) $(BUILD)/filesystem/ || true
	-$(FUMOUNT) /tmp/installer/	|| true
	rm -f $(BUILD)/livedisk.iso
	$(MAKE) $(BUILD)/livedisk.iso

popsicle: $(BUILD)/livedisk.iso
		popsicle-gtk $(BUILD)/livedisk.iso

image:
	-$(FUMOUNT) $(BUILD)/filesystem/ || true
	-$(FUMOUNT) /tmp/installer/	|| true
	rm -f $(BUILD)/harddrive.img $(BUILD)/livedisk.iso
	$(MAKE) all

rebuild:
	-$(FUMOUNT) $(BUILD)/filesystem/ || true
	-$(FUMOUNT) /tmp/installer/ || true
	rm -rf $(BUILD)/repo.tag $(BUILD)/harddrive.img $(BUILD)/livedisk.iso
	$(MAKE) all

clean: $(CONTAINER_TAG)
ifeq ($(PODMAN_BUILD),1)
		$(PODMAN_RUN) $(MAKE) $@
else
endif
	-$(FUMOUNT) $(BUILD)/filesystem/ || true
	_$(FUMOUNT) /tmp/installer/	|| true
	rm -rf $(BUILD)

# 交叉编译配置
include mk/prefix.mk

# Podman
include mk/podman.mk

# 硬盘镜像
include mk/disk.mk

# 模拟器配置
include mk/qemu.mk

env: prefx FORCE $(CONTAINER_TAG)
ifeq ($(PODMAN_BUILD),1)
	$(PODMAN_RUN) $(MAKE) $@
else
	export PATH="$(PREFIX_PATH):$$PATH" && \
	bash
endif

# Necessary when debugging for another architecture than the host
export RUST_GDB=gdb-multiarch
GDB_KERNEL_FILE=
gdb: FORCE
		rust-gdb $(GDB_KERNEL_FILE) --eval-command="target remote :1234"

FORCE: