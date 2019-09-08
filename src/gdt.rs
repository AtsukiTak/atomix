//! ## IST
//!
//! x86_64 アーキテクチャでは、例外発生時に事前に定義されたスタック領域に
//! スイッチすることができる。
//! これはハードウェアレベルで発生するため、CPU がスタックフレームを push
//! する前に実行される。
//! このメカニズムを Interrupt Stack Table (IST) と呼ぶ。
//!
//! IST はスタック領域へのポインタを最大7つ持つテーブルである。
//!
//! それぞれの例外ハンドラについて、どの IST に登録されているスタック領域を
//! 使うかを指定することができる。
//!
//! ### IST を作成する
//!
//! IST は、レガシーなメカニズムである Task State Segment (TSS) の一部である。
//! TSS は 64bit 環境以前では様々な情報を指定するメカニズムであったが、
//! x86_64 では単に2つの stack table を指定するだけになっている。
//! その一つが IST である。
//! ちなみにもう一つは Privilege Stack Table というもので、
//! Privilege Level が変化した時に使用するスタック領域を規定する。
//!
//! ### TSS をロードする
//!
//! TSS を CPU にロードするのは、若干面倒。
//! TSS は歴史的理由から Segmentation System を用いているため、
//! まず新しい Segment を作ってから、そこにロードする必要がある。
//! 具体的には、新しい Segment Descriptor を Global Descriptor Table (GDT)
//! に追加し、その index とともに `ltr` 命令を実行し、 CPU にロードする。

use crate::println;
use lazy_static::lazy_static;
use x86_64::{structures::tss::TaskStateSegment, VirtAddr};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096;
            // immutable な領域として宣言してしまうと、
            // bootloader はそれを read-only なページにマップしてしまう
            // かもしれないので、mutable な領域として宣言する必要がある。
            // これは Rust 的にめっちゃ unsafe なので、後で修正する。
            //
            // また、guard page も設定していないので、
            // double fault handler はスタックを使い切らないように
            // 注意する必要がある。
            // もし使い切ると、その先のメモリ領域を侵食してしまう。
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            // stack 領域は上に伸びていく
            stack_end
        };
        tss
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    _err_code: u64,
) {
    panic!("EXCEPTION : DOUBLE FAULT\n{:#?}", stack_frame);
}

#[cfg(test)]
mod tests {
    use crate::{serial_print, serial_println};

    #[test_case]
    fn test_breakpoint_exception() {
        serial_print!("test_breakpoint_exception...");
        // invoke a breakpoint exception
        // should continue if exception handler works correctly
        x86_64::instructions::interrupts::int3();
        serial_println!("[ok]");
    }
}
