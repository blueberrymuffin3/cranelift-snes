use regalloc2::MachineEnv;
use regalloc2::PReg;
use regalloc2::PRegSet;

use crate::RealReg;
use crate::Reg;
use crate::Writable;
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

    pub const fn p_reg(self) -> PReg {
        PReg::new(self.index() + 3, RegClass::Int)
    }

    pub fn fixed_stack_slots() -> PRegSet {
        let mut set = PRegSet::empty();
        for i in 0..W65C816FixedStackSlot::COUNT {
            set.add(W65C816FixedStackSlot::new(i).p_reg());
        }
        set
    }
}

#[derive(Clone, Copy, Debug)]
pub enum W65C816Reg {
    A,
    X,
    Y,
}

impl W65C816Reg {
    pub fn index_regs() -> PRegSet {
        PRegSet::empty()
            .with(W65C816Reg::X.p_reg())
            .with(W65C816Reg::Y.p_reg())
    }

    pub const fn p_reg(self) -> PReg {
        let id = match self {
            Self::A => 0,
            Self::X => 1,
            Self::Y => 2,
        };
        PReg::new(id, RegClass::Int)
    }

    pub fn reg(self) -> Reg {
        self.p_reg().into()
    }
}

impl From<PReg> for W65C816Reg {
    fn from(value: PReg) -> Self {
        let value = value.as_valid().expect("Invalid register");
        match value.hw_enc() {
            0 => Self::A,
            1 => Self::X,
            2 => Self::Y,
            _ => panic!("Unknown register {value}"),
        }
    }
}

impl From<RealReg> for W65C816Reg {
    fn from(value: RealReg) -> Self {
        value.preg().into()
    }
}

impl<T: Into<W65C816Reg>> From<Writable<T>> for W65C816Reg {
    fn from(value: Writable<T>) -> Self {
        value.to_reg().into()
    }
}

pub fn make_machine_env() -> MachineEnv {
    MachineEnv {
        preferred_regs_by_class: [W65C816Reg::index_regs(), PRegSet::empty(), PRegSet::empty()],
        non_preferred_regs_by_class: [PRegSet::empty(); 3],
        scratch_by_class: [None; 3],
        fixed_stack_slots: W65C816FixedStackSlot::fixed_stack_slots()
            .into_iter()
            .collect(),
    }
}
