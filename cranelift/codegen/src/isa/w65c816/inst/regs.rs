use regalloc2::MachineEnv;
use regalloc2::PReg;
use regalloc2::PRegSet;

use crate::machinst::RegClass;

#[derive(Clone, Copy, Debug)]
pub struct W65C816FixedStackSlot(u8);

impl W65C816FixedStackSlot {
    const COUNT: usize = 8;

    pub fn new(index: usize) -> Self {
        assert!(
            (0..Self::COUNT).contains(&index),
            "Illegal stack reg index {index}"
        );
        Self(index as u8)
    }

    const fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Clone, Copy, Debug)]
pub enum W65C816Reg {
    X,
    Y,
    FixedStack(W65C816FixedStackSlot),
}

impl W65C816Reg {
    pub fn iter_regs() -> impl Iterator<Item = W65C816Reg>  {
        (&[W65C816Reg::X, W65C816Reg::Y]).into_ter()
    }

    pub fn iter_fixed_stack_slots() -> impl Iterator<Item = W65C816Reg> {
        (0..W65C816FixedStackSlot::COUNT)
            .map(W65C816FixedStackSlot::new)
            .map(W65C816Reg::FixedStack)
    }

    const fn hw_enc(self) -> usize {
        match self {
            Self::X => 0,
            Self::Y => 1,
            Self::FixedStack(reg) => reg.index() + 2,
        }
    }

    const fn class(self) -> RegClass {
        RegClass::Int
    }

    pub const fn p_reg(self) -> PReg {
        PReg::new(self.hw_enc(), self.class())
    }
}

pub fn make_machine_env() -> MachineEnv {
    MachineEnv {
        preferred_regs_by_class: [PRegSet::empty().with(), PRegSet::empty(), PRegSet::empty()],
        non_preferred_regs_by_class: [PRegSet::empty(); 3],
        scratch_by_class: [None; 3],
        fixed_stack_slots: W65C816Reg::iter_fixed_stack_slots()
            .map(W65C816Reg::p_reg)
            .collect(),
    }
}
