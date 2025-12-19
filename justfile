set shell := ["bash", "-e", "-c"]

ISO_ROOT := "iso_root"
ISO := "os.iso"
LIMINE := "boot/limine"

limine-configured:
	cd {{LIMINE}} && \
	test -f GNUmakefile || ( \
	  ./bootstrap && \
	  ./configure \
	    --enable-bios \
	    --enable-bios-cd \
	    --enable-uefi-x86-64 \
	    --enable-uefi-cd \
	)

limine: limine-configured
	cd {{LIMINE}} && make

kernel:
	cd kernel && cargo clean && cargo build --release

iso: kernel limine
	rm -rf {{ISO_ROOT}}
	mkdir -p {{ISO_ROOT}}/EFI/BOOT
	mkdir -p {{ISO_ROOT}}/boot

	cp {{LIMINE}}/bin/limine-bios-cd.bin {{ISO_ROOT}}/boot/
	cp {{LIMINE}}/bin/limine-uefi-cd.bin {{ISO_ROOT}}/boot/
	cp {{LIMINE}}/bin/BOOTX64.EFI {{ISO_ROOT}}/EFI/BOOT/
	cp {{LIMINE}}/bin/limine-bios.sys {{ISO_ROOT}}/boot/
	cp boot/limine.conf {{ISO_ROOT}}/boot/

	cp kernel/target/*/release/kernel {{ISO_ROOT}}/boot/kernel

	xorriso -as mkisofs \
	  -b boot/limine-bios-cd.bin \
	  -no-emul-boot -boot-load-size 4 -boot-info-table \
	  --efi-boot boot/limine-uefi-cd.bin \
	  -efi-boot-part --efi-boot-image --protective-msdos-label \
	  {{ISO_ROOT}} \
	  -o {{ISO}}

run: iso
	qemu-system-x86_64 \
	  -cdrom {{ISO}} \
	  -serial stdio

