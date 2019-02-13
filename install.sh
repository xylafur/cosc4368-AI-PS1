if [ -f /etc/fedora-release ]; then
    OS="FEDORA"
elif [ -f /etc/arch-release ]; then
    OS="ARCH"
else
    echo "Don't know how to install the dependencies for this distro / OS"
    exit
fi

case $OS in
FEDORA)
    sudo dnf install rust cargo
    ;;
ARCH)
    sudo pacman -S rust
    ;;
esac

