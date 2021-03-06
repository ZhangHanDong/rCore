use crate::memory::phys_to_virt;
use apic::{LocalApic, XApic};
use raw_cpuid::CpuId;
use x86_64::registers::control::{Cr0, Cr0Flags, Cr4, Cr4Flags};

/// Exit qemu
/// See: https://wiki.osdev.org/Shutdown
/// Must run qemu with `-device isa-debug-exit`
/// The error code is `value written to 0x501` *2 +1, so it should be odd when non-zero
pub unsafe fn exit_in_qemu(error_code: u8) -> ! {
    use x86_64::instructions::port::Port;
    if error_code == 0 {
        Port::new(0xB004).write(0x2000 as u16);
    } else {
        assert_eq!(error_code & 1, 1, "error code should be odd");
        Port::new(0x501).write((error_code - 1) / 2);
    }
    unreachable!()
}

pub unsafe fn reboot() -> ! {
    use x86_64::instructions::port::Port;
    Port::new(0x64).write(0xfeu8);
    unreachable!()
}

pub fn id() -> usize {
    CpuId::new()
        .get_feature_info()
        .unwrap()
        .initial_local_apic_id() as usize
}

pub fn send_ipi(cpu_id: usize) {
    let mut lapic = unsafe { XApic::new(phys_to_virt(0xfee00000)) };
    lapic.send_ipi(cpu_id as u8, 0x30); // TODO: Find a IPI trap num
}

pub fn init() {
    let mut lapic = unsafe { XApic::new(phys_to_virt(0xfee00000)) };
    lapic.cpu_init();

    // enable FPU, the manual Volume 3 Chapter 13
    unsafe {
        Cr4::update(|cr4| {
            cr4.insert(Cr4Flags::OSFXSR);
            cr4.insert(Cr4Flags::OSXMMEXCPT_ENABLE);
        });
        Cr0::update(|cr0| {
            cr0.remove(Cr0Flags::EMULATE_COPROCESSOR);
            cr0.insert(Cr0Flags::MONITOR_COPROCESSOR);
        });
    }
}

pub fn halt() {
    use x86_64::instructions::hlt;
    hlt();
}
