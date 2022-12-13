rm -rf build/
mkdir build/
mkdir build/boot/
mkdir build/kernel/
mkdir -p build/isodir/boot/

as --32 boot/boot.s -o build/boot/boot.o
g++ -m32 -c kernel/kernel.cpp -o build/kernel/kernel.o -std=c++20 -ffreestanding -O2 -Wall -Wextra -fno-exceptions -fno-rtti
gcc -m32 -T linker.ld -o build/isodir/boot/bsos.bin -ffreestanding -O2 -nostdlib build/boot/boot.o build/kernel/kernel.o -lgcc

if grub2-file --is-x86-multiboot build/isodir/boot/bsos.bin; then
  echo multiboot confirmed
else
  echo the file is not multiboot
fi

cp grub.cfg build/isodir/grub.cfg
grub2-mkrescue -o bsos.iso build/isodir/
