fn main() {
    println!("Inicio do programa!");
    
    let x = 2;
    println!("\n\nX vale {}",x); // perceba que para concatenar valores em strings de saída literais podemos utilizar o operador {}, podendo adicionar o nome da variavel dentro dele ou, ao final da string literal adicionar na ordem de inserção os nomes das variaveis separados por vírgula. Perceba também que temos um \n dentro de nossa string, ele indica para o progrma que deve pular uma linha cada \n é uma quebra de linha. 

    // vamos tetar alterar o valor de x sem ser mátavel e pedir para o cargo rodar a apllicação (Para visualizar o erro retire os comentários de x=3 e a macro println)
    // x=3;  
    // println!("Novo valor de x tentando mudaar uma let sem mut {x}"); // perceba que o cargo gerou um erro indicando que existe um erro 
    /*
        error[E0384]: cannot assign twice to immutable variable `x`
         --> src/main.rs:7:5
          |
        4 |     let x = 2;
          |         - first assignment to `x`
        ...
        7 |     x=3;
          |     ^^^ cannot assign twice to immutable variable
          |
        help: consider making this binding mutable
          |
        4 |     let mut x = 2;
          |         +++

        For more information about this error, try `rustc --explain E0384`.
        error: could not compile `declaring_variables` (bin "declaring_variables") due to 1 previous error

    */

    // porém agora perceba como funciona variaveis mutáveis perceba a palavra reservada mut na frente de y
    let mut y = 4;
    println!("\n\nO valor contido na variavel y é igual a {}",y);
    y = y+2;
    println!("\nPerceba agora que conseguimos alterar o valor da variavel y sem obtenção de erros, ela agora vale {}",y);

    // mas caso queiramos mudar o tipo do valor atribuido, perceba que vai gera um erro (retire os comentários para poder vizualizar os erros!)
    //y = "Olá";
    //println!("Agora y não deve ser mais um número e diz {}",y); // perceba que isto não deu certo pois estamos mudando o tipo da variavel criado. y incialmente teve um inteiro sendo atribuido em sua memória, ao tentarmos atribuir agora uma string, isso ocasionará erro, pois a linguagem Rust é fortemente tipada.
    /* Veja o erro a seguir:
        error[E0308]: mismatched types
          --> src/main.rs:37:9
           |
        31 |     let mut y = 4;
           |                 - expected due to this value
        ...
        37 |     y = "Olá";
           |         ^^^^^ expected integer, found `&str`

        For more information about this error, try `rustc --explain E0308`.
        error: could not compile `declaring_variables` (bin "declaring_variables") due to 1 previous error
    */


    // Porém se por algum motivo necessitarmos que uma variavel assuma diversos tipos ao longo do programa podemos utilizar o conceito de shadowing
    let z = "Olá, me chamo Carlos!";
    println!("\n\nPerceba que o valor de z é um texto, e diz \"{z}\"");
    // agora para alterar não só o valor da variavel z, mas também o tipo poodemos usar o shadowing que nos permite destruir a variavel z anterior e reatribuí-la
    let z = 16;
    println!("\nA valor de z agora não é mais uma string mas sim um inteiro com valor igual a {}. veja a seguir um texto corrido que sofrerá diversos shadowings em diferentes escopos (leia os comentários do arquivo para compreender melhor o que está ocorrento): \n\n",z);
    // então temos que z apartir deste momento representa 16 e recebe apenas numeros inteiros. Porém ainda tem uma coisa ainda mais legal que pode ocorrer com o conceito de shadowing, porém que deve ser utilizada com parciomonia, que são os conceitos de escopos, veja o exeplo asseguir ainda utilizando a variavel z
    print!("Mesmo que eu tenha somente {} anos, ",z);
    {
        // veja que dentro deste escopo realizamos um sombreamento da variavel z anterior
        let z = "vinte e três";
        println!("muitas pessoas acreditam que eu tenho {} anos de idade.",z);
    }
    // porém sem a necessidade de reatribuir z como 16 podemos obter o valor anterior dela pois o shadowing contendo a string "vinte e três" estava encapsulada dentro de um escopo menor que a variavel z com atribuição em 16
    println!("Mas ainda assim não deixei de ter {} anos de idade!", z);

    println!("\n\nFinal do programa!");
}
