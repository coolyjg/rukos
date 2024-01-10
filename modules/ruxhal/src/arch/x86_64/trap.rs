/* Copyright (c) [2023] [Syswonder Community]
 *   [Ruxos] is licensed under Mulan PSL v2.
 *   You can use this software according to the terms and conditions of the Mulan PSL v2.
 *   You may obtain a copy of Mulan PSL v2 at:
 *               http://license.coscl.org.cn/MulanPSL2
 *   THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 *   See the Mulan PSL v2 for more details.
 */

use x86::{controlregs::cr2, irq::*};

use super::context::TrapFrame;

core::arch::global_asm!(include_str!("trap.S"));

const IRQ_VECTOR_START: u8 = 0x20;
const IRQ_VECTOR_END: u8 = 0xff;

#[no_mangle]
fn x86_trap_handler(tf: &mut TrapFrame) {
    match tf.vector as u8 {
        #[cfg(feature = "musl")]
        INVALID_OPCODE_VECTOR => {
            debug!(
                "rax: {:#x}, rdi: {:#x}, rsi: {:#x}, rdx:{:#x}, \nr10: {:#x}, r8: {:#x}, r9: {:#x}",
                tf.rax, tf.rdi, tf.rsi, tf.rdx, tf.r10, tf.r8, tf.r9,
            );
            let ret = crate::trap::handle_syscall(
                tf.rax as usize,
                [
                    tf.rdi as _,
                    tf.rsi as _,
                    tf.rdx as _,
                    tf.r10 as _,
                    tf.r8 as _,
                    tf.r9 as _,
                ],
            );
            tf.rax = ret as _;
            tf.rip += 2; // `syscall` instruction
        }
        PAGE_FAULT_VECTOR => {
            if tf.is_user() {
                warn!(
                    "User #PF @ {:#x}, fault_vaddr={:#x}, error_code={:#x}",
                    tf.rip,
                    unsafe { cr2() },
                    tf.error_code,
                );
            } else {
                panic!(
                    "Kernel #PF @ {:#x}, fault_vaddr={:#x}, error_code={:#x}:\n{:#x?}",
                    tf.rip,
                    unsafe { cr2() },
                    tf.error_code,
                    tf,
                );
            }
        }
        BREAKPOINT_VECTOR => debug!("#BP @ {:#x} ", tf.rip),
        GENERAL_PROTECTION_FAULT_VECTOR => {
            panic!(
                "#GP @ {:#x}, error_code={:#x}:\n{:#x?}",
                tf.rip, tf.error_code, tf
            );
        }
        IRQ_VECTOR_START..=IRQ_VECTOR_END => crate::trap::handle_irq_extern(tf.vector as _),
        _ => {
            panic!(
                "Unhandled exception {} (error_code = {:#x}) @ {:#x}:\n{:#x?}",
                tf.vector, tf.error_code, tf.rip, tf
            );
        }
    }
}
