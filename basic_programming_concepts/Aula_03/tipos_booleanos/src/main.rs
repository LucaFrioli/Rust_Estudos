mod table;

use table::display_table;

fn main() {
    println!("Início do Programa!\n\n");
     
    println!("Antes de mais nada primeiro devemos entender um pouco da lógica booleana, para isso observe a seguinte tabela:\n");
    display_table();


    println!("\nApós entendermos esta situação podemos então entender que é possível realizar comparações dentro de programação, isso torna nosso código ainda mais inteligente.\n\nAlém de realizar cálculos, podemos criar condições de comparação para eles.\n\nVamos a um exemplo mais prático:\n - Suponhamos que temos três números inteiros de 8 bits gerados por cálculos, e gostaríamos de compará-los. Primeiro devemos descobrir se A é maior ou igual a B, e se C é diferente de B ou se C é menor do que A.\n Para realizar a comparação, iremos tomar que A:u8 = 10+7, B:u8 = 29-4, e C:u8 = 13+15-3. Devemos antes realizar a comparação dentro do operador `||`. Isso deverá resultar em um valor falso.\n");

    // Inicialização das variáveis:
    let a: u8 = 10 + 7; // A = 17
    let b: u8 = 29 - 4; // B = 25
    let c: u8 = 13 + 15 - 3; // C = 25

    // Primeira comparação:
    let mut comparacao: bool = a >= b && (c != b || c < a);

    // Explicação da ordem de avaliação:
    // 1. Avaliar `a >= b` (comparação): 17 >= 25 -> false
    // 2. Avaliar `c != b` (comparação): 25 != 25 -> false
    // 3. Avaliar `c < a` (comparação): 25 < 17 -> false
    // 4. Avaliar `(c != b || c < a)` (OR lógico): false || false -> false
    // 5. Avaliar `a >= b && (c != b || c < a)` (AND lógico): false && false -> false

    println!("\n\nO resultado dessa comparação é {}.", comparacao); // Resultado: false

    println!("\nAgora vamos comparar se A é igual a B ou se C é igual a B:");

    // Segunda comparação:
    comparacao = a == b || c == b;

    // Explicação da ordem de avaliação:
    // 1. Avaliar `a == b` (comparação): 17 == 25 -> false
    // 2. Avaliar `c == b` (comparação): 25 == 25 -> true
    // 3. Avaliar `a == b || c == b` (OR lógico): false || true -> true

    println!("Isso gera um resultado {}.", comparacao); // Resultado: true

    println!("\nAgora que temos um valor para `comparacao`, vamos ver se a expressão `(comparacao && a < b || c <= b) && c >= b` gera um valor verdadeiro:");

    // Terceira comparação:
    comparacao = (comparacao && a < b || c <= b) && c >= b;

    // Explicação da ordem de avaliação:
    // 1. Avaliar `comparacao` (valor anterior): true
    // 2. Avaliar `a < b` (comparação): 17 < 25 -> true
    // 3. Avaliar `comparacao && a < b` (AND lógico): true && true -> true
    // 4. Avaliar `c <= b` (comparação): 25 <= 25 -> true
    // 5. Avaliar `(comparacao && a < b || c <= b)` (OR lógico): true || true -> true
    // 6. Avaliar `c >= b` (comparação): 25 >= 25 -> true
    // 7. Avaliar `(comparacao && a < b || c <= b) && c >= b` (AND lógico): true && true -> true

    println!("O resultado desta expressão é {}.", comparacao); // Resultado: true

    println!("\n\nFinal do Programa!");
}
