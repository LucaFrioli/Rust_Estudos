macro_rules! separator {
    ($width:expr) => {
        println!("|{:-<1$}|","",$width);
    };
}

const TABLE_WIDTH:usize = 50;

pub fn display_table(){
    println!(" {:<width$}", "Entendendo lógica booleana", width = TABLE_WIDTH-1);
    separator!(TABLE_WIDTH+14);
    println!("| {:<width$}| {:<width$} | {:<width$} | {:<width$} | {:<width$} | {:<width$} |", "A","B","A and B"," A or B","not A","not B",width= (TABLE_WIDTH/6));
    separator!(TABLE_WIDTH+14);

    for &(a,b) in &[
        (true, true),
        (false, true),
        (false, false),
        (true, false)
    ]{
        println!("| {:<width$}| {:<width$} | {:<width$} | {:<width$} | {:<width$} | {:<width$} |", a, b, a && b,a || b, !a, !b,width= (TABLE_WIDTH/6));
    }

    separator!(TABLE_WIDTH+14);



    // Explicação sobre a precedência das operações booleanas
    println!("\n\nSobre a precedência das operações booleanas:");
    println!("- Em Rust, as operações booleanas seguem uma ordem de precedência específica:");
    println!("  1. `!` (NOT lógico): Sempre avaliado primeiro.");
    println!("  2. Comparações (`<`, `<=`, `>`, `>=`, `==`, `!=`): Avaliadas em seguida.");
    println!("  3. `&&` (AND lógico): Avaliado após as comparações.");
    println!("  4. `||` (OR lógico): Avaliado por último.");
    println!("- Quando várias operações aparecem juntas, a precedência determina a ordem de execução.");
    println!("- É sempre possível usar parênteses `()` para alterar a ordem de avaliação e tornar o código mais claro e previsível.\n");
    
}