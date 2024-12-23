// @Hustler
//
// Self-Education Only

/// Base address for memory-mapped I/O (MMIO).
const MMIO_BASE: usize = 0xFE000000;
/// Offset for GPIO Function Select 0 register within MMIO.
const GPFSEL0: usize = MMIO_BASE + 0x200000;

/// Converts GPIO alternate function number to corresponding bits.
fn alt2bits(alt: u8) -> u8 {
	match alt {
		0 => 0b100,
		1 => 0b101,
		2 => 0b110,
		3 => 0b111,
		4 => 0b011,
		5 => 0b010,
		_ => 0,
	}
}

/// Selects the alternate function for a GPIO pin.
#[no_mangle]
#[inline(never)]
pub fn gpio_select_function(gpio: u8, alt: u8) {
	let mut gpfsel;
	let field_offset;
	if gpio <= 9 {
		gpfsel = unsafe { *(GPFSEL0 as *const u32) };
		field_offset = (gpio as u32) % 10 * 3;
		gpfsel &= !(1 << field_offset);
		gpfsel &= !(1 << (field_offset + 1));
		gpfsel &= !(1 << (field_offset + 2));
		gpfsel |= (alt2bits(alt) as u32) << field_offset;
		unsafe { (GPFSEL0 as *mut u32).write_volatile(gpfsel) };
	}
}
