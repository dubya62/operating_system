use core::panic::PanicInfo;

#[macro_use]
mod serial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    // SAFETY: This is the correct port and the correct code
    unsafe {
        x86::io::outl(0xf4, exit_code as u32);
    }
}

pub trait TestFn {
    fn run(&self) -> ();
}

impl<F> TestFn for F
where
    F: Fn() -> (),
{
    fn run(&self) -> () {
        serial_print!("test {} ...", core::any::type_name::<F>());
        self();
        serial_println!("ok");
    }
}

pub fn test_runner(tests: &[&dyn TestFn]) {
    serial_println!("Running {} tests\n", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("FAILED\n");
    serial_println!("Error: {}", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[test_case]
fn test_tests() {
    assert_eq!(1, 1);
}
