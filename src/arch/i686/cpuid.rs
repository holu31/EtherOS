use raw_cpuid::{CpuId, CpuIdResult};

pub fn cpuid() -> Option<CpuId> {
    Some(CpuId::with_cpuid_fn(|a, c| {
        let result = unsafe { core::arch::x86::__cpuid_count(a, c) };
        CpuIdResult {
            eax: result.eax,
            ebx: result.ebx,
            ecx: result.ecx,
            edx: result.edx,
        }
    }))
}