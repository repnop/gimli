use crate::common::Register;

macro_rules! registers {
    ($struct_name:ident, { $($name:ident = ($val:expr, $disp:expr)),+ $(,)? }) => {
        #[allow(missing_docs)]
        impl $struct_name {
            $(
                pub const $name: Register = Register($val);
            )+
        }

        impl $struct_name {
            /// The name of a register, or `None` if the register number is unknown.
            pub fn register_name(register: Register) -> Option<&'static str> {
                match register {
                    $(
                        Self::$name => Some($disp),
                    )+
                    _ => return None,
                }
            }

	    /// Converts a register name into a register number.
	    pub fn name_to_register(value: &str) -> Option<Register> {
		match value {
                    $(
                        $disp => Some(Self::$name),
                    )+
                    _ => return None,
		}
	    }
        }
    };
}

/// ARM architecture specific definitions.
///
/// See [DWARF for the ARM Architecture](http://infocenter.arm.com/help/topic/com.arm.doc.ihi0040b/IHI0040B_aadwarf.pdf).
#[derive(Debug, Clone, Copy)]
pub struct Arm;

// TODO: add more registers.
registers!(Arm, {
    R0 = (0, "R0"),
    R1 = (1, "R1"),
    R2 = (2, "R2"),
    R3 = (3, "R3"),
    R4 = (4, "R4"),
    R5 = (5, "R5"),
    R6 = (6, "R6"),
    R7 = (7, "R7"),
    R8 = (8, "R8"),
    R9 = (9, "R9"),
    R10 = (10, "R10"),
    R11 = (11, "R11"),
    R12 = (12, "R12"),
    R13 = (13, "R13"),
    R14 = (14, "R14"),
    R15 = (15, "R15"),
});

/// Intel i386 architecture specific definitions.
///
/// See Intel386 psABi version 1.1 at the [X86 psABI wiki](https://github.com/hjl-tools/x86-psABI/wiki/X86-psABI).
#[derive(Debug, Clone, Copy)]
pub struct X86;

registers!(X86, {
    EAX = (0, "eax"),
    ECX = (1, "ecx"),
    EDX = (2, "edx"),
    EBX = (3, "ebx"),
    ESP = (4, "esp"),
    EBP = (5, "ebp"),
    ESI = (6, "esi"),
    EDI = (7, "edi"),

    // Return Address register. This is stored in `0(%esp, "")` and is not a physical register.
    RA = (8, "RA"),

    ST0 = (11, "st0"),
    ST1 = (12, "st1"),
    ST2 = (13, "st2"),
    ST3 = (14, "st3"),
    ST4 = (15, "st4"),
    ST5 = (16, "st5"),
    ST6 = (17, "st6"),
    ST7 = (18, "st7"),

    XMM0 = (21, "xmm0"),
    XMM1 = (22, "xmm1"),
    XMM2 = (23, "xmm2"),
    XMM3 = (24, "xmm3"),
    XMM4 = (25, "xmm4"),
    XMM5 = (26, "xmm5"),
    XMM6 = (27, "xmm6"),
    XMM7 = (28, "xmm7"),

    MM0 = (29, "mm0"),
    MM1 = (30, "mm1"),
    MM2 = (31, "mm2"),
    MM3 = (32, "mm3"),
    MM4 = (33, "mm4"),
    MM5 = (34, "mm5"),
    MM6 = (35, "mm6"),
    MM7 = (36, "mm7"),

    MXCSR = (39, "mxcsr"),

    ES = (40, "es"),
    CS = (41, "cs"),
    SS = (42, "ss"),
    DS = (43, "ds"),
    FS = (44, "fs"),
    GS = (45, "gs"),

    TR = (48, "tr"),
    LDTR = (49, "ldtr"),

    FS_BASE = (93, "fs.base"),
    GS_BASE = (94, "gs.base"),
});

/// AMD64 architecture specific definitions.
///
/// See x86-64 psABI version 1.0 at the [X86 psABI wiki](https://github.com/hjl-tools/x86-psABI/wiki/X86-psABI).
#[derive(Debug, Clone, Copy)]
pub struct X86_64;

registers!(X86_64, {
    RAX = (0, "rax"),
    RDX = (1, "rdx"),
    RCX = (2, "rcx"),
    RBX = (3, "rbx"),
    RSI = (4, "rsi"),
    RDI = (5, "rdi"),
    RBP = (6, "rbp"),
    RSP = (7, "rsp"),

    R8 = (8, "r8"),
    R9 = (9, "r9"),
    R10 = (10, "r10"),
    R11 = (11, "r11"),
    R12 = (12, "r12"),
    R13 = (13, "r13"),
    R14 = (14, "r14"),
    R15 = (15, "r15"),

    // Return Address register. This is stored in `0(%rsp, "")` and is not a physical register.
    RA = (16, "RA"),

    XMM0 = (17, "xmm0"),
    XMM1 = (18, "xmm1"),
    XMM2 = (19, "xmm2"),
    XMM3 = (20, "xmm3"),
    XMM4 = (21, "xmm4"),
    XMM5 = (22, "xmm5"),
    XMM6 = (23, "xmm6"),
    XMM7 = (24, "xmm7"),

    XMM8 = (25, "xmm8"),
    XMM9 = (26, "xmm9"),
    XMM10 = (27, "xmm10"),
    XMM11 = (28, "xmm11"),
    XMM12 = (29, "xmm12"),
    XMM13 = (30, "xmm13"),
    XMM14 = (31, "xmm14"),
    XMM15 = (32, "xmm15"),

    ST0 = (33, "st0"),
    ST1 = (34, "st1"),
    ST2 = (35, "st2"),
    ST3 = (36, "st3"),
    ST4 = (37, "st4"),
    ST5 = (38, "st5"),
    ST6 = (39, "st6"),
    ST7 = (40, "st7"),

    MM0 = (41, "mm0"),
    MM1 = (42, "mm1"),
    MM2 = (43, "mm2"),
    MM3 = (44, "mm3"),
    MM4 = (45, "mm4"),
    MM5 = (46, "mm5"),
    MM6 = (47, "mm6"),
    MM7 = (48, "mm7"),

    RFLAGS = (49, "rFLAGS"),
    ES = (50, "es"),
    CS = (51, "cs"),
    SS = (52, "ss"),
    DS = (53, "ds"),
    FS = (54, "fs"),
    GS = (55, "gs"),

    FS_BASE = (58, "fs.base"),
    GS_BASE = (59, "gs.base"),

    TR = (62, "tr"),
    LDTR = (63, "ldtr"),
    MXCSR = (64, "mxcsr"),
    FCW = (65, "fcw"),
    FSW = (66, "fsw"),

    XMM16 = (67, "xmm16"),
    XMM17 = (68, "xmm17"),
    XMM18 = (69, "xmm18"),
    XMM19 = (70, "xmm19"),
    XMM20 = (71, "xmm20"),
    XMM21 = (72, "xmm21"),
    XMM22 = (73, "xmm22"),
    XMM23 = (74, "xmm23"),
    XMM24 = (75, "xmm24"),
    XMM25 = (76, "xmm25"),
    XMM26 = (77, "xmm26"),
    XMM27 = (78, "xmm27"),
    XMM28 = (79, "xmm28"),
    XMM29 = (80, "xmm29"),
    XMM30 = (81, "xmm30"),
    XMM31 = (82, "xmm31"),

    K0 = (118, "k0"),
    K1 = (119, "k1"),
    K2 = (120, "k2"),
    K3 = (121, "k3"),
    K4 = (122, "k4"),
    K5 = (123, "k5"),
    K6 = (124, "k6"),
    K7 = (125, "k7"),
});

/// RV64 architecture specific definitions.
///
/// See [RISC-V ELF psABI specification](https://github.com/riscv/riscv-elf-psabi-doc/blob/master/riscv-elf.md#dwarf-register-numbers)
#[derive(Debug, Clone, Copy)]
pub struct RiscV64;

registers!(RiscV64, {
    // Integer registers (with ABI name)
    X0 = (0, "x0/zero"),
    X1 = (1, "x1/ra"),
    X2 = (2, "x2/sp"),
    X3 = (3, "x3/gp"),
    X4 = (4, "x4/tp"),
    X5 = (5, "x5/t0"),
    X6 = (6, "x6/t1"),
    X7 = (7, "x7/t2"),
    X8 = (8, "x8/s0"),
    X9 = (9, "x9/s1"),
    X10 = (10, "x10/a0"),
    X11 = (11, "x11/a1"),
    X12 = (12, "x12/a2"),
    X13 = (13, "x13/a3"),
    X14 = (14, "x14/a4"),
    X15 = (15, "x15/a5"),
    X16 = (16, "x16/a6"),
    X17 = (17, "x17/a7"),
    X18 = (18, "x18/s2"),
    X19 = (19, "x19/s3"),
    X20 = (20, "x20/s4"),
    X21 = (21, "x21/s5"),
    X22 = (22, "x22/s6"),
    X23 = (23, "x23/s7"),
    X24 = (24, "x24/s8"),
    X25 = (25, "x25/s9"),
    X26 = (26, "x26/s10"),
    X27 = (27, "x27/s11"),
    X28 = (28, "x28/t3"),
    X29 = (29, "x29/t4"),
    X30 = (30, "x30/t5"),
    X31 = (31, "x31/t6"),

    // Floating point registers
    F0 = (32, "f0"),
    F1 = (33, "f1"),
    F2 = (34, "f2"),
    F3 = (35, "f3"),
    F4 = (36, "f4"),
    F5 = (37, "f5"),
    F6 = (38, "f6"),
    F7 = (39, "f7"),
    F8 = (40, "f8"),
    F9 = (41, "f9"),
    F10 = (42, "f10"),
    F11 = (43, "f11"),
    F12 = (44, "f12"),
    F13 = (45, "f13"),
    F14 = (46, "f14"),
    F15 = (47, "f15"),
    F16 = (48, "f16"),
    F17 = (49, "f17"),
    F18 = (50, "f18"),
    F19 = (51, "f19"),
    F20 = (52, "f20"),
    F21 = (53, "f21"),
    F22 = (54, "f22"),
    F23 = (55, "f23"),
    F24 = (56, "f24"),
    F25 = (57, "f25"),
    F26 = (58, "f26"),
    F27 = (59, "f27"),
    F28 = (60, "f28"),
    F29 = (61, "f29"),
    F30 = (62, "f30"),
    F31 = (63, "f31"),

    // Vector registers
    V0 = (96, "v0"),
    V1 = (97, "v1"),
    V2 = (98, "v2"),
    V3 = (99, "v3"),
    V4 = (100, "v4"),
    V5 = (101, "v5"),
    V6 = (102, "v6"),
    V7 = (103, "v7"),
    V8 = (104, "v8"),
    V9 = (105, "v9"),
    V10 = (106, "v10"),
    V11 = (107, "v11"),
    V12 = (108, "v12"),
    V13 = (109, "v13"),
    V14 = (110, "v14"),
    V15 = (111, "v15"),
    V16 = (112, "v16"),
    V17 = (113, "v17"),
    V18 = (114, "v18"),
    V19 = (115, "v19"),
    V20 = (116, "v20"),
    V21 = (117, "v21"),
    V22 = (118, "v22"),
    V23 = (119, "v23"),
    V24 = (120, "v24"),
    V25 = (121, "v25"),
    V26 = (122, "v26"),
    V27 = (123, "v27"),
    V28 = (124, "v28"),
    V29 = (125, "v29"),
    V30 = (126, "v30"),
    V31 = (127, "v31"),

    // CSRs
    USTATUS = (4096, "ustatus"),
    UIE = (4100, "uie"),
    UTVEC = (4101, "utvec"),
    USCRATCH = (4160, "uscratch"),
    UEPC = (4161, "uepc"),
    UCAUSE = (4162, "ucause"),
    UTVAL = (4163, "utval"),
    UIP = (4164, "uip"),
    FFLAGS = (4097, "fflags"),
    FRM = (4098, "frm"),
    FCSR = (4099, "fcsr"),
    CYCLE = (7168, "cycle"),
    TIME = (7169, "time"),
    INSTRET = (7170, "instret"),
    HPMCOUNTER3 = (7171, "hpmcounter3"),
    HPMCOUNTER4 = (7172, "hpmcounter4"),
    HPMCOUNTER5 = (7173, "hpmcounter5"),
    HPMCOUNTER6 = (7174, "hpmcounter6"),
    HPMCOUNTER7 = (7175, "hpmcounter7"),
    HPMCOUNTER8 = (7176, "hpmcounter8"),
    HPMCOUNTER9 = (7177, "hpmcounter9"),
    HPMCOUNTER10 = (7178, "hpmcounter10"),
    HPMCOUNTER11 = (7179, "hpmcounter11"),
    HPMCOUNTER12 = (7180, "hpmcounter12"),
    HPMCOUNTER13 = (7181, "hpmcounter13"),
    HPMCOUNTER14 = (7182, "hpmcounter14"),
    HPMCOUNTER15 = (7183, "hpmcounter15"),
    HPMCOUNTER16 = (7184, "hpmcounter16"),
    HPMCOUNTER17 = (7185, "hpmcounter17"),
    HPMCOUNTER18 = (7186, "hpmcounter18"),
    HPMCOUNTER19 = (7187, "hpmcounter19"),
    HPMCOUNTER20 = (7188, "hpmcounter20"),
    HPMCOUNTER21 = (7189, "hpmcounter21"),
    HPMCOUNTER22 = (7190, "hpmcounter22"),
    HPMCOUNTER23 = (7191, "hpmcounter23"),
    HPMCOUNTER24 = (7192, "hpmcounter24"),
    HPMCOUNTER25 = (7193, "hpmcounter25"),
    HPMCOUNTER26 = (7194, "hpmcounter26"),
    HPMCOUNTER27 = (7195, "hpmcounter27"),
    HPMCOUNTER28 = (7196, "hpmcounter28"),
    HPMCOUNTER29 = (7197, "hpmcounter29"),
    HPMCOUNTER30 = (7198, "hpmcounter30"),
    HPMCOUNTER31 = (7199, "hpmcounter31"),
    SSTATUS = (4352, "sstatus"),
    SEDELEG = (4354, "sedeleg"),
    SIDELEG = (4355, "sideleg"),
    SIE = (4356, "sie"),
    STVEC = (4357, "stvec"),
    SCOUNTEREN = (4358, "scounteren"),
    SSCRATCH = (4416, "sscratch"),
    SEPC = (4417, "sepc"),
    SCAUSE = (4418, "scause"),
    STVAL = (4419, "stval"),
    SIP = (4420, "sip"),
    SATP = (4480, "satp"),
    MVENDORID = (7953, "mvendorid"),
    MARCHID = (7954, "marchid"),
    MIMPID = (7955, "mimpid"),
    MHARTID = (7956, "mhartid"),
    MSTATUS = (4864, "mstatus"),
    MISA = (4865, "misa"),
    MEDELEG = (4866, "medeleg"),
    MIDELEG = (4867, "mideleg"),
    MIE = (4868, "mie"),
    MTVEC = (4869, "mtvec"),
    MCOUNTEREN = (4870, "mcounteren"),
    MSCRATCH = (4928, "mscratch"),
    MEPC = (4929, "mepc"),
    MCAUSE = (4930, "mcause"),
    MTVAL = (4931, "mtval"),
    MIP = (4932, "mip"),
    PMPCFG0 = (5024, "pmpcfg0"),
    PMPCFG2 = (5026, "pmpcfg2"),
    PMPADDR0 = (5040, "pmpaddr0"),
    PMPADDR1 = (5041, "pmpaddr1"),
    PMPADDR2 = (5042, "pmpaddr2"),
    PMPADDR3 = (5043, "pmpaddr3"),
    PMPADDR4 = (5044, "pmpaddr4"),
    PMPADDR5 = (5045, "pmpaddr5"),
    PMPADDR6 = (5046, "pmpaddr6"),
    PMPADDR7 = (5047, "pmpaddr7"),
    PMPADDR8 = (5048, "pmpaddr8"),
    PMPADDR9 = (5049, "pmpaddr9"),
    PMPADDR10 = (5050, "pmpaddr10"),
    PMPADDR11 = (5051, "pmpaddr11"),
    PMPADDR12 = (5052, "pmpaddr12"),
    PMPADDR13 = (5053, "pmpaddr13"),
    PMPADDR14 = (5054, "pmpaddr14"),
    PMPADDR15 = (5055, "pmpaddr15"),
    MCYCLE = (6912, "mcycle"),
    MINSTRET = (6914, "minstret"),
    MHPMCOUNTER3 = (6915, "mhpmcounter3"),
    MHPMCOUNTER4 = (6916, "mhpmcounter4"),
    MHPMCOUNTER5 = (6917, "mhpmcounter5"),
    MHPMCOUNTER6 = (6918, "mhpmcounter6"),
    MHPMCOUNTER7 = (6919, "mhpmcounter7"),
    MHPMCOUNTER8 = (6920, "mhpmcounter8"),
    MHPMCOUNTER9 = (6921, "mhpmcounter9"),
    MHPMCOUNTER10 = (6922, "mhpmcounter10"),
    MHPMCOUNTER11 = (6923, "mhpmcounter11"),
    MHPMCOUNTER12 = (6924, "mhpmcounter12"),
    MHPMCOUNTER13 = (6925, "mhpmcounter13"),
    MHPMCOUNTER14 = (6926, "mhpmcounter14"),
    MHPMCOUNTER15 = (6927, "mhpmcounter15"),
    MHPMCOUNTER16 = (6928, "mhpmcounter16"),
    MHPMCOUNTER17 = (6929, "mhpmcounter17"),
    MHPMCOUNTER18 = (6930, "mhpmcounter18"),
    MHPMCOUNTER19 = (6931, "mhpmcounter19"),
    MHPMCOUNTER20 = (6932, "mhpmcounter20"),
    MHPMCOUNTER21 = (6933, "mhpmcounter21"),
    MHPMCOUNTER22 = (6934, "mhpmcounter22"),
    MHPMCOUNTER23 = (6935, "mhpmcounter23"),
    MHPMCOUNTER24 = (6936, "mhpmcounter24"),
    MHPMCOUNTER25 = (6937, "mhpmcounter25"),
    MHPMCOUNTER26 = (6938, "mhpmcounter26"),
    MHPMCOUNTER27 = (6939, "mhpmcounter27"),
    MHPMCOUNTER28 = (6940, "mhpmcounter28"),
    MHPMCOUNTER29 = (6941, "mhpmcounter29"),
    MHPMCOUNTER30 = (6942, "mhpmcounter30"),
    MHPMCOUNTER31 = (6943, "mhpmcounter31"),
    MCOUNTINHIBIT = (4896, "mcountinhibit"),
    MHPMEVENT3 = (4899, "mhpmevent3"),
    MHPMEVENT4 = (4900, "mhpmevent4"),
    MHPMEVENT5 = (4901, "mhpmevent5"),
    MHPMEVENT6 = (4902, "mhpmevent6"),
    MHPMEVENT7 = (4903, "mhpmevent7"),
    MHPMEVENT8 = (4904, "mhpmevent8"),
    MHPMEVENT9 = (4905, "mhpmevent9"),
    MHPMEVENT10 = (4906, "mhpmevent10"),
    MHPMEVENT11 = (4907, "mhpmevent11"),
    MHPMEVENT12 = (4908, "mhpmevent12"),
    MHPMEVENT13 = (4909, "mhpmevent13"),
    MHPMEVENT14 = (4910, "mhpmevent14"),
    MHPMEVENT15 = (4911, "mhpmevent15"),
    MHPMEVENT16 = (4912, "mhpmevent16"),
    MHPMEVENT17 = (4913, "mhpmevent17"),
    MHPMEVENT18 = (4914, "mhpmevent18"),
    MHPMEVENT19 = (4915, "mhpmevent19"),
    MHPMEVENT20 = (4916, "mhpmevent20"),
    MHPMEVENT21 = (4917, "mhpmevent21"),
    MHPMEVENT22 = (4918, "mhpmevent22"),
    MHPMEVENT23 = (4919, "mhpmevent23"),
    MHPMEVENT24 = (4920, "mhpmevent24"),
    MHPMEVENT25 = (4921, "mhpmevent25"),
    MHPMEVENT26 = (4922, "mhpmevent26"),
    MHPMEVENT27 = (4923, "mhpmevent27"),
    MHPMEVENT28 = (4924, "mhpmevent28"),
    MHPMEVENT29 = (4925, "mhpmevent29"),
    MHPMEVENT30 = (4926, "mhpmevent30"),
    MHPMEVENT31 = (4927, "mhpmevent31"),
    TSELECT = (6048, "tselect"),
    TDATA1 = (6049, "tdata1"),
    TDATA2 = (6050, "tdata2"),
    TDATA3 = (6051, "tdata3"),
    DCSR = (6064, "dcsr"),
    DPC = (6065, "dpc"),
    DSCRATCH0 = (6066, "dscratch0"),
    DSCRATCH1 = (6067, "dscratch1"),
});
