# Tipos Primitivos: Tipos escalares

Rust é uma linguagem fortemente tipada. Por esse motivo, é fundamental compreender os seus tipos primitivos. Neste primeiro momento, focaremos no subgrupo dos tipos escalares e em como utilizá-los de maneira eficiente. Além disso, exploraremos as diferenças técnicas entre esses tipos e analisaremos como a escolha adequada da tipagem pode impactar diretamente a capacidade do software desenvolvido, bem como sua performance.

---
## Sumário

- [Entendendo o que são tipos escalares.](#entendendo-o-que-são-tipos-escalares)

- [Compreendendo tipos numéricos e suas diferentes declarações:](#compreendendo-tipos-numéricos-e-suas-diferentes-declarações)
    - [Qual a diferença entre um inteiro e um ponto flutuante;](#qual-a-diferença-entre-um-inteiro-e-um-ponto-flutuante)
    - [Entendendo as diferentes maneiras de declarar inteiros;](#entendendo-as-diferentes-maneiras-de-declarar-inteiros-em-rust)
    - [Entendendo as diferentes maneiras de declarar pontos flutuantes;](#entendendo-as-diferentes-maneiras-de-declarar-pontos-flutuantes-em-rust)

- [Compreendendo tipos booleanos:](#compreendendo-tipos-booleanos)
    - [O que são tipos booleanos e commpreendendo lógica booleana;](#o-que-são-tipos-booleanos-e-compreendendo-a-lógica-booleana)
    - [Operadores lógicos que geram tipos booleanos;](#operadores-lógicos-que-geram-tipos-booleanos)
    - [Comparações lógicas;](#comparações-lógicas)

- [Compreendendo tipo caractere:](#compreendendo-o-tipo-caractere-char-em-rust)
    - [O que é um caractere;](#o-que-é-um-caractere)
    - [O que são os padrões UTF8;](#o-que-são-os-padrões-utf-8)
    - [Natureza do tipo caractere em Rust;](#natureza-do-tipo-caractere-em-rust)

- [Bibliografia](#bibliografia)

---

## Entendendo o que são tipos escalares

Rust é uma linguagem moderna e poderosa, projetada para oferecer segurança, desempenho e controle sobre o gerenciamento de memória. Ao lidar com dados em Rust, um dos primeiros conceitos fundamentais que encontramos são os **tipos escalares**. Apesar de muitas vezes confundidos com os **tipos primitivos**, eles têm distinções importantes que influenciam como os dados são representados e manipulados no código.

---

### O Que São Tipos Escalares?  

Os **tipos escalares** são uma categoria de tipos de dados que representam um único valor indivisível. Em termos simples, eles são os blocos mais básicos de informação com os quais um programa pode trabalhar. Em contraste com tipos compostos, como arrays ou tuplas, os tipos escalares não possuem estrutura interna, sendo elementos unitários que representam valores discretos.  

Rust classifica como **tipos escalares** os seguintes:  

1. **Tipos Inteiros (integers)**  
2. **Tipos de Ponto Flutuante (floating-point numbers)**  
3. **Tipos Booleanos (booleans)**  
4. **Tipos de Caracteres (characters)**  

Esses tipos são amplamente utilizados para representar números, condições lógicas e símbolos textuais no programa.

---

### Diferença Entre Tipos Escalares e Tipos Primitivos  

Muitas vezes, os **tipos escalares** são chamados erroneamente de **tipos primitivos**, mas há uma diferença fundamental entre os dois conceitos:  

1. **Tipos Primitivos**  
   Os tipos primitivos são os tipos básicos e intrínsecos de uma linguagem de programação. Eles incluem tanto os **tipos escalares** quanto os **tipos compostos primitivos**, como arrays (`[T; N]`) e tuplas (`(T1, T2, ...)`). Assim, os tipos escalares são um subconjunto dos tipos primitivos.

2. **Tipos Escalares**  
   Exclusivamente usados para representar valores unitários. Eles não têm subdivisões ou agrupamentos internos, ao contrário dos tipos compostos.
 

| Característica            | Tipos Escalares                                  | Tipos Primitivos                |  
|---------------------------|--------------------------------------------------|---------------------------------|  
| Unidade de Valor          | Um único valor indivisível                       | Pode conter múltiplos valores   |  
| Exemplos                  | Inteiros, ponto flutuante, booleanos, caracteres | Arrays, tuplas, tipos escalares |  
| Estrutura Interna         | Ausente                                          | Presente (em tipos compostos)   |  

---

### Quais São os Tipos Escalares em Rust?  

Como mencionado, Rust define quatro categorias de tipos escalares, que exploraremos mais a fundo ao longo do texto:  

1. **Tipos Inteiros**: Representam números inteiros, positivos ou negativos, sem parte decimal.  
2. **Tipos de Ponto Flutuante**: Usados para armazenar números com casas decimais.  
3. **Tipos Booleanos**: Representam valores lógicos simples, como verdadeiro ou falso.  
4. **Tipos de Caracteres**: Armazenam um único caractere Unicode, incluindo símbolos e emojis.  

---

Ao longo desta documentação, abordaremos cada um desses tipos em detalhes, explicando suas propriedades, aplicações e peculiaridades. Esse entendimento é essencial para criar programas eficientes, seguros e otimizados em Rust.

## Compreendendo tipos numéricos e suas diferentes declarações

Rust é uma linguagem fortemente tipada e projetada para oferecer controle preciso sobre o gerenciamento de dados. Um dos pilares dessa precisão é o uso de **tipos numéricos**, que representam diferentes formas de armazenar e manipular números. Dentro dessa categoria, é essencial compreender a diferença entre **inteiros** e **pontos flutuantes**, dois tipos fundamentais em Rust e em muitas linguagens de programação.  

---

### Qual a Diferença Entre um Inteiro e um Ponto Flutuante?  

Embora ambos sejam usados para representar números, inteiros e pontos flutuantes têm características distintas que os tornam adequados para diferentes contextos.  

#### 1. Inteiros  

Um **inteiro** é um número que não contém frações ou partes decimais. Ele é usado para representar valores exatos, como contagens, índices e outras operações que envolvam números inteiros.  

##### Características dos Inteiros:  
- **Natureza Discreta:** Representam apenas valores inteiros.  
- **Exatidão:** Não possuem arredondamentos ou perdas de precisão.  
- **Faixa de Valores:** Dependem do tamanho do tipo (ex.: `i8` para 8 bits ou `i32` para 32 bits).  
- **Armazenamento Compacto:** Consomem menos memória em comparação a pontos flutuantes de tamanho equivalente.  

##### Exemplos de Inteiros:  
- **Números Positivos:** `0`, `42`, `1000`.  
- **Números Negativos:** `-1`, `-99`, `-2048`.  

---

#### 2. Pontos Flutuantes  

Um **ponto flutuante** é um número que pode conter frações ou partes decimais, sendo útil para cálculos matemáticos mais complexos e operações que requerem maior precisão. Eles são representados seguindo o padrão IEEE 754 em Rust.  

##### Características dos Pontos Flutuantes:  
- **Natureza Contínua:** Permitem valores com frações (ex.: `3.14`).  
- **Precisão Limitada:** Dependem do número de bits disponíveis, podendo resultar em erros de arredondamento.  
- **Faixa de Valores Extensa:** Podem armazenar números muito grandes ou muito pequenos, incluindo valores irracionais.  
- **Maior Custo Computacional:** Operações com pontos flutuantes consomem mais recursos computacionais.  

##### Exemplos de Pontos Flutuantes:  
- **Com Decimais:** `0.1`, `2.718`, `-3.14`.  
- **Números Exponenciais:** `1.23e4` (equivalente a \( 1.23 \times 10^4 \)).  

---

#### Diferenças Principais  

| Característica             | Inteiros                           | Pontos Flutuantes                 |  
|----------------------------|------------------------------------|-----------------------------------|  
| Representação              | Números sem casas decimais         | Números com ou sem casas decimais |  
| Precisão                   | Exata                              | Aproximada                        |  
| Tamanhos Disponíveis       | `i8`, `i16`, `i32`, `i64`, `i128`  | `f32`, `f64`                      |  
| Consumo de Recursos        | Menor                              | Maior                             |  
| Exemplos de Aplicações     | Contagens, índices                 | Cálculos científicos, física      |  

---

#### 3. Escolhendo Entre Inteiros e Pontos Flutuantes  

A escolha entre usar um inteiro ou ponto flutuante depende do contexto da aplicação.  

#### Quando Usar Inteiros:  
- **Contagem de Objetos:** Por exemplo, o número de itens em um carrinho de compras.  
- **Índices de Arrays:** Para acessar elementos de estruturas de dados.  
- **Códigos Exatos:** Identificadores únicos como IDs numéricos.  

#### Quando Usar Pontos Flutuantes:  
- **Cálculos Precisos:** Como operações científicas e trigonométricas.  
- **Representação de Frações:** Para medições, como peso ou altura.  
- **Simulações Matemáticas:** Onde números irracionais ou grandes faixas são comuns.  

---

Ao entender a diferença entre inteiros e pontos flutuantes, você pode tomar decisões informadas sobre qual tipo usar, maximizando a eficiência e a precisão do seu código em Rust. No próximo passo, exploraremos como esses tipos podem ser declarados e manipulados no código, detalhando suas aplicações e nuances práticas.


### Entendendo as Diferentes Maneiras de Declarar Inteiros em Rust  

Em Rust, os inteiros são um dos tipos numéricos mais fundamentais, amplamente usados para representar números sem parte decimal. Declarar inteiros de forma eficiente e correta é crucial para garantir que o programa atenda aos requisitos de memória, desempenho e precisão. Rust oferece uma ampla gama de opções para declaração de inteiros, permitindo aos desenvolvedores ajustar suas escolhas de acordo com as necessidades específicas da aplicação.

---

#### O Que São Inteiros?  

Os **inteiros** são números que representam valores discretos, positivos ou negativos, incluindo o zero, sem frações ou casas decimais. Em Rust, os inteiros podem ser **assinados** (podem assumir valores negativos) ou **não assinados** (apenas valores positivos).

---

#### Tipos Inteiros em Rust  

Rust fornece uma variedade de tipos inteiros, categorizados por:  

1. **Sinalização (Assinados ou Não Assinados):**
   - **Assinados (`iN`)**: Podem armazenar números negativos e positivos.  
   - **Não Assinados (`uN`)**: Armazenam apenas números positivos.

2. **Tamanho do Tipo:**  
   Define a quantidade de bits usados para armazenar o valor, determinando a faixa de valores que pode ser representada.  

#### Tabela de Tipos Inteiros  

| Tipo       | Tamanho (bits)            | Faixa de Valores Assinados      | Faixa de Valores Não Assinados |  
|------------|---------------------------|---------------------------------|--------------------------------|  
| `i8`       | 8                         | -128 a 127                      | 0 a 255                        |  
| `i16`      | 16                        | -32.768 a 32.767                | 0 a 65.535                     |  
| `i32`      | 32                        | -2.147.483.648 a 2.147.483.647  | 0 a 4.294.967.295              |  
| `i64`      | 64                        | \( -2^{63} \) a \( 2^{63}-1 \)  | \( 0 \) a \( 2^{64}-1 \)       |  
| `i128`     | 128                       | \( -2^{127} \) a \( 2^{127}-1 \)| \( 0 \) a \( 2^{128}-1 \)      |  
| `isize`    | Dependente da arquitetura | Mesmo que `i32` ou `i64`        | Mesmo que `u32` ou `u64`       |  

---

#### Declarando Inteiros em Rust  

Rust permite declarar inteiros de diversas maneiras, oferecendo flexibilidade e clareza no código. A seguir, exploramos as formas mais comuns de declaração.

#### 1. **Literal Decimal**  
A forma mais básica e usada para declarar inteiros. Representa números na base 10.  
```rust
fn main() {
    let inteiro_decimal = 42; // Declaração padrão.
}
```

---

#### 2. **Literal Hexadecimal**  
Inteiros podem ser declarados na base 16, precedidos por `0x`. Esse formato é comum em manipulação de bits e operações de baixo nível.  
```rust
fn main() {
    let inteiro_hexadecimal = 0x2A; // 42 em base decimal.
}
```

---

#### 3. **Literal Octal**  
Inteiros na base 8 são prefixados com `0o`. Embora menos comum, pode ser útil em certos contextos, como sistemas de arquivos UNIX.  
```rust
fn main() {
    let inteiro_octal = 0o52; // 42 em base decimal.
}
```

---

#### 4. **Literal Binário**  
Rust permite declarar números em base 2, precedidos por `0b`. Ideal para manipulação de bits e máscaras binárias.  
```rust
fn main() {
    let inteiro_binario = 0b101010; // 42 em base decimal.
}
```

---

#### 5. **Separadores Visuais**  
Para melhorar a legibilidade, números grandes podem usar underscores (`_`) como separadores visuais. Eles não afetam o valor do número.  
```rust
fn main() {
    let numero_grande = 1_000_000; // Mais legível que 1000000.
}
```

---

#### 6. **Tipos Explicitamente Especificados**  
Rust infere o tipo inteiro na maioria dos casos, mas você pode especificar explicitamente o tipo para garantir precisão e evitar erros.  
```rust
fn main() {
    let pequeno: u8 = 255;   // Inteiro não assinado de 8 bits.
    let grande: i64 = -9223372036854775808; // Inteiro assinado de 64 bits.
}
```

---

#### Como Escolher o Tipo Inteiro Adequado  

A escolha do tipo inteiro depende de vários fatores, incluindo a necessidade de economizar memória, representar valores negativos ou lidar com números grandes.  

##### Diretrizes Gerais:
- Use **`uN`** quando souber que os valores nunca serão negativos (ex.: contagens).  
- Prefira **`iN`** quando precisar lidar com números negativos.  
- Utilize **`isize`** ou **`usize`** para tamanhos de índices em arrays ou operações relacionadas à arquitetura do sistema.  
- Se o desempenho é crítico, escolha tipos menores (`i8`, `u8`) em sistemas embarcados ou com restrição de memória.  

---

Compreender as diferentes maneiras de declarar inteiros em Rust permite maior controle sobre os recursos do sistema e maior precisão no desenvolvimento de aplicações. Usar o tipo certo no contexto certo pode evitar erros e melhorar a eficiência, tornando seu código mais robusto e otimizado.

---

## Entendendo as Diferentes Maneiras de Declarar Pontos Flutuantes em Rust  

Os **pontos flutuantes** são tipos numéricos projetados para representar números que possuem casas decimais, tornando-os fundamentais em cálculos científicos, gráficos computacionais e operações financeiras. Rust oferece suporte a dois tipos principais de pontos flutuantes, com diferentes níveis de precisão. Compreender as maneiras de declarar esses tipos ajuda a otimizar cálculos e a lidar com as nuances de precisão inerentes a esses números.

---

### O Que São Pontos Flutuantes?  

Pontos flutuantes são números representados no formato de notação científica, onde um valor é descrito como um número base (a **mantissa**) multiplicado por uma potência de dez (o **expoente**). Em sistemas computacionais, eles seguem o padrão IEEE 754, que define como números de ponto flutuante são representados em bits.

---

### Tipos de Pontos Flutuantes em Rust  

Rust suporta dois tipos de pontos flutuantes:  

1. **`f32` (precisão simples):**
   - Ocupa 32 bits.  
   - Precisão de aproximadamente 7 dígitos significativos.  
   - Menor consumo de memória e mais rápido em arquiteturas que o suportam diretamente.  

2. **`f64` (precisão dupla):**
   - Ocupa 64 bits.  
   - Precisão de aproximadamente 15 dígitos significativos.  
   - Ideal para cálculos que exigem maior precisão, como em aplicações científicas.  

#### Comparação Entre `f32` e `f64`  

| Característica      | `f32`                         | `f64`                         |  
|---------------------|-------------------------------|-------------------------------|  
| Precisão            | ~7 dígitos significativos     | ~15 dígitos significativos    |  
| Consumo de Memória  | 32 bits                       | 64 bits                       |  
| Faixa de Valores    | \( \pm 10^{38} \)             | \( \pm 10^{308} \)            |  
| Uso Comum           | Gráficos, jogos               | Física, finanças, ciência     |  

---

### Declarando Pontos Flutuantes em Rust  

Rust permite declarar números de ponto flutuante de várias formas, dependendo das necessidades do desenvolvedor.  

#### 1. **Literal Decimal**  
A maneira mais simples de declarar pontos flutuantes é utilizando a notação decimal padrão.  
```rust
fn main() {
    let numero_decimal = 3.14; // Declarado como `f64` por padrão.
}
```

Por padrão, Rust infere que números de ponto flutuante declarados sem tipo explícito são do tipo `f64`.  

---

#### 2. **Notação Exponencial**  
Rust suporta a notação científica para representar números muito grandes ou muito pequenos. Esta forma é especialmente útil em cálculos científicos.  
```rust
fn main() {
    let numero_exponencial = 1.23e4; // 1.23 x 10^4, equivalente a 12.300.
    let pequeno = 5.67e-3;          // 5.67 x 10^-3, equivalente a 0.00567.
}
```

---

#### 3. **Tipos Explicitamente Especificados**  
Embora Rust possa inferir o tipo, é possível especificá-lo explicitamente para garantir a precisão ou otimizar o desempenho.  
```rust
fn main() {
    let precisao_simples: f32 = 3.14; // Ponto flutuante de precisão simples.
    let precisao_dupla: f64 = 3.141592653589793; // Ponto flutuante de precisão dupla.
}
```

---

#### 4. **Separadores Visuais**  
Assim como em inteiros, é possível usar underscores (`_`) para melhorar a legibilidade de números grandes.  
```rust
fn main() {
    let numero_grande = 1_000_000.000_001; // Mais fácil de ler.
}
```

---

#### 5. **Valores Especiais**  
Rust também suporta valores especiais para pontos flutuantes, definidos pelo padrão IEEE 754:  
- **`NaN` (Not a Number):** Representa valores indefinidos, como \( \frac{0}{0} \).  
- **`INFINITY` e `NEG_INFINITY`:** Representam positivo e negativo infinito.  
- **`0.0` e `-0.0`:** São valores distintos, embora geralmente tratados como equivalentes na maioria das operações.  

Exemplo:  
```rust
fn main() {
    let infinito = f64::INFINITY;
    let indefinido = f64::NAN;
    println!("Infinito: {}, Indefinido: {}", infinito, indefinido);
}
```

---

### Como Escolher Entre `f32` e `f64`?  

A escolha do tipo de ponto flutuante depende do equilíbrio entre precisão e desempenho:  

- **Prefira `f32`:**  
  - Quando o consumo de memória for uma prioridade, como em sistemas embarcados ou gráficos em tempo real.  
  - Em cálculos onde a precisão de 7 dígitos seja suficiente.  

- **Prefira `f64`:**  
  - Quando maior precisão for necessária, como em cálculos financeiros ou simulações científicas.  
  - Se estiver manipulando números muito grandes ou pequenos.  

---

Os pontos flutuantes em Rust são ferramentas poderosas para representar números com casas decimais. Entender as diferentes maneiras de declará-los e as implicações de cada escolha permite que os desenvolvedores criem aplicações mais robustas, precisas e otimizadas. Nos próximos passos, abordaremos como manipular esses números em operações matemáticas e contextos específicos.

---

## Compreendendo Tipos Booleanos  

Os tipos booleanos estão no cerne da programação moderna e são essenciais para expressar condições verdadeiras ou falsas. Em Rust, o tipo `bool` é usado para representar valores booleanos, que fazem parte do conjunto de tipos escalares primitivos. Este documento aborda os tipos booleanos e explora a lógica booleana em sua concepção matemática, incluindo operadores e comparações básicas.

---

### **O que são tipos booleanos e compreendendo a lógica booleana**  

Um **tipo booleano** é um tipo de dado escalar que pode assumir apenas dois valores:  
- **`true`** (verdadeiro).  
- **`false`** (falso).  

O termo "booleano" deriva do matemático **George Boole**, que desenvolveu a lógica algébrica agora conhecida como lógica booleana. Este sistema matemático é usado para representar relações lógicas e é a base do processamento computacional.  

#### **Lógica Booleana: Uma Abordagem Matemática**  

Na lógica booleana, toda expressão pode ser reduzida a um valor binário: verdadeiro (`1`) ou falso (`0`). Operações lógicas combinam esses valores usando operadores lógicos para construir expressões mais complexas.  

A lógica booleana é representada por um conjunto de operadores fundamentais:  
- **E lógico (AND):** O resultado é verdadeiro apenas se ambas as condições forem verdadeiras.  
- **OU lógico (OR):** O resultado é verdadeiro se ao menos uma das condições for verdadeira.  
- **NÃO lógico (NOT):** Inverte o valor de uma condição.  

Tabela verdade para os operadores básicos:  

| A     | B     | A AND B | A OR B | NOT A | NOT B |  
|-------|-------|---------|--------|-------|-------|  
| true  | true  | true    | true   | false | false |  
| true  | false | false   | true   | false | true  |  
| false | true  | false   | true   | true  | false |  
| false | false | false   | false  | true  | true  |  

Rust implementa esses operadores diretamente em seu sistema de tipos para permitir a manipulação de valores booleanos de forma eficiente e clara.  

---

### **Operadores Lógicos que Geram Tipos Booleanos**  

Rust oferece suporte a operadores lógicos fundamentais, que geram valores booleanos como resultado. Eles permitem combinar e transformar condições simples em expressões mais complexas.  

#### **E Lógico (`&&`)**  

O operador `&&` retorna `true` apenas se ambas as condições forem verdadeiras.  

**Exemplo:**  
```rust
fn main() {
    let condicao1 = true;
    let condicao2 = false;

    let resultado = condicao1 && condicao2; // false
    println!("Resultado de AND: {}", resultado);
}
```

#### **OU Lógico (`||`)**  

O operador `||` retorna `true` se ao menos uma das condições for verdadeira.  

**Exemplo:**  
```rust
fn main() {
    let condicao1 = true;
    let condicao2 = false;

    let resultado = condicao1 || condicao2; // true
    println!("Resultado de OR: {}", resultado);
}
```

#### **NÃO Lógico (`!`)**  

O operador `!` inverte o valor de um booleano, transformando `true` em `false` e vice-versa.  

**Exemplo:**  
```rust
fn main() {
    let condicao = true;

    let resultado = !condicao; // false
    println!("Resultado de NOT: {}", resultado);
}
```

#### **Combinações de Operadores**  

##### **Exemplo 1: Combinando `&&`, `||` e `!` em Uma Expressão**
Neste exemplo, combinamos operadores `&&` (AND), `||` (OR) e `!` (NOT) para criar uma expressão complexa.  

```rust
fn main() {
    let a = true;
    let b = false;
    let c = true;

    let resultado = (a || b) && !(b && c); // true
    println!("Resultado combinado: {}", resultado);
}
```
**Explicação:**  
1. A subexpressão `(a || b)` avalia para `true` (pois `a` é `true`).  
2. A subexpressão `(b && c)` avalia para `false` (pois `b` é `false`).  
3. O operador `!` inverte o resultado da segunda subexpressão para `true`.  
4. O operador `&&` combina os resultados das subexpressões para `true`.  

---

##### **Exemplo 2: Priorizando Operadores em Expressões Complexas**
Aqui, demonstramos como a prioridade entre operadores pode afetar o resultado.  

```rust
fn main() {
    let x = false;
    let y = true;
    let z = true;

    let resultado = x || y && z; // true
    println!("Resultado combinado: {}", resultado);
}
```
**Explicação:**  
1. O operador `&&` tem prioridade sobre `||`.  
2. A subexpressão `(y && z)` é avaliada primeiro e resulta em `true`.  
3. A subexpressão `(x || true)` é avaliada e resulta em `true`.  

---

##### **Exemplo 3: Trabalhando com Mais Variáveis e Operadores**
Este exemplo combina mais variáveis booleanas para criar uma expressão mais longa.  

```rust
fn main() {
    let a = true;
    let b = false;
    let c = true;
    let d = false;

    let resultado = (a && c) || (b || d) && !(a || b); // true
    println!("Resultado combinado: {}", resultado);
}
```
**Explicação:**  
1. `(a && c)` avalia para `true` (ambos são `true`).  
2. `(b || d)` avalia para `false` (ambos são `false`).  
3. `(a || b)` avalia para `true` (pois `a` é `true`), e `!(a || b)` avalia para `false`.  
4. A expressão completa resulta em `true` devido ao `||` que une a primeira subexpressão com as demais.  

---

##### **Exemplo 4: Uso de Resultados Parciais**
Neste exemplo, resultados intermediários são armazenados em variáveis para maior clareza.  

```rust
fn main() {
    let cond1 = true;
    let cond2 = false;
    let cond3 = true;

    let parcial1 = cond1 && cond3; // true
    let parcial2 = cond2 || cond3; // true
    let resultado = parcial1 || !parcial2; // true

    println!("Parcial1: {}", parcial1);
    println!("Parcial2: {}", parcial2);
    println!("Resultado final: {}", resultado);
}
```
**Explicação:**  
1. `parcial1` avalia para `true` porque `cond1` e `cond3` são `true`.  
2. `parcial2` avalia para `true` porque `cond3` é `true`.  
3. A expressão final usa `||` para unir `parcial1` e `!parcial2`.  

---

##### **Exemplo 5: Aninhando Operadores**
Expressões aninhadas ajudam a criar condições altamente detalhadas.  

```rust
fn main() {
    let a = true;
    let b = false;
    let c = true;
    let d = false;

    let resultado = (a && (b || c)) || (d && !(a || b)); // true
    println!("Resultado aninhado: {}", resultado);
}
```
**Explicação:**  
1. `(b || c)` avalia para `true` (pois `c` é `true`).  
2. `(a && true)` avalia para `true`.  
3. `(a || b)` avalia para `true`, e `!(a || b)` avalia para `false`.  
4. A expressão completa avalia para `true` devido ao primeiro bloco `(a && (b || c))`.  

---

Esses exemplos ilustram como os operadores lógicos podem ser usados para combinar condições booleanas de diferentes formas, ajudando a construir expressões claras e funcionais em Rust.

---

### **Comparações Lógicas**  

As comparações lógicas permitem avaliar relações entre valores, retornando um valor booleano como resultado. Em Rust, essas comparações são essenciais para validar condições em expressões.  

#### **Igual (`==`)**  

Verifica se dois valores são iguais.  

**Exemplo:**  
```rust
fn main() {
    let a = 5;
    let b = 5;

    let resultado = a == b; // true
    println!("a == b: {}", resultado);
}
```

#### **3.2. Maior ou Igual (`>=`)**  

Verifica se o primeiro valor é maior ou igual ao segundo.  

**Exemplo:**  
```rust
fn main() {
    let a = 10;
    let b = 5;

    let resultado = a >= b; // true
    println!("a >= b: {}", resultado);
}
```

#### **3.3. Menor ou Igual (`<=`)**  

Verifica se o primeiro valor é menor ou igual ao segundo.  

**Exemplo:**  
```rust
fn main() {
    let a = 3;
    let b = 8;

    let resultado = a <= b; // true
    println!("a <= b: {}", resultado);
}
```

#### **3.4. Menor (`<`)**  

Verifica se o primeiro valor é menor que o segundo.  

**Exemplo:**  
```rust
fn main() {
    let a = 2;
    let b = 7;

    let resultado = a < b; // true
    println!("a < b: {}", resultado);
}
```

#### **3.5. Maior (`>`)**  

Verifica se o primeiro valor é maior que o segundo.  

**Exemplo:**  
```rust
fn main() {
    let a = 9;
    let b = 4;

    let resultado = a > b; // true
    println!("a > b: {}", resultado);
}
```



#### **3.6. Diferente (`!=`)**

O operador `!=` verifica se dois valores são diferentes. Ele retorna `true` se os valores comparados não forem iguais e `false` caso sejam iguais. Este operador é amplamente utilizado para validar desigualdades em condições.

**Exemplo:**  
```rust
fn main() {
    let a = 5;
    let b = 10;

    let resultado = a != b; // true
    println!("a != b: {}", resultado);
}
```
---

Aqui estão seis exemplos que combinam operadores de comparação para criar expressões mais complexas. Esses exemplos ilustram como múltiplas condições podem ser avaliadas ao mesmo tempo em Rust.  

---
### Combinando diferentes tipos de comparações junto com os operadores lógicos

#### **Exemplo 1: Combinação de `!=` e `==`**  

```rust
fn main() {
    let a = 5;
    let b = 10;
    let c = 5;

    let resultado = (a != b) && (a == c); // true
    println!("(a != b) && (a == c): {}", resultado);
}
```

**Explicação:**  
1. `a != b` avalia para `true` porque `5` não é igual a `10`.  
2. `a == c` avalia para `true` porque `a` e `c` são iguais (`5`).  
3. O operador `&&` combina as condições, e o resultado final é `true`.  

---

#### **Exemplo 2: Combinação de `>` e `<`**  

```rust
fn main() {
    let x = 15;
    let y = 10;
    let z = 20;

    let resultado = (x > y) || (x < z); // true
    println!("(x > y) || (x < z): {}", resultado);
}
```

**Explicação:**  
1. `x > y` avalia para `true` porque `15` é maior que `10`.  
2. `x < z` avalia para `true` porque `15` é menor que `20`.  
3. O operador `||` retorna `true` se pelo menos uma das condições for verdadeira.  

---

#### **Exemplo 3: Combinação de `>=` e `<=`**  

```rust
fn main() {
    let a = 8;
    let b = 8;
    let c = 12;

    let resultado = (a >= b) && (b <= c); // true
    println!("(a >= b) && (b <= c): {}", resultado);
}
```

**Explicação:**  
1. `a >= b` avalia para `true` porque `a` é igual a `b`.  
2. `b <= c` avalia para `true` porque `b` é menor ou igual a `c`.  
3. O operador `&&` combina as condições, resultando em `true`.  

---

#### **Exemplo 4: Combinação de `!=` e `<=`**  

```rust
fn main() {
    let p = 30;
    let q = 25;
    let r = 30;

    let resultado = (p != q) || (r <= q); // true
    println!("(p != q) || (r <= q): {}", resultado);
}
```

**Explicação:**  
1. `p != q` avalia para `true` porque `30` não é igual a `25`.  
2. `r <= q` avalia para `false` porque `30` não é menor ou igual a `25`.  
3. O operador `||` retorna `true` porque uma das condições é verdadeira.  

---

#### **Exemplo 5: Combinação de `==` e `>`**  

```rust
fn main() {
    let num1 = 50;
    let num2 = 50;
    let num3 = 40;

    let resultado = (num1 == num2) && (num1 > num3); // true
    println!("(num1 == num2) && (num1 > num3): {}", resultado);
}
```

**Explicação:**  
1. `num1 == num2` avalia para `true` porque ambos têm o mesmo valor.  
2. `num1 > num3` avalia para `true` porque `50` é maior que `40`.  
3. O operador `&&` retorna `true` porque ambas as condições são verdadeiras.  

---

#### **Exemplo 6: Combinação de `>` e `<` com `!`**  

```rust
fn main() {
    let x = 10;
    let y = 20;
    let z = 15;

    let resultado = !(x > y) && (z < y); // true
    println!("!(x > y) && (z < y): {}", resultado);
}
```

**Explicação:**  
1. `x > y` avalia para `false` porque `10` não é maior que `20`.  
2. `!(x > y)` inverte o resultado para `true`.  
3. `z < y` avalia para `true` porque `15` é menor que `20`.  
4. O operador `&&` combina as condições, resultando em `true`.  

---

Esses exemplos mostram como combinar operadores de comparação para construir expressões mais ricas e flexíveis. Eles são úteis em muitas situações práticas, como validação de entradas, verificação de limites e construção de condições em algoritmos.

---

### Compreendendo o Tipo Caractere (`char`) em Rust  

O tipo `char` em Rust é uma representação de um único ponto de código Unicode, com suporte para caracteres de diferentes idiomas, emojis e outros símbolos. Para entender profundamente o tipo caractere, vamos abordar sua definição, os padrões UTF-8, e como sua natureza é implementada no Rust.

---

#### **O que é um Caractere?**  

Na computação, um caractere é a menor unidade de texto legível, podendo ser uma letra, número, símbolo ou espaço. Cada caractere é armazenado como um valor numérico que o representa em um padrão de codificação.

No Rust, o tipo `char` é usado para armazenar caracteres individuais, mas ao contrário de linguagens como C, ele não se limita a caracteres ASCII (7 bits). O Rust trabalha com Unicode, permitindo representar uma ampla gama de caracteres.  

**Exemplo:**  
```rust
fn main() {
    let letra = 'a'; // Um caractere ASCII.
    let emoji = '😊'; // Um caractere Unicode.
    println!("Caractere: {}, Emoji: {}", letra, emoji);
}
```

---

#### **O que são os Padrões UTF-8?**  

O padrão UTF-8 (Unicode Transformation Format - 8 bits) é uma forma de codificar caracteres Unicode em bytes. É amplamente utilizado porque é compatível com o ASCII e suporta todos os caracteres Unicode, incluindo idiomas com alfabetos distintos, emojis e símbolos técnicos.

- UTF-8 representa caracteres usando entre 1 a 4 bytes.  
- Caracteres comuns (ASCII) ocupam 1 byte.  
- Caracteres mais complexos (como emojis) podem ocupar até 4 bytes.  

**Exemplo de Representação UTF-8:**  
- O caractere `a` é armazenado como `0x61` em 1 byte.  
- O emoji `😊` é armazenado como `0xF0 0x9F 0x98 0x8A` em 4 bytes.  

Rust utiliza UTF-8 como padrão para codificação de strings, e o tipo `char` trabalha diretamente com pontos de código Unicode, abstraindo a complexidade da codificação.  

**Exemplo de Uso com Unicode:**  
```rust
fn main() {
    let letra = 'A'; // Representado como U+0041 no Unicode.
    let emoji = '🌍'; // Representado como U+1F30D no Unicode.
    println!("Letra: {}, Emoji: {}", letra, emoji);
}
```

---

#### **Natureza do Tipo Caractere em Rust**  

O tipo `char` em Rust não é limitado a 1 byte como em algumas linguagens. Ele representa um único ponto de código Unicode, armazenado em 4 bytes (32 bits). Isso significa que ele pode conter qualquer caractere válido do padrão Unicode.  

**Características do Tipo `char`:**  
1. Representa pontos de código Unicode (não necessariamente "caracteres visíveis").  
2. É ideal para manipular elementos individuais em strings.  
3. Trabalha bem com operações de comparação e iteração.  

**Exemplo de Comparação:**  
```rust
fn main() {
    let a = 'x';
    let b = 'y';

    if a < b {
        println!("'{}' vem antes de '{}'", a, b);
    } else {
        println!("'{}' vem depois ou é igual a '{}'", a, b);
    }
}
```

---

- O tipo `char` em Rust representa um único ponto de código Unicode, permitindo o uso de caracteres de várias linguagens e emojis.  
- O padrão UTF-8 é usado para strings, garantindo compatibilidade e flexibilidade na representação de textos.  
- A natureza do tipo `char` em Rust, com 4 bytes de largura, o torna poderoso para trabalhar com caracteres Unicode de forma eficiente e precisa.  

Com essa base, você pode manipular caracteres em Rust com segurança e aproveitar o suporte completo ao Unicode para criar aplicativos internacionais e robustos.

---

Após entendermos melhor esses conceitos fundamentais da linguagem Rust, podemos explorar os diversos métodos padrão disponíveis nos tipos escalares, que fazem parte da biblioteca padrão (*standard library*) da linguagem. Para acessar a documentação completa da biblioteca padrão, você pode [clicar aqui](https://doc.rust-lang.org/stable/std/index.html). Além disso, recomendamos revisar as implementações básicas comentadas disponíveis na aula, que fornecem exemplos práticos e esclarecedores. Caso deseje visualizar algumas tabelas, basta executar os exemplos fornecidos, que ilustram as operações e funcionalidades discutidas.

Além dos tipos escalares básicos, Rust também oferece tipos primitivos compostos, como *Tuples* e *Arrays*. Abordaremos essa temática na [próxima aula. Clique aqui para continuar seus estudos e explorar mais sobre esses conceitos](./../Aula_04/readme.md). 

---

# Bibliografia


- [Curso ministrado pelo Prof. Rômulo Silva de Oliveira](https://www.udemy.com/share/10aqpY3@bVJOeg1cn2IOx8kUaGicVb6pZ6Ni_e0QpYg5OuoXpiJ9zz5F0ejLJ8Xm7AgyDPf_mw==/)
- [The Rust Programming Language: 3.2 Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Doc Rust-lang: THe Rust Standard Library](https://doc.rust-lang.org/stable/std/index.html)
    - [Primitive Type i8](https://doc.rust-lang.org/stable/std/primitive.i8.html)
    - [Primitive Type u8](https://doc.rust-lang.org/stable/std/primitive.u8.html)
    - [Primitive Type i16](https://doc.rust-lang.org/stable/std/primitive.i16.html)
    - [Primitive Type u16](https://doc.rust-lang.org/stable/std/primitive.u16.html)
    - [Primitive Type i32](https://doc.rust-lang.org/stable/std/primitive.i32.html)
    - [Primitive Type u32](https://doc.rust-lang.org/stable/std/primitive.u32.html)
    - [Primitive Type i64](https://doc.rust-lang.org/stable/std/primitive.i64.html)
    - [Primitive Type u64](https://doc.rust-lang.org/stable/std/primitive.u64.html)
    - [Primitive Type i128](https://doc.rust-lang.org/stable/std/primitive.i128.html)
    - [Primitive Type u128](https://doc.rust-lang.org/stable/std/primitive.u128.html)
    - [Primitive Type isize](https://doc.rust-lang.org/stable/std/primitive.isize.html)
    - [Primitive Type usize](https://doc.rust-lang.org/stable/std/primitive.usize.html)
    - [Primitive Type f32](https://doc.rust-lang.org/stable/std/primitive.f32.html)
    - [Primitive Type f64](https://doc.rust-lang.org/stable/std/primitive.f64.html)
    - [Primitive Type bool](https://doc.rust-lang.org/stable/std/primitive.bool.html)
    - [Primitive Type char](https://doc.rust-lang.org/stable/std/primitive.char.html)