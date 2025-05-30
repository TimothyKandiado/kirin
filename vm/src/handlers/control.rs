use crate::{VM, VmStatus};
use instructions::Instruction;

impl VM {
    #[inline]
    pub(crate) fn do_return(&mut self, _instruction: Instruction) {
        if let Some(frame) = self.frames.pop() {
            if let Some(return_address) = frame.return_address {
                self.instruction_pointer = return_address;
                return;
            }
        }

        self.status = VmStatus::Halted
    }
}
