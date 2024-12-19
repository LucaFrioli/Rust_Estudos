mod table;

use table::display_table;

fn main() {
    println!("Início do programa.\n\n");
    display_table();

    // O tipo char em Rust representa um único caractere Unicode escalar.
    // Vamos declarar um caractere simples:
    let c: char = 'A';
    println!("Declaramos c como um caractere: {c}");

    // O tipo char pode representar mais do que apenas caracteres ASCII.
    // Vamos declarar um caractere Unicode:
    let smiley: char = '😊';
    println!("Declaramos smiley como um caractere Unicode: {smiley}");

    // Podemos verificar o tamanho em bytes de um char. 
    // Note que sempre terá 4 bytes, independentemente do caractere armazenado.
    println!("O tamanho em bytes de um char é sempre: {} bytes", std::mem::size_of::<char>());

    // Vamos explorar o valor numérico do caractere em Unicode:
    let unicode_value = smiley as u32; // Convertendo o char para seu código Unicode
    println!("O valor Unicode do smiley é: U+{:X}", unicode_value);

    // Explorando caracteres especiais e escapes:
    let newline: char = '\n';
    let tab: char = '\t';
    println!("Caractere especial para nova linha: {newline}");
    println!("Caractere especial para tabulação: {tab}");

    // Comparação de caracteres:
    let letra1: char = 'a';
    let letra2: char = 'b';
    println!("Comparando letras: {letra1} < {letra2} é {}", letra1 < letra2);

    // Métodos úteis para char:
    // Verificar se o caractere é alfabético:
    println!("'{letra1}' é alfabético? {}", letra1.is_alphabetic());

    // Verificar se é numérico:
    let num: char = '9';
    println!("'{num}' é numérico? {}", num.is_numeric());

    // Verificar se é um caractere de espaço:
    let space: char = ' ';
    println!("'{space}' é um caractere de espaço? {}", space.is_whitespace());

    // Transformar letras em maiúsculas ou minúsculas:
    println!("'{letra1}' em maiúscula é '{}'", letra1.to_uppercase());
    println!("'{letra2}' em minúscula é '{}'", letra2.to_lowercase());

    println!("\n\nFinal do programa.");
}


