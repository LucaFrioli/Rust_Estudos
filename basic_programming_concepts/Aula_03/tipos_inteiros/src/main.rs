mod table;

use std::{i128, u64}; // trazendo métodos std(standard/padrões) que o tipo u64 contém
use table::display_table;
// conceitos de função e modularidade serão tratados futuramente, então leia somente as informações declaradas no scopo main() ignorando a função display_table

fn main() {
    println!("Início do programa.\n\n");
    display_table();

    // vamos declarar alguns inteiros e com diversas tipagens para compreender melhor sua natureza na prática:
    let mut x:i8 = -128;
    println!("Vamos começar declarando o menor número possível dentro de um inteiro de 8 bits, veja seu valor: {x}");
    // por ser uma vvariavel mutávvel podemos mudar seu valor, mas apenas contendo o  número de bits declarados, veja a tabela da documentaação para entender melhor, tire o comentário da próxima linha e perceba que temos agora um erro
    //x= -129;    // isso não pode ser realizado, porque estoura a quantidade de memória que a tipagem suporta

    // agora vamos passar ele para um número positivo, veja que também iremos até o limite:
    x= 127; // esta atribuição é possível pois ainda está contendo 8 bits
    println!("Veja quen aqui conseguimos mudar o valor de x sem que obtenhamos erros, x agora vale {x}\n");

    // vamos criar uma nova variavel mutável agora com o tamanho de u16 que correspondem a apenas números inteiros positivos e comporta ranges(intervalos), de até 16 bits, vamos colocar o valor máximo dela menos o ultimo valor declarado de x, isso seria o número 65_408, isso paara começarmos a realizar calculos básicos. Além disso vamos declara-lo de fórma hexadecimal
    let mut y:u16 = 0xFF80;
    println!("Perceba que y mesmo declarado em hexadecimal ainda tem o valor sugerido no comentário: {y}");

    // vamos atingir o valor máximo do valor numérico de y agora somando x ao valor de y. Perceba que a declaração a seeguir se tirada do comentário não pode ocorrer pois os valores são de tipos distintos, mesmo sendo inteiros
    // y = y+x; //Perceba que se executarmos teremos um erro de tipagem 

    // agora perceba que podemos alterar a tipagem de forma que consista em uma mudança temporária dentro de uma operação, para que a soma de tipos possa corresponder, para isso utilizamos a palavra reservada `as`, veja a seguir
    y= y+x as u16;
    println!("Atingimos o valor máximo suportado por u16 dentro da vriavel y, veja : {y}\n");

    // veja que se tirarmos o comentário da próxima linha que soma mais um a y vamos esturar o valaor suportado por u16 e obtemos um erro
    // y= y+1; // perceba que ao executarmos o program iremos ter um erro de overflow

    // vamos neste momento então criar uma outra variavel, a qual iremos tipar com i32, e vamos utilizar o menor número possível declaravel dentro de 32 bits, -2.147.483.648, vamos declarar este número utilizando a notação literal binária e utilizar separadores para conseguir ter melhor leitura do número, OBS: isto é pouco usula, mas poderá ser necessário dependendo de que tipo de software está sendo desenvolvido
    let mut z:i32 = -0b1000_0000_0000_0000_0000_0000_0000_0000 ;
    println!("Veja que temos extamente o número desejado declarado na variavel z, porém declarado em binário {z}");

    // agora vamos somar o numero resultante y que temos anteriormente a variável z, ele deverá dar uma saída de −2.147.418.113.
    z= z+ y as i32;
    println!("Aqui temos o resultado de z esperado: {z}\n");
    

    // vamos agora para uma declaração de um número máximo contendo 64bits não assinado, para isso iremos utilizar um método que gera o máximo que um tipo numárico pode caber, para isso utilizaremos duaz vezes o dois pontos, e então iremos adicionar MAX, fica assim ::MAX;
    let mut alph: u64 = u64::MAX;

    println!("O valor da variavel alph é igua a {alph}");

    // podemos então por exemplo reduzir  o ultimo valor de z deste número gerado, para isso vamos utilizar um método que o transformar em positivo, e depois devemos o converter para o tipo u64, então aí sim poderemos realizar a subtração padrão. Para transforma-lo em um número positivo utilizamos o método .abs() e a transformação ed tipo padrao antes conhecida.
    alph = alph - z.abs() as u64;
    println!("Veja o número gerado dentro da variável alph reduzida {alph}\n");
    

    // finalmente vamos ver o tamnaho do calculo que dá para realizar com um inteiro assinado de 128 bits, vamos pegar o menor número plausível dentro de uma variavel de 128 bits
    let mut bta: i128 = i128::MIN;
    println!("O valor de bta é de {bta}");

    // veja o valor de beta mais o número de alph
    bta = bta + alph as i128;
    println!("O valor de bta + alph é de {bta}\n");


    println!("\n\nFinal do programa.");
}
