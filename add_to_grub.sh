#Quick shell script to add my systems grub
rm /boot/quecto_kernel-x86_64.bin
cp build/quecto_kernel-x86_64.bin /boot/
echo "quecto_kernel-x86_64.bin added to GRUB"