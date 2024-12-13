use std::{i128, i16, i32, i64, i8, u128, u32, u64, u8}; // trazendo métodos std(standard/padrões) que os tipos contém

pub fn display_table() {
    let min_8: i8 = i8::MIN;
    let max_8: i8 = i8::MAX;
    let unsigned_8: u8 = u8::MAX;

    let min_16: i16 = i16::MIN;
    let max_16: i16 = i16::MAX;
    let unsigned_16: u16 = u16::MAX;

    let min_32: i32 = i32::MIN;
    let max_32: i32 = i32::MAX;
    let unsigned_32: u32 = u32::MAX;

    let min_64: i64 = i64::MIN;
    let max_64: i64 = i64::MAX;
    let unsigned_64: u64 = u64::MAX;

    let min_128: i128 = i128::MIN;
    let max_128: i128 = i128::MAX;
    let unsigned_128: u128 = u128::MAX;

    println!("{:<10} | {:<60} | {:<60} | {:<60}", "TIPO", "ASSINADO MÁXIMO", "ASSINADO MÍNIMO", "NÃO ASSINADO MÁXIMO");
    println!("{:-<10}-+-{:-<60}-+-{:-<60}-+-{:-<50}", "", "", "", "");

    println!("{:<10} | {:<60} | {:<60} | {:<60}", "i8", max_8, min_8, unsigned_8);
    println!("{:<10} | {:<60} | {:<60} | {:<60}", "i16", max_16, min_16, unsigned_16);
    println!("{:<10} | {:<60} | {:<60} | {:<60}", "i32", max_32, min_32, unsigned_32);
    println!("{:<10} | {:<60} | {:<60} | {:<60}", "i64", max_64, min_64, unsigned_64);
    println!("{:<10} | {:<60} | {:<60} | {:<60}", "i128", max_128, min_128, unsigned_128);
    println!("\n\n\n");
}