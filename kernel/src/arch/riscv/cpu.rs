pub unsafe fn set_cpu_id(cpu_id: usize) {
    llvm_asm!("mv gp, $0" : : "r"(cpu_id));
}

pub fn id() -> usize {
    let cpu_id;
    unsafe {
        llvm_asm!("mv $0, gp" : "=r"(cpu_id));
    }
    cpu_id
}

pub fn send_ipi(cpu_id: usize) {
    super::sbi::send_ipi(1 << cpu_id);
}

pub fn halt() {
    unsafe { riscv::asm::wfi() }
}

pub unsafe fn exit_in_qemu(_error_code: u8) -> ! {
    super::sbi::shutdown()
}

pub unsafe fn reboot() -> ! {
    super::sbi::shutdown()
}
