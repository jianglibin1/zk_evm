use crate::cpu::kernel::aggregator::KERNEL;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RegistersState {
    pub program_counter: usize,
    pub is_kernel: bool,
    pub stack_len: usize,
    pub context: usize,
}

impl Default for RegistersState {
    fn default() -> Self {
        Self {
            program_counter: KERNEL.global_labels["main"],
            is_kernel: true,
            stack_len: 0,
            context: 0,
        }
    }
}
