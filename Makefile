.PHONY: all kernel limine iso clean

KERNEL_DIR := kernel
LIMINE_DIR := boot/limine
ISO_ROOT   := iso_root
ISO        := os.iso

KERNEL_BIN := $(KERNEL_DIR)/target/*/release/kernel
LIMINE_CONFIG := $(LIMINE_DIR)/config.status

LIMINE_CONFIG_ARGS := \
	--enable-bios \
	--enable-bios-cd \
	--enable-uefi-x86-64 \
	--enable-uefi-cd

all: iso

kernel: $(KERNEL_BIN)

$(KERNEL_BIN):
	cd $(KERNEL_DIR) && cargo build --release

$(LIMINE_CONFIG):
	cd $(LIMINE_DIR) && \
	./bootstrap && \
	./configure $(LIMINE_CONFIG_ARGS)

limine: $(LIMINE_CONFIG)
	$(MAKE) -C $(LIMINE_DIR)

$(ISO): kernel limine boot/limine.conf
	rm -rf $(ISO_ROOT)
	mkdir -p $(ISO_ROOT)/EFI/BOOT
	mkdir -p $(ISO_ROOT)/boot

	cp $(LIMINE_DIR)/bin/limine-bios-cd.bin $(ISO_ROOT)/boot/
	cp $(LIMINE_DIR)/bin/limine-uefi-cd.bin $(ISO_ROOT)/boot/
	cp $(LIMINE_DIR)/bin/BOOTX64.EFI $(ISO_ROOT)/EFI/BOOT/
	cp $(LIMINE_DIR)/bin/limine-bios.sys $(ISO_ROOT)/boot/
	cp boot/limine.conf $(ISO_ROOT)/boot/
	cp $(KERNEL_BIN) $(ISO_ROOT)/boot/kernel

	xorriso -as mkisofs \
		-b boot/limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot boot/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		$(ISO_ROOT) \
		-o $(ISO)

iso: $(ISO)

clean:
	cd $(KERNEL_DIR) && cargo clean
	$(MAKE) -C $(LIMINE_DIR) distclean || true
	rm -rf $(ISO_ROOT) $(ISO)


