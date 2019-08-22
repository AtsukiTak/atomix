//! ## Interrupt Descriptor Table
//! 例外をキャッチし適切に処理するために、Interrupt Descriptor Table (IDT) を設定する。
//! IDT は複数のエントリーから構成される。
//! それぞれのエントリーは、対応する例外（divide by zero など）の処理方法（ジャンプ先など）が記載されている。
//! 例外が発生すると、CPU は IDT の対応するエントリーを見て、どのように処理するか判断する。
//!
//! IDT を設定するためには、メモリ上の任意の位置に IDT の設定を保存し、
//! そのアドレスを `lidt` 命令で読み込む。
//!
//! ### 参照
//! - https://os.phil-opp.com/cpu-exceptions/#the-interrupt-descriptor-table
//!
//! ### 疑問
//! IDT のエントリーに記述されている Interrupt Stack Table Index って何？
//! 何に使われる？
//! →Interrupt Stack Table の指定 index にスイッチするって書いてある。
//! →そもそも Interrupt Stack Table ってなに？
//!
//!
//! ## extern "x86-interrupt"
//! `extern` keyword は関数の呼び出し規約（calling convention）を規定する。
//! 例えば `extern "C"` は、関数がC呼び出し規約に準ずるようにする。
//!
//! 呼び出し規約は、関数呼び出しの詳細を規定する。
//! 例えば、関数の引数をどのように渡すか（レジスタ渡しなのか、スタック渡しなのか）、戻り値をどのように返すか、など。
//!
//! `x86-interrupt` 呼び出し規約は、そのような呼び出し規約の一種。
//!
//! ### `x86-interrupt` 呼び出し規約は C呼び出し規約とどう違うか
//!
//! C呼び出し規約では、レジスタを２種類に分類している。
//! 関数呼び出しの前後で値が変わるものと、変わらないもの、の２つ。
//! 関数呼び出しの前後で値が変わるレジスタは、
//! もし関数を呼び出した側（caller）がレジスタの以前の値を
//! 保持しておきたければ、caller自身がそれを保存する必要があるので、
//! caller-saved なレジスタと呼ばれる。
//! 反対に、関数呼び出しの前後で値が変わらないよう要求されるレジスタは、
//! もし呼び出された側（callee）がそのレジスタを使いたい場合は
//! callee が関数開始時の値を保存し、関数終了時に復元する必要がある。
//! そのため callee-saved なレジスタと呼ばれる。
//!
//! C言語の関数の場合はこれでいいが、割り込み命令の関数の場合は問題がある。
//! 割り込み命令の関数はいつ呼び出されるかわからない（割り込みがいつ発生するかわからない）ので、
//! caller-save なレジスタを規定することができない。
//! そのため、`x86-interrupt` 呼び出し規約は、 **（グローバルな）全ての**
//! レジスタの値を callee-saved なレジスタと規定する。
//!
//! `extern "x86-interrupt"` がつけられた関数が呼び出される前に、
//! （たぶん）llvmによってレジスタの値がpushされ、
//! rustcはそれを引数として関数を呼び出す。
//!
//! ### unstable
//! `x86-interrupt` 呼び出し規約はまだ安定化されていないので、 `#![feature(abi_x86_interrupt)]` を
//! `lib.rs` フィアルの先頭に追加する必要がある。

use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    /// 当たり前だが、IDT のライフタイムは static である必要がある.
    ///
    /// lazy_static を利用して宣言された変数は、メモリの data 領域に置かれる（たぶん）
    /// data 領域にメモリを確保して、最初の呼び出し時に初期化処理を行う（たぶん）
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION : BREAKPOINT");
    println!("{:#?}", stack_frame);
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
