ASM = asm/start.o asm/gdt_flush.o asm/idt_load.o asm/irq.o asm/isr.o asm/paging.o

TARGET = i686
KERNEL = kernel.elf

LD = ld.lld

RUST = target/$(TARGET)/release/libswan_os.a

all: $(KERNEL)

setup:
	@rustup override set nightly
	@rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
	@rustup target add x86_64-unknown-none

$(KERNEL): Cargo.toml src/*.rs $(ASM)
	@cargo rustc --release --target targets/$(TARGET).json

	$(LD) -n $(RUST) $(ASM) \
		-T src/link.ld \
		-o $(KERNEL)

$(ASM): asm/%.o : asm/%.s
	@$(AS) $< --32 -o $@

iso: $(KERNEL)
	-mkdir -p isodir/boot/grub
	mv $(KERNEL) isodir/boot/
	cp grub.cfg isodir/boot/grub

	grub-mkrescue isodir/ -o swan-os.iso

run:
	qemu-system-i386 -name "SwanOS" -m 125M -cpu max -serial mon:stdio -cdrom swan-os.iso

debug:
	qemu-system-i386 -name "SwanOS" -m 125M -cpu max -serial file:Qemu.log -cdrom swan-os.iso -s -S &
	gdb -ex "target remote localhost:1234"

runiso: iso
	@make run

clean:
	-rm $(ASM) Qemu.log swan-os.iso Cargo.lock
	-rm -rf isodir target
