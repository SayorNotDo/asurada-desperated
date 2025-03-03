#!/usr/bin/env bash

#######################################
#
# 构建脚本
#
#######################################

usage()
{
  echo "build.sh:         Invoke make for a particular architecture and configuration"
  echo "Usage:"
  echo "./build.sh [-X | -A | -6 | -a ARCH] [-c CONFIG] [-f FILESYSTEM_CONFIG] TARGET..."
  echo "    -A            Equivalent to -a aarch64."
  echo "    -a ARCH:      Processor Architecture."
  echo "    -c CONFIG:    Name of config, e.g. desktop, server or demo."
  echo "    NOTE:         If you do not change ARCH or CONFIG very often, edit mk/config.mk"
  echo "                  and set ARCH and FILESYSTEM_CONFIG. You only need to use this"
  echo "                  script when you want to override them."
}

if [ "$1" == "-h" ] || [ "$1" == "--help" ]; then
    usage
    exit
fi

defaultarch="aarch64"
defaultname="demo"
ARCH=""
CONFIG_NAME=""
FILESYSTEM_CONFIG=""

while getopts ":c:f:a:hA" opt
do
    case "$opt" in
            a) ARCH="$OPTARG";;
            c) CONFIG_NAME="$OPTARG";;
            f) FILESYSTEM_CONFIG="$OPTARG";;
            A) ARCH="aarch64";;
            h) usage;;
            \?) echo "Unknown option -$OPTARG, try -h for help"; exit ;;
            :) echo "-$OPTARG requires a value"; exit;;
    esac
done
shift $((OPTIND -1)) # 清理已处理的命令行选项，后续非选项参数移动至位置参数的起始位置

if [ -z "$ARCH" ] && [ -n "$FILESYSTEM_CONFIG" ]; then
    dirname=`dirname "$FILESYSTEM_CONFIG"`
    ARCH=`basename $dirname`
    case "$ARCH" in
      aarch64) : ;;
      \?) ARCH=""; echo "Unknown Architecture, please specify aarch64";;
    esac
fi

if [ -z "$config_name" ] && [ -n "$FILESYSTEM_CONFIG" ]; then
    CONFIG_NAME=`basename "$FILESYSTEM_CONFIG" .toml`
fi

if [ -z "$ARCH" ]; then
    ARCH="$defaultarch"
fi

if [ -z "$CONFIG_NAME" ]; then
    CONFIG_NAME="$defaultname"
fi

if [ -z "$FILESYSTEM_CONFIG" ]; then
    FILESYSTEM_CONFIG="config/$ARCH/$CONFIG_NAME.toml"
fi

export ARCH CONFIG_NAME FILESYSTEM_CONFIG
make $@