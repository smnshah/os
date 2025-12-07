.PHONY: all clean run kernel limine iso

# Configuration
KERNEL_DIR := kernel
KERNEL_BINARY := $(KERNEL_DIR)/target/x86_64-unknown-none/release/kernel
ISO_DIR := iso_root
ISO_FILE := os.iso

# Default target
all: iso

# Build the kernel
kernel:
	@echo "Building kernel..."
	cd $(KERNEL_DIR) && cargo build --release

# Build limine bootloader
limine:
	@echo "Building Limine..."
	$(MAKE) -C boot/limine

# Create bootable ISO (UEFI only)
iso: kernel limine
	@echo "Creating UEFI bootable ISO..."
	rm -rf $(ISO_DIR)
	
	# Create directory structure
	mkdir -p $(ISO_DIR)/EFI/BOOT
	mkdir -p $(ISO_DIR)/boot/limine
	
	# Copy kernel
	cp $(KERNEL_BINARY) $(ISO_DIR)/boot/kernel
	
	# Copy necessary limine boot files
	cp boot/limine.conf $(ISO_DIR)/boot/limine/limine.conf
	cp boot/limine/bin/BOOTX64.EFI $(ISO_DIR)/EFI/BOOT/
	cp boot/limine/bin/limine-uefi-cd.bin $(ISO_DIR)/boot/
	
	# Create ISO with UEFI support
	xorriso -as mkisofs \
		-R -r -J \
		--efi-boot boot/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		$(ISO_DIR) -o $(ISO_FILE)
	
	@echo "ISO created: $(ISO_FILE)"

# Run in QEMU with UEFI
run: iso
	@echo "Launching QEMU with UEFI..."
	qemu-system-x86_64 \
		-cdrom $(ISO_FILE) \
		-bios /usr/share/ovmf/OVMF.fd \
		-m 256M \
		-serial stdio

# Clean build artifacts
clean:
	@echo "Cleaning..."
	cd $(KERNEL_DIR) && cargo clean
	rm -rf $(ISO_DIR) $(ISO_FILE)