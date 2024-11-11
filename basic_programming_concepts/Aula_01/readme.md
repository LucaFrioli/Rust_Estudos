# Instalando e criando o primeiro Hello World de nossa aplicação

## Instalando a linguagem

- [Instalação para Windows](#instalação-no-windows)
- [Instalação para Linux e MacOs](#instalação-no-linux-e-macos)
- [Comandos Úteis](#comandos-úteis)

### Pré-requisitos

- Conexão com a internet
- Acesso ao terminal do sistema operacional

### Instalação no Windows

1. **Baixar o Instalador**:
   - Acesse [https://rustup.rs](https://rustup.rs) e clique em **"Install Rust"**.
   - Alternativamente, abra o terminal do Windows (Prompt de Comando ou PowerShell) e execute:
     ```powershell
     iwr -useb https://win.rustup.rs | iex
     ```
2. **Executar o Instalador**:

   - O comando acima iniciará o instalador do Rust.
   - Siga as instruções na tela para concluir a instalação.

3. **Configurar o Path**:

   - O instalador do `rustup` normalmente adiciona o caminho do Rust automaticamente ao PATH do sistema.
   - Para verificar, feche e reabra o terminal e execute:
     ```bash
     rustc --version
     ```
   - A saída deve exibir a versão do Rust instalada.

4. **Atualizar e Gerenciar Ferramentas**:
   - Use o comando `rustup update` para manter o Rust atualizado.

---

### Instalação no Linux e MacOs

1. **Baixar e Executar o Instalador**:

   - No terminal, execute o comando:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   - Isso fará o download do script de instalação e iniciará o processo.

2. **Seguir as Instruções do Instalador**:

   - Durante a instalação, o `rustup` perguntará se você deseja configurar o PATH automaticamente. Recomenda-se aceitar.

3. **Verificar Instalação**:

   - Após a instalação, abra um novo terminal (ou reinicie o shell) e execute:
     ```bash
     rustc --version
     ```
   - Deve exibir a versão do Rust instalada.

4. **Atualizar e Gerenciar Ferramentas**:
   - Para atualizações e gerenciamento de componentes, use `rustup update`.

---

### Comandos úteis

- **Verificar instalação do Rust**:
  ```bash
  rustc --version
  ```
- **Atualizar Rust**:
  ```bash
  rustup update
  ```
- **Desinstalar Rust**:
  ```bash
  rustup self uninstall
  ```

---

Com o Rust instalado, você poderá compilar programas Rust e usar o gerenciador de pacotes `cargo`, que vem junto com o `rustup`.

# Criando nosso primeiro hello world

Esta primeira aula iremos criar dois hello Worlds distintos, um utilizando o rustc e outro iremos utilizar o cargo para criar o projeto.

Para usuários do windows utilize o powershell, para ter uma maior consistencia de comandos.

Primeiramente criaremos um diretetório que receberá nosso primeiro hello world, para isso abriremos nosso terminal, e utilizaremos o comando mkdir, para criar o primeiro diretório.

```bash
mkdir hello_world
```

Então dentro do diretório criaremos um arquivo chamado `main.rs`, `.rs` é a extensão padrão de arquivos rust. Para isso primeiro iremos entrar em nosso diretório e depois criatr o arquivo. Veja a seguir o comando para isso:

```bash
cd hello_world && touch main.rs
```

Após isso podemos então abrir o projeto no VSCode com o comando `code .`, ao abrir o projeto nos depararemos com um arquivo main.rs. Neste arquivo então iremos declarar a função de entrada de nosso código que é a seguinte:

```rust
fn main() {
    println!("Hello world!");
}
```

Vamos destrinchar passo a passo o que é cada elemento desta função :

### Linha 1: Definição da Função `main`

```rust
fn main() {
```

1. **`fn`**: Esta palavra-chave indica a definição de uma função em Rust. Todas as funções em Rust começam com `fn`.

2. **`main`**: Este é o nome da função. No caso de uma aplicação em Rust, o ponto de entrada obrigatório para execução é uma função chamada `main`. Quando o programa é executado, ele começa a rodar a partir dessa função.

3. **`()`**: Os parênteses depois de `main` indicam que essa função não recebe nenhum parâmetro. Em Rust, todas as funções precisam dos parênteses, mesmo que estejam vazios.

4. **`{`**: A chave de abertura `{` marca o início do bloco de código da função. Todo o código que compõe a função `main` estará entre `{` e `}`.

---

### Linha 2: Comando `println!`

```rust
    println!("Hello world!");
```

1. **`println!`**: `println!` é uma _macro_ em Rust. Macros são identificadas pelo `!` no final do nome. A macro `println!` imprime texto no console e adiciona uma nova linha automaticamente no final.

2. **`("Hello world!")`**: Os parênteses contêm o argumento passado para a macro `println!`. Neste caso, o argumento é a string `"Hello world!"`, que será exibida no console.

3. **`"Hello world!"`**: Esta é uma string literal, um tipo de dado que representa texto entre aspas duplas (`"`). Este texto será exatamente o que a função imprime no console.

4. **`;`**: O ponto e vírgula (`;`) termina a instrução em Rust. Cada linha de comando dentro da função geralmente precisa de um ponto e vírgula para indicar o final da instrução.

---

### Linha 3: Fechando a Função `main`

```rust
}
```

1. **`}`**: A chave de fechamento (`}`) encerra o bloco de código da função `main`. É onde Rust entende que o código da função termina.

---

Essa função `main` é simples e executa apenas uma tarefa: imprime `"Hello world!"` no console quando o programa é executado.
