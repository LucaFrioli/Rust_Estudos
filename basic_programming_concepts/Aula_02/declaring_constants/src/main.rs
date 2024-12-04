const SWORD_BASE_DAMAGE: i32 = 3; // perceba que uma boa prática para criar constantes é mantelas com o nome todo em uppercase(todas as letras maísculas), e devem ser obrigatoriamente tipadas (veremos isto mais adiante, porém i32 corresponde a um número inteiro)
const SHARPENING_BUFF: i32 = 2;

fn main() {
    println!("Inicio do programa!\n\n");
    // podemos utilizar as constantes como padrões para calculos dentro de um fluxo de um progrma. Por exeplo vamos supor que um personagem fictício com uma vida incial de 25 pontos receba três espadadas, então podemos utilizar a contante para este calculo.
    let mut player_live = 25;
    let player_sword = SWORD_BASE_DAMAGE *2 + SHARPENING_BUFF;// perceba que aqui conseguimos criar com base em dano de espadas básicas e o buff novos tipos de espadas sem que seja necessário pensar na base. imagine que seja uma espada duas vezes melhor então utilizamos a base mínima de espadas e ela também está afiada então somamos o buff.
    println!("Seu perssonagem está com vida cheia: {}", player_live);
    
    let num_of_attacks = 3;
    let damage = num_of_attacks * SWORD_BASE_DAMAGE;// perceba que utilizamos o dano básico de espadas para definir o dano total inflingido
    player_live= player_live - damage;
    println!("\nPorém ele levou {} ataques do inimigo utilizando uma espada básica", num_of_attacks);
    println!("O dano total inflingido foi {}, o que te deixou com {} pontos de vida", damage, player_live);

    let mut enemy_live = player_sword * 3 + 2 ;
    println!("\nCom uma magia você percebeu que o inimigo está com a vida cheia e ela representa {} pontos.", enemy_live);
    println!("\nAssim você tem a certeza que terá que dar três golpes em seu inimigo. Isso por que você veio bem preparado e afiou a sua espada, e percebe que ela é duas vezes melhor que a do inimigo, ela está dando {} pontos de dano!", player_sword);
    let damage = num_of_attacks * player_sword;
    enemy_live = enemy_live - damage;
    println!("Então após a execução de três ataques o inimigo fica com {} pontos de vida. Ele está abalado não esperando tamanha retaliação", enemy_live);

    println!("\n\nFinal do programa!");
}
