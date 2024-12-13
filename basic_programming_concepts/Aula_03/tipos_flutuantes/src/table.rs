use core::{f32, f64};

pub fn display_table() {
    let float_32_max: f32 = f32::MAX;
    let float_32_min: f32 = f32::MIN;

    let float_64_max: f64 = f64::MAX;
    let float_64_min: f64 = f64::MIN;

    println!("{:-<10}-+-{:-<60}-+-{:-<60}|", "", "", "");
    println!("{:<10} | {:<60} | {:<60}|", "TIPO", "MÁXIMO", "MÍNIMO");
    println!("{:-<10}-+-{:-<60}-+-{:-<60}|", "", "", "");

    println!("{:<10} | {:<60} | {:<60}|", "f32", float_32_max, float_32_min);
    println!("{:<10} | {:<60e} | {:<60e}|", "f64", float_64_max, float_64_min);

    println!("{:-<10}-+-{:-<60}-+-{:-<60}|", "", "", "");
}
