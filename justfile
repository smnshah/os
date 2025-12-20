build:
	make all

run:
	qemu-system-x86_64 \
	  -cdrom os.iso \
	  -serial stdio

clean:
	make clean

