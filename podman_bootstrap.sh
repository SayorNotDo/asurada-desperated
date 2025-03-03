#!/usr/bin/env bash

# 配置构建Podman系统
# 安装Podman依赖，Rustup，交叉编译的配方依赖项

set -e

banner()
{
  echo "<***------------------------------------***>"
  echo "<***--- Welcome to Asurada bootstrap ---***>"
  echo "<***----- for building with Podman -----***>"
  echo "<***------------------------------------***>"
}


install_bsd_pkg()
{
  PKG_MANAGER=$1
  PKG_NAME=$2
  BIN_NAME=$3
  if [ -z "$BIN_NAME" ]; then
    BIN_NAME=$PKG_NAME
  fi

  BIN_LOCATION=$(which $BIN_NAME || true)
  if [ -z "$BIN_LOCATION" ]; then
    echo  "$PKG_MANAGER install $PKG_NAME"
    $PKG_MANAGER install "$PKG_NAME"
  else
    echo "$BIN_NAME already exists at $BIN_LOCATION, no need to install $PKG_NAME..."
  fi
}

install_macports_pkg()
{
  install_bsd_pkg "sudo port" "$1" "$2"
}

install_brew_pkg()
{
  install_bsd_pkg "brew" $@
}

install_brew_cask_pkg()
{
  install_bsd_pkg "brew cask" $@
}

osx()
{
  echo "Detected macOS!"

  if [ ! -z "$(which brew)" ]; then
      osx_homebrew $@
  elif [ ! -z "$(which port)" ]; then
      osx_macports $@
  else
      echo "Please install either Homebrew or MacPorts, if you wish to use this script"
      echo "Re-run this script once you installed one of those package managers"
      echo "Will not install, now exiting..."
      exit 1
  fi
}

osx_macports()
{
  echo "MacPorts detected! Now updating..."
  sudo port -v selfupdate

  echo "Installing missing packages..."

  install_macports_pkg "git"
  install_macports_pkg "gmake"
  install_macports_pkg "curl"
  install_macports_pkg "osxfuse"
  install_macports_pkg "podman"

  if [ "$1" == "qemu" ]; then
        install_macports_pkg "qemu" "qemu-system-aarch64"
  elif [ "$1" == "virtualbox" ]; then
        install_macports_pkg "virtualbox"
  else
        echo "Unknown emulator: $1"
        exit 1
  fi
}

osx_homebrew()
{
  echo "Homebrew detected! Now updating..."
  brew update

  echo "Installing missing package..."

  install_brew_pkg "git"
  install_brew_pkg "make"
  install_brew_pkg "curl"
  install_brew_pkg "osxfuse"
  install_brew_pkg "fuse-overlayfs"
  install_brew_pkg "slirp4netns"
  install_brew_pkg "podman"

  if [ "$1" == "qemu" ]; then
        install_brew_pkg "qemu" "qemu-system-aarch64"
  elif [ "$1" == "virtualbox" ]; then
        install_brew_pkg "virtualbox"
  else
        echo "Unknown emulator: $1"
        exit 1
  fi
}

usage()
{
  echo "<***------------------------------------***>"
  echo "<***----- Asurada bootstrap script -----***>"
  echo "Usage: /podman_bootstrap.sh"
  echo "OPTIONS:"
  echo
  echo "  -h,--help             Show this prompt"
  echo "  -u [branch]           Update git repo and update rust"
  echo "                        If blank default to master"
  echo "  -s                    Check the status of the current travis build"
  echo "  -e [emulator]         Install specific emulator, virtualbox or qemu"
  echo "  -p [package manager]  Choose an Ubuntu package manager, apt-fast or aptitude"
  echo "  -d                    Only install the dependencies, skip bootstrap step"
  echo "EXAMPLE:"
  echo
  echo "./podman_bootstrap.sh -e qemu"
  exit
}

cargoInstall()
{
  if [[ "`cargo install --list`" != *"$1 v$2"* ]]; then
          cargo install --force --version "$2" "$1"
  else
          echo "You have $1 version $2 installed already!"
  fi
}

rustInstall()
{
  noninteractive=$1
  if [ -e /usr/local/bin/rustlib/uninstall.sh ]; then
        echo "It appears that multirust is installed on your system."
        echo "This tool has been deprecated by the maintainer, and will cause issues."
        echo "This script can remove multirust from your system if you wish"
        echo "Uninstall multirust (y/N):"
        read multirust
        if echo "$multirust" | grep -iq "^y"; then
                sudo /usr/local/lib/rustlib/uninstall.sh
        else
                echo "Please manually uninstall multirust and any other version of rust, then re-run bootstrap."
                exit 1
        fi
  fi

  # If rustup is not installed we should offer to install if for them
  if [ -z "$(which rustup)" ]; then
    rustup_options="--default-toolchain stable"
        echo "You do not have rustup installed."
        if [ "$noninteractive" = true ]; then
            rustup="y"
            rustup_options+=" -y"
        else
            echo "We HIGHLY recommend using rustup."
            echo "Would you like to install it now?"
            echo "*WARNING* this involves a 'curl | sh' style command"
            printf "(y/N): "
            read rustup
        fi
        if echo "$rustup" | grep -iq "^y"; then
          # install rustup
          curl https://sh.rustup.rs -sSf | sh -s -- $rustup_options
          # You have to add the rustup variables to the $PATH
          echo "export PATH=\"\$HOME/.cargo/bin:\$PATH\"" >> ~/.bashrc
          # source the variables so that we can execute rustup commands in the current shell
          source ~/.cargo/env
        else
          echo "Rustup will not be installed!"
        fi
        if [ -z "$(which rustc)" ];then
                echo "Rust is not installed"
                echo "Please either run the script again, accepting rustup install"
                echo "or install rustc stable manually (not recommended) via:"
                echo "\#curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=stable"
                exit 1
        else
                echo "Your Rust install looks good!"
        fi
  fi
}

#############################################
# Bootstrap 主逻辑
#############################################
boot()
{
  echo "Creating .config with PODMAN_BUILD=1"
  echo "PODMAN_BUILD?=1" > asurada/.config
  echo "The file asurada/.config was created with PODMAN_BUILD=1."
  echo
  echo "** Be sure to update your path to include Rust - run the following command: **"
  echo "source $HOME/.cargo/env"
  echo
  echo "Run the following commands to build asurada using Podman:"
  echo
  echo "cd asurada"
  MAKE="make"
  echo "$MAKE all"
  echo "$MAKE virtualbox or qemu"
  echo
  echo "Finished."
  exit
}

if [ "$1" == "-h" ] || [ "$1" == "--help" ]; then
        usage
elif [ "$1" == "-u" ]; then
        git pull upstream master
        git submodule update --recursive --init
        rustup update nightly
        exit
elif [ "$1" == "-s" ]; then
        statusCheck
        exit
fi

emulator="qemu"
defpackman="apt-get"
dependenciesonly=false
update=false
while getopts ":e:p:udhs" opt
do
    case  "$opt" in
            e) emulator="$OPTARG";;
            p) defpackman="$OPTARG";;
            d) dependenciesonly=true;;
            u) update=true;;
            h) usage;;
            s) statusCheck && exit;;
            \?) echo "I don't know what to do with that option, try -h for help"; exit 1;;
    esac
done

banner

rustInstall "$noninteractive"

if [ "$update" == "true" ]; then
        git pull upstream master
        git submodule update --recursive --init
        exit
fi

if [ "Darwin" == "$(uname -s)" ]; then
        osx "$emulator"
else
        # TODO: Here we will use package managers to determine which operating system the user is using?
        printf "\e[31;1mFatal error: \e[0;31mUnsupported platform, please open an issue\e[0m\n"
fi

if [ "$dependenciesonly" = false ]; then
        boot
fi

echo "Asurada bootstrap complete!"