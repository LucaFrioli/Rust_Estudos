# Instalando e criando o primeiro "Hello World" em nossa aplicação

## Sumário

1. [Instalando a linguagem](#instalando-a-linguagem)
   - [Pré-requisitos](#pré-requisitos)
   - [Instalação no Windows](#instalação-no-windows)
   - [Instalação no Linux e MacOS](#instalação-no-linux-e-macos)
   - [Comandos úteis](#comandos-úteis)
2. [Criando o primeiro Hello World](#criando-o-primeiro-hello-world)
   - [Criando o projeto manualmente](#criando-o-projeto-manualmente)
   - [Explicação do código](#explicação-do-código)
3. [Entendendo conceito de Cargo](./hello_cargo.md)

---

## Instalando a linguagem

### Pré-requisitos

- Conexão com a internet.
- Acesso ao terminal do sistema operacional.

---

### Instalação no Windows

1. **Baixar e executar o instalador**:

   - Acesse [https://rustup.rs](https://rustup.rs) e clique em **"Install Rust"**.
   - Alternativamente, no terminal do Windows (Prompt de Comando ou PowerShell), execute:
     ```powershell
     iwr -useb https://win.rustup.rs | iex
     ```
   - Siga as instruções do instalador.

2. **Verificar instalação**:

   - Feche e reabra o terminal e execute:
     ```bash
     rustc --version
     ```
   - A versão do Rust instalada será exibida.

3. **Manter o Rust atualizado**:
   - Use:
     ```bash
     rustup update
     ```

---

### Instalação no Linux e MacOS

1. **Baixar e executar o instalador**:

   - No terminal, execute:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   - Siga as instruções na tela.

2. **Verificar instalação**:

   - Após concluir, reinicie o terminal e execute:
     ```bash
     rustc --version
     ```

3. **Manter o Rust atualizado**:
   - Utilize:
     ```bash
     rustup update
     ```

---

### Comandos úteis

- Verificar a instalação do Rust:
  ```bash
  rustc --version
  ```
- Atualizar o Rust:
  ```bash
  rustup update
  ```
- Desinstalar o Rust:
  ```bash
  rustup self uninstall
  ```

---

## Criando o primeiro "Hello World"

Nesta seção, criaremos dois programas _Hello World_: um utilizando diretamente o `rustc` e outro com o `cargo`.

### Criando o projeto manualmente

1. **Criar um diretório para o projeto**:

   - No terminal, crie o diretório `hello_world` com:
     ```bash
     mkdir hello_world
     ```
   - Acesse o diretório:
     ```bash
     cd hello_world
     ```

2. **Criar o arquivo de código**:

   - Crie um arquivo chamado `main.rs`:
     ```bash
     touch main.rs
     ```

3. **Escrever o código**:

   - Abra o arquivo `main.rs` no editor de texto, como o Visual Studio Code, usando:
     ```bash
     code .
     ```
   - Insira o seguinte código no arquivo:
     ```rust
     fn main() {
         println!("Hello, world!");
     }
     ```

4. **Compilar e executar o programa**:
   - Para compilar o código, utilize o `rustc`:
     ```bash
     rustc main.rs
     ```
   - Isso gerará um executável. Execute-o com:
     ```bash
     ./main
     ```

---

### Explicação do código

#### Linha 1: Definição da função `main`

```rust
fn main() {
```

- **`fn`**: Define uma função.
- **`main`**: Ponto de entrada do programa.
- **`()`**: Indica que a função não recebe parâmetros.
- **`{`**: Início do bloco de código.

#### Linha 2: Impressão no console

```rust
    println!("Hello, world!");
```

- **`println!`**: Macro que imprime texto no console, adicionando uma nova linha.
- **`"Hello, world!"`**: String a ser exibida.

#### Linha 3: Encerramento da função

```rust
}
```

- **`}`**: Marca o fim do bloco de código.

---

Com isso, você já criou e executou seu primeiro programa em Rust! Na próxima etapa, exploraremos o uso do `cargo` para gerenciar projetos Rust de forma mais eficiente. [Para isso continue aprendendo, clique Aqui!](./hello_cargo.md)
