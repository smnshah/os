.PHONY: all build limine run clean FORCE

KERNEL_DIR := kernel
LIMINE_DIR := boot/limine
ISO_ROOT   := iso_root
ISO        := os.iso

KERNEL_BIN := $(KERNEL_DIR)/target/x86_64-unknown-none/release/kernel
LIMINE_CONFIG_SCR := $(LIMINE_DIR)/configure
LIMINE_MK := $(LIMINE_DIR)/GNUmakefile
LIMINE_EFI := $(LIMINE_DIR)/bin/BOOTX64.EFI
LIMINE_UEFI_CD := $(LIMINE_DIR)/bin/limine-uefi-cd.bin

QEMU       := qemu-system-x86_64
QEMU_SHARE := $(dir $(shell command -v $(QEMU)))../share/qemu
OVMF_CODE  := $(QEMU_SHARE)/edk2-x86_64-code.fd

LIMINE_ARGS := \
	--enable-uefi-x86-64 \
	--enable-uefi-cd

all: build

build: $(ISO)

FORCE:

$(KERNEL_BIN): FORCE
	cd $(KERNEL_DIR) && cargo build --release

limine: $(LIMINE_EFI) $(LIMINE_UEFI_CD)

$(LIMINE_EFI) $(LIMINE_UEFI_CD): $(LIMINE_MK)
	$(MAKE) -C $(LIMINE_DIR)

$(LIMINE_MK): $(LIMINE_CONFIG_SCR)
	cd $(LIMINE_DIR) && ./configure $(LIMINE_ARGS)

$(LIMINE_DIR)/configure:
	cd $(LIMINE_DIR) && ./bootstrap

$(ISO): $(KERNEL_BIN) $(LIMINE_EFI) $(LIMINE_UEFI_CD) boot/limine.conf
	rm -rf $(ISO_ROOT)
	mkdir -p $(ISO_ROOT)/EFI/BOOT
	mkdir -p $(ISO_ROOT)/boot
	cp $(LIMINE_EFI) $(ISO_ROOT)/EFI/BOOT/
	cp $(LIMINE_UEFI_CD) $(ISO_ROOT)/boot/
	cp boot/limine.conf $(ISO_ROOT)/boot/
	cp $(KERNEL_BIN) $(ISO_ROOT)/boot/kernel
	xorriso -as mkisofs \
		-R -r -J \
		--efi-boot boot/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		$(ISO_ROOT) \
		-o $(ISO)

run:
	test -f "$(ISO)" || (echo "OS image not found: $(ISO). Run 'make build' first."; exit 1)
	test -f "$(OVMF_CODE)" || (echo "OVMF firmware not found: $(OVMF_CODE)"; exit 1)
	$(QEMU) -machine q35 \
		-drive if=pflash,format=raw,readonly=on,file=$(OVMF_CODE) \
		-cdrom $(ISO) \
		-serial stdio

clean:
	cd $(KERNEL_DIR) && cargo clean
	$(MAKE) -C $(LIMINE_DIR) distclean || true
	rm -rf $(ISO_ROOT) $(ISO)
