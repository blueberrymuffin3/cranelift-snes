use std::fmt::Display;
use std::string::String;
use std::string::ToString;

use itertools::Itertools;

use crate::Reg;
use crate::isa::w65c816::inst::AModeDirect;
use crate::isa::w65c816::inst::AModeIndexed;
use crate::isa::w65c816::inst::AModeLoad;
use crate::isa::w65c816::inst::AModeMem;
use crate::isa::w65c816::inst::AModeStore;
use crate::isa::w65c816::inst::Inst;
use crate::isa::w65c816::inst::W65C816Reg;
use crate::isa::w65c816::lower::isle::generated_code::AluOp;
use crate::isa::w65c816::lower::isle::generated_code::StackOffset;

fn format_reg(reg: Reg) -> String {
    match reg.to_real_reg() {
        Some(reg) => W65C816Reg::from(reg).to_string(),
        None => format!("{reg:?}"),
    }
}

fn format_reg_pair(reg1: Reg, reg2: Reg) -> String {
    if reg1 == reg2 {
        format_reg(reg1)
    } else {
        format!("{} = {}", format_reg(reg1), format_reg(reg2))
    }
}

impl Display for W65C816Reg {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let string = match self {
            W65C816Reg::A => "A",
            W65C816Reg::X => "X",
            W65C816Reg::Y => "Y",
        };
        f.write_str(string)
    }
}

impl Display for StackOffset {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            StackOffset::Arg { offset } => write!(f, "arg+{offset}"),
            StackOffset::Slot { offset } => write!(f, "slot+{offset}"),
        }
    }
}

impl Display for AModeMem {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AModeMem::Direct { direct } => match direct {
                AModeDirect::Absolute { addr } => write!(f, "${addr:04X}"),
                AModeDirect::DirectPage { addr } => write!(f, "${addr:02X}"),
                AModeDirect::DirectPageIndirect { addr } => write!(f, "$({addr:02X})"),
                AModeDirect::StackOffset { offset } => write!(f, "S,{offset}"),
            },
            AModeMem::Indexed { base, offset } => {
                let offset = format_reg(*offset);
                match base {
                    AModeIndexed::AbsoluteIndexed { addr } => write!(f, "${addr:04X},{offset}"),
                    AModeIndexed::DirectPageIndexed { addr } => write!(f, "${addr:02X},{offset}"),
                    AModeIndexed::DirectPageIndexedIndirect { addr } => {
                        write!(f, "$({addr:02X},{offset})")
                    }
                    AModeIndexed::DirectPageIndirectIndexed { addr } => {
                        write!(f, "$({addr:02X}),{offset}")
                    }
                    AModeIndexed::StackOffsetIndirectIndexed {
                        offset: offset_stack,
                    } => {
                        write!(f, "$(S,{offset}),{offset}")
                    }
                }
            }
        }
    }
}

impl Display for AModeLoad {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AModeLoad::Mem { mem } => mem.fmt(f),
            AModeLoad::Immediate { value } => write!(f, "#${value:04x}"),
            AModeLoad::VReg { vreg } => format_reg(*vreg).fmt(f),
        }
    }
}

impl Display for AModeStore {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AModeStore::Mem { mem } => mem.fmt(f),
            AModeStore::VReg { vreg } => format_reg(vreg.to_reg()).fmt(f),
        }
    }
}

impl Display for AluOp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let string = match self {
            AluOp::Ora => "ora",
            AluOp::And => "and",
            AluOp::Adc => "adc",
            AluOp::Sbc => "sbc",
        };
        f.write_str(string)
    }
}

impl Display for Inst {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut inst = self.clone();
        inst.fixup_stack_vregs();
        match inst {
            Inst::DummyUse { reg } => write!(f, "dummyuse {}", format_reg(reg)),
            Inst::Nop => write!(f, "nop"),
            Inst::Args { args } => write!(
                f,
                "args {}",
                args.iter()
                    .map(|pair| format_reg_pair(pair.preg, pair.vreg.to_reg()))
                    .join(", ")
            ),
            Inst::Rets { rets } => write!(
                f,
                "rets {}",
                rets.iter()
                    .map(|pair| format_reg_pair(pair.preg, pair.vreg))
                    .join(", ")
            ),
            Inst::Move { rd, rs } => {
                write!(f, "mov {}, {}", format_reg(rd.to_reg()), format_reg(rs))
            }
            Inst::Store { mem, reg } => write!(f, "store {}, {}", format_reg(reg), mem),
            Inst::Load { mem, reg } => write!(f, "load {}, {}", format_reg(reg.to_reg()), mem),
            Inst::Alu { op, rd, rs1, rs2 } => {
                write!(
                    f,
                    "{op} {}, {}, {}",
                    format_reg(rd.to_reg()),
                    format_reg(rs1),
                    rs2
                )
            }
            Inst::Bra { target } => write!(f, "bra {target:?}"),
            Inst::Ret => write!(f, "ret"),
        }
    }
}
