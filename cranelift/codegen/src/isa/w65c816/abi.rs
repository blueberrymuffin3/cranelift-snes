use crate::isa::w65c816::inst::Inst;
use crate::isa::w65c816::settings::Flags as W65C816Flags;
use crate::machinst::ABIMachineSpec;
use crate::machinst::Callee;
use crate::machinst::IsaFlags;

/// Support for the W65C816 ABI from the callee side (within a function body).
pub(crate) type W65C816Callee = Callee<W65C816MachineDeps>;

/// W65C816-specific ABI behavior. This struct just serves as an implementation
/// point for the trait; it is never actually instantiated.
pub struct W65C816MachineDeps;

impl IsaFlags for W65C816Flags {}

impl ABIMachineSpec for W65C816MachineDeps {
    type I = Inst;

    type F = W65C816Flags;

    const STACK_ARG_RET_SIZE_LIMIT: u32 = 256;

    fn word_bits() -> u32 {
        todo!()
    }

    fn stack_align(call_conv: crate::isa::CallConv) -> u32 {
        1
    }

    fn compute_arg_locs(
        call_conv: crate::isa::CallConv,
        flags: &crate::settings::Flags,
        params: &[crate::ir::AbiParam],
        args_or_rets: crate::machinst::ArgsOrRets,
        add_ret_area_ptr: bool,
        args: crate::machinst::ArgsAccumulator,
    ) -> crate::CodegenResult<(u32, Option<usize>)> {
        todo!()
    }

    fn gen_load_stack(
        mem: crate::machinst::StackAMode,
        into_reg: crate::Writable<crate::Reg>,
        ty: crate::ir::Type,
    ) -> Self::I {
        todo!()
    }

    fn gen_store_stack(
        mem: crate::machinst::StackAMode,
        from_reg: crate::Reg,
        ty: crate::ir::Type,
    ) -> Self::I {
        todo!()
    }

    fn gen_move(
        to_reg: crate::Writable<crate::Reg>,
        from_reg: crate::Reg,
        ty: crate::ir::Type,
    ) -> Self::I {
        todo!()
    }

    fn gen_extend(
        to_reg: crate::Writable<crate::Reg>,
        from_reg: crate::Reg,
        is_signed: bool,
        from_bits: u8,
        to_bits: u8,
    ) -> Self::I {
        todo!()
    }

    fn gen_args(args: std::prelude::v1::Vec<crate::machinst::ArgPair>) -> Self::I {
        todo!()
    }

    fn gen_rets(rets: std::prelude::v1::Vec<crate::machinst::RetPair>) -> Self::I {
        todo!()
    }

    fn gen_add_imm(
        call_conv: crate::isa::CallConv,
        into_reg: crate::Writable<crate::Reg>,
        from_reg: crate::Reg,
        imm: u32,
    ) -> crate::machinst::SmallInstVec<Self::I> {
        todo!()
    }

    fn gen_stack_lower_bound_trap(limit_reg: crate::Reg) -> crate::machinst::SmallInstVec<Self::I> {
        todo!()
    }

    fn gen_get_stack_addr(
        mem: crate::machinst::StackAMode,
        into_reg: crate::Writable<crate::Reg>,
    ) -> Self::I {
        todo!()
    }

    fn get_stacklimit_reg(call_conv: crate::isa::CallConv) -> crate::Reg {
        todo!()
    }

    fn gen_load_base_offset(
        into_reg: crate::Writable<crate::Reg>,
        base: crate::Reg,
        offset: i32,
        ty: crate::ir::Type,
    ) -> Self::I {
        todo!()
    }

    fn gen_store_base_offset(
        base: crate::Reg,
        offset: i32,
        from_reg: crate::Reg,
        ty: crate::ir::Type,
    ) -> Self::I {
        todo!()
    }

    fn gen_sp_reg_adjust(amount: i32) -> crate::machinst::SmallInstVec<Self::I> {
        todo!()
    }

    fn compute_frame_layout(
        call_conv: crate::isa::CallConv,
        flags: &crate::settings::Flags,
        sig: &crate::ir::Signature,
        regs: &[crate::Writable<crate::RealReg>],
        function_calls: crate::machinst::FunctionCalls,
        incoming_args_size: u32,
        tail_args_size: u32,
        stackslots_size: u32,
        fixed_frame_storage_size: u32,
        outgoing_args_size: u32,
    ) -> crate::FrameLayout {
        todo!()
    }

    fn gen_prologue_frame_setup(
        call_conv: crate::isa::CallConv,
        flags: &crate::settings::Flags,
        isa_flags: &Self::F,
        frame_layout: &crate::FrameLayout,
    ) -> crate::machinst::SmallInstVec<Self::I> {
        todo!()
    }

    fn gen_epilogue_frame_restore(
        call_conv: crate::isa::CallConv,
        flags: &crate::settings::Flags,
        isa_flags: &Self::F,
        frame_layout: &crate::FrameLayout,
    ) -> crate::machinst::SmallInstVec<Self::I> {
        todo!()
    }

    fn gen_return(
        call_conv: crate::isa::CallConv,
        isa_flags: &Self::F,
        frame_layout: &crate::FrameLayout,
    ) -> crate::machinst::SmallInstVec<Self::I> {
        todo!()
    }

    fn gen_probestack(insts: &mut crate::machinst::SmallInstVec<Self::I>, frame_size: u32) {
        todo!()
    }

    fn gen_inline_probestack(
        insts: &mut crate::machinst::SmallInstVec<Self::I>,
        call_conv: crate::isa::CallConv,
        frame_size: u32,
        guard_size: u32,
    ) {
        todo!()
    }

    fn gen_clobber_save(
        call_conv: crate::isa::CallConv,
        flags: &crate::settings::Flags,
        frame_layout: &crate::FrameLayout,
    ) -> smallvec::SmallVec<[Self::I; 16]> {
        todo!()
    }

    fn gen_clobber_restore(
        call_conv: crate::isa::CallConv,
        flags: &crate::settings::Flags,
        frame_layout: &crate::FrameLayout,
    ) -> smallvec::SmallVec<[Self::I; 16]> {
        todo!()
    }

    fn gen_memcpy<F: FnMut(crate::ir::Type) -> crate::Writable<crate::Reg>>(
        call_conv: crate::isa::CallConv,
        dst: crate::Reg,
        src: crate::Reg,
        size: usize,
        alloc_tmp: F,
    ) -> smallvec::SmallVec<[Self::I; 8]> {
        todo!()
    }

    fn get_number_of_spillslots_for_value(
        rc: crate::machinst::RegClass,
        target_vector_bytes: u32,
        isa_flags: &Self::F,
    ) -> u32 {
        todo!()
    }

    fn get_machine_env(
        flags: &crate::settings::Flags,
        call_conv: crate::isa::CallConv,
    ) -> &regalloc2::MachineEnv {
        todo!()
    }

    fn get_regs_clobbered_by_call(
        call_conv_of_callee: crate::isa::CallConv,
        is_exception: bool,
    ) -> regalloc2::PRegSet {
        todo!()
    }

    fn get_ext_mode(
        call_conv: crate::isa::CallConv,
        specified: crate::ir::ArgumentExtension,
    ) -> crate::ir::ArgumentExtension {
        todo!()
    }

    fn retval_temp_reg(call_conv_of_callee: crate::isa::CallConv) -> crate::Writable<crate::Reg> {
        todo!()
    }
}
