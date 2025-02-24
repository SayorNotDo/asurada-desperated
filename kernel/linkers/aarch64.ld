ENTRY(_start) /* 设置程序入口点 */
OUTPUT_FORMAT("elf64-littleaarch64", "elf64-littleaarch64", "elf64-littleaarch64")

KERNEL_OFFSET = 0xFFFFFF0000000000;

SECTIONS {
    /*
     * 内部代码和数据的虚拟地址从KERNEL_OFFSET开始，确保内核代码在虚拟内存高地址运行
     * SIZEOF_HEADERS 跳过 ELF 文件头部买避免头部占用内核实际代码的虚拟地址空间
     */
    . = KERNEL_OFFSET;

    . += SIZEOF_HEADERS;

    /*
     * 通过在零页创建一个虚拟部分，强制零页称为段的一部分
     * 强制引导加载器Limine将最低虚拟地址的段映射到物理地址0xFFFFFFFF80000000（
     * 即使段拥有一个更高的虚拟地址），以解决内核非重定位问题；若零页不属于段的一部分，
     * 内核则会在偏离预期位置的地址上加载，不支持动态重定位的内核会出现崩溃
     */
    .dummy ALIGN(8) : AT(ADDR(.dummy) - KERNEL_OFFSET) {}

    . = ALIGN(4096);

    .text : AT(ADDR(.text) - KERNEL_OFFSET) {
        __text_start = .;   /* 代码段起始 */
        *(.text*)   /* 所有代码段（函数、指令）*/
        __usercopy_start = .;   /* 用户空间拷贝函数起始 */
        *(.usercopy-fns)    /* 用户空间安全拷贝函数（防止越界） */
        __usercopy_end = .;
        . = ALIGN(4096);    /* 4KB对齐（页对齐） */
        __text_end = .;
    }

    /*
     * 只读数据段存放不可修改的全局数据（日志字符串、驱动配置表），对齐页面边界以优化内存保护
     */
    .rodata : AT(ADDR(.rodata) - KERNEL_OFFSET) {
        __rodata_start = .;
        *(.rodata*)
        . = ALIGN(4096);
        __rodata_end = .;
    }

    .data : AT(ADDR(.data) - KERNEL_OFFSET) {
        __data_start = .;
        *(.data*)
        . = ALIGN(4096);
        __data_end = .;
        __bss_start = .;
        *(.bss*)
        . = ALIGN(4096);
        __bss_end = .;
    }

    __end = .; /* 内核结束地址 */

    /DISCARD/ : {
        *(.comment*)
        *(.eh_frame*)
        *(.gcc_except_table*)
        *(.note*)
        .(.rel.eh_frame*)
    }
}
