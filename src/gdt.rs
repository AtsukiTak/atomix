//! ## GDT
//!
//! Global Descriptor Table (GDT) は、Paging がまだデファクトスタンダード
//! になっていなかった頃、プログラム同士を独立させる
//! メモリセグメンテーションのために使われていた。
//! セグメンテーションは 64-bit モードではもうサポートされていないが、
//! 今でも GDT は主に２つの用途に必要である。
//! １つは kernel 空間と user 空間をスイッチするため。
//! そしてもう１つが、TSS をロードするためである。
//! GlobalDescriptorTable 構造体を作成しているコードをみるとよくわかる。
//!
//! TSS については tss モジュールを参照

pub mod tss;

use lazy_static::lazy_static;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors)= {
        let mut gdt = GlobalDescriptorTable::new();
        // code_selectorとtss_selectorはそれぞれ、コード領域とTSS領域が
        // どのセグメントなのかを指す値。(GDTのインデックス)
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&tss::TSS));
        (gdt, Selectors { code_selector, tss_selector })
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    // CPU に GDT をロードする。
    GDT.0.load();

    unsafe {
        // 新しいGDTをロードした後でも、code segment registerは
        // 古い値を保持し続けているのでそれを更新してやる必要がある。
        x86_64::instructions::segmentation::set_cs(GDT.1.code_selector);

        // 新しいGDTを作成したので、CPUにそのGDT上のTSSを使うように指示する。
        // モジュールドキュメントの仮想手順の最後のステップ。
        x86_64::instructions::tables::load_tss(GDT.1.tss_selector);
    }
}
