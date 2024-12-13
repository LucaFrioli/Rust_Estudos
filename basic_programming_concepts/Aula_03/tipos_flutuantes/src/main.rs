mod table;

use std::{f32, f64}; // trazendo os métodos padrão que os tipos de pontos flutuantes (f32 e f64) contêm
use table::display_table;
// conceitos de função e modularidade serão tratados futuramente, então leia somente as informações declaradas no escopo main() ignorando a função display_table

fn main() {
    println!("Início do programa.\n\n");
    display_table();

    // Vamos começar declarando alguns números em ponto flutuante, iniciando com o tipo f32 (32 bits de precisão):
    let mut x: f32 = -1.0e-38; // -1.0 vezes 10 elevado a -38, valor próximo ao menor número normalizado de f32.
    println!("Declaramos x como um número em ponto flutuante de 32 bits, próximo ao limite inferior: {x}");

    // Podemos alterar seu valor para um número positivo maior, mas ainda dentro do intervalo suportado por f32:
    x = 3.4e38; // Valor próximo ao máximo permitido pelo tipo f32.
    println!("Agora x foi alterado para o valor máximo suportado por f32: {x}\n");

    // Vamos declarar um número com precisão dupla (f64), que possui maior capacidade e intervalo do que f32:
    let mut y: f64 = -1.79e308; // Valor mínimo normalizado de f64.
    println!("Declaramos y como um número em ponto flutuante de 64 bits, próximo ao limite inferior: {y}");

    // Alterando y para seu valor máximo suportado, ainda dentro do intervalo de f64:
    y = 1.79e308; // Valor máximo suportado por f64.
    println!("Agora y foi alterado para o valor máximo suportado por f64: {y}\n");

    // Realizando uma operação básica: divisão de x por um número.
    let mut z: f32 = x / 10.0; // Dividimos o valor atual de x por 10.
    println!("Dividimos x por 10, o novo valor de z é: {z}");

    // Para realizar operações entre valores de diferentes precisões (f32 e f64), é necessário converter um deles para o mesmo tipo:
    let soma = y + z as f64; // Convertendo z para f64 antes de somar com y.
    println!("Somamos y com z (convertido para f64), o resultado é: {soma}\n");

    // Trabalhando com valores especiais definidos na especificação IEEE 754:
    let infinito = f32::INFINITY; // Representa o infinito positivo em f32.
    let nan = f32::NAN; // Representa um valor "Not a Number" (não é um número).
    println!("Valores especiais: infinito é {infinito}, e NaN (Not a Number) é representado como: {nan}\n");

    // Qualquer operação envolvendo NaN sempre resulta em NaN:
    let resultado_nan = nan + 10.0; // Adicionando 10 a NaN ainda resulta em NaN.
    println!("Qualquer operação com NaN, como (NaN + 10), resulta em: {resultado_nan}\n");

    // Divisão por zero também possui comportamento especial para números em ponto flutuante:
    let div_zero = 1.0 / 0.0; // Resultado será infinito positivo.
    let neg_div_zero = -1.0 / 0.0; // Resultado será infinito negativo.
    println!("Divisão por zero: 1.0 / 0.0 é {div_zero}, e -1.0 / 0.0 é {neg_div_zero}\n");

    // Explorando o conceito de números subnormais (ou denormalizados), que são menores que o menor número normalizado:
    let subnormal: f32 = 1.0e-45; // Valor próximo ao menor número subnormal representável em f32.
    println!("Declaramos um número subnormal em f32: {subnormal}");

    // Operações envolvendo subnormais podem levar a perdas de precisão, resultando em zero:
    let subnormal_result = subnormal / 10.0; // Dividindo o subnormal por 10, podemos obter zero devido à precisão limitada.
    println!("Dividindo o número subnormal por 10, obtemos: {subnormal_result}\n");

    // Explorando um número muito pequeno em f64 para observar seu comportamento:
    let mut pequeno: f64 = 1.0e-323; // Valor próximo ao menor número representável em f64.
    println!("Declaramos um número muito pequeno em f64: {pequeno}");

    // Multiplicando este número pequeno por 10 para observar seu comportamento:
    pequeno = pequeno * 10.0;
    println!("Após multiplicar o número pequeno por 10, temos: {pequeno}\n");

    // Explorando o conceito de epsilon, que define a menor diferença entre dois números de ponto flutuante distintos:
    let epsilon = f64::EPSILON; // Valor do epsilon para f64.
    println!("O epsilon de f64, que é a menor diferença entre dois números distintos, é: {epsilon}\n");

    // Comparando dois números que diferem por menos que o epsilon:
    let a = 1.0; // Primeiro número.
    let b = 1.0 + epsilon / 2.0; // Segundo número, ligeiramente maior que a.
    println!("Comparação de dois números muito próximos: a = {a}, b = {b}");
    println!("a é igual a b? {}\n", a == b); // A comparação resulta em falso, pois b está fora do intervalo de tolerância.

    println!("\n\nFinal do programa.");
}
