# Sobre o Cargo

---

## Sumário

1. [Criando nosso primeiro "Hello World" com o Cargo](#criando-nosso-primeiro-hello-world-com-o-cargo)
   - [Compreendendo os arquivos iniciais](#compreendendo-os-arquivos-iniciais)
   - [Compilando arquivos com o Cargo](#compilando-arquivos-com-o-cargo)
   - [Entendendo a estrutura de diretórios criada pelo Cargo](#entendendo-a-estrutura-de-diretórios-criada-pelo-cargo)
     - [Estrutura e propósito da pasta Target](#estrutura-e-propósito-da-pasta-target)
   - [Executando o programa a partir do Cargo](#executando-o-programa-a-partir-do-cargo)

---

Antes de mais nada, devemos entender mais profundamente o que é o `cargo`. Ele é o gerenciador de pacotes e sistema de build do Rust, projetado para facilitar o desenvolvimento. Com o `cargo`, você pode inicializar novos projetos, compilar o código, gerenciar dependências e executar testes de forma simplificada. Ele atua como uma ponte entre o desenvolvedor e as ferramentas do ecossistema Rust, simplificando tarefas complexas. Seu arquivo de configuração principal, o `Cargo.toml`, organiza metadados do projeto e define bibliotecas necessárias. Essa integração prática acelera a produtividade e mantém o código organizado e eficiente.

---

## Criando nosso primeiro "Hello World" com o Cargo

Agora que entendemos o propósito do `cargo`, podemos criar nosso primeiro projeto. Para isso, navegamos até a pasta onde desejamos trabalhar, e utilizamos o seguinte comando no terminal:

```bash
cargo new hello_cargo
```

Após executar o comando, note que uma nova pasta chamada `hello_cargo` foi criada. Vamos explorar seu conteúdo.

---

### Compreendendo os arquivos iniciais

Dentro da pasta, encontramos o arquivo `Cargo.toml`, que é a configuração principal do projeto. Ele contém informações como nome, versão e dependências. Ao abrir o arquivo, veremos:

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
```

#### Explicação linha a linha:

- **`[package]`**  
  Define as configurações do pacote, incluindo nome, versão e edição.

- **`name = "hello_cargo"`**  
  Nome do projeto, usado para identificar o pacote no _crates.io_ (se publicado) e para gerar executáveis.

- **`version = "0.1.0"`**  
  Indica a versão do projeto, seguindo o padrão [SemVer](https://semver.org/).

- **`edition = "2021"`**  
  Define a edição do Rust usada no projeto. A edição mais recente (`2021`) é recomendada.

- **`[dependencies]`**  
  Lista dependências externas do projeto. Está vazia por padrão.

Também notamos que o arquivo `main.rs`, que criamos manualmente em outros exemplos, agora está no diretório `src`. É aqui que todo o código do projeto será escrito.

Ao abrir o arquivo `src/main.rs`, veremos uma estrutura inicial parecida com a que criamos manualmente:

```rust
fn main() {
    println!("Hello, world!");
}
```

---

### Compilando arquivos com o Cargo

Além de criar a estrutura inicial, o `cargo` permite compilar o projeto facilmente. Para compilar, usamos:

```bash
cargo build
```

Esse comando gera o executável na pasta `target/debug`. O processo é similar ao uso do `rustc`, mas agora automatizado e integrado ao sistema de pacotes do Rust.

---

### Entendendo a estrutura de diretórios criada pelo Cargo

Após rodar o comando `cargo build`, algumas mudanças são perceptíveis no diretório do projeto. Uma delas é o arquivo `Cargo.lock`, que:

- É gerado automaticamente pelo `cargo` para registrar as versões exatas de todas as dependências utilizadas.
- Garante consistência entre builds, mesmo em máquinas ou momentos diferentes.
- É atualizado automaticamente quando as dependências mudam e deve ser incluído no controle de versão para projetos de aplicativos, porém se estiver criando uma biblioteca ele deve ser declarado no `.gitignore`.

Além disso, a pasta `target` é criada para armazenar os artefatos de compilação.

#### Estrutura e propósito da pasta Target

1. **`target/debug`**  
   Contém artefatos gerados no modo de desenvolvimento (_debug_), com verificações extras para facilitar a depuração.

2. **`target/release`**  
   Usada no modo otimizado (_release_), criado com o comando `cargo build --release`. Os binários aqui são otimizados para produção.

3. **Cache de Dependências**  
   O `cargo` armazena cópias compiladas das dependências, reduzindo o tempo de recompilação.

4. **Arquivos auxiliares**  
   Inclui arquivos como `CACHEDIR.TAG` e `.rustc_info.json`, que ajudam o compilador e ferramentas do ecossistema.

> **Nota:** A pasta `target` é gerada automaticamente e não deve ser incluída no controle de versão. Ela pode ser recriada pelo comando `cargo build`.

---

### Executando o programa a partir do Cargo

Depois de explorar a estrutura do projeto, podemos finalmente rodar nosso programa. Uma maneira direta seria executar o binário gerado manualmente:

```bash
./target/debug/hello_cargo
```

No entanto, o `cargo` oferece uma alternativa mais prática. Podemos compilar e executar o projeto com:

```bash
cargo run
```

Este comando recompila o código, se necessário, e executa o programa automaticamente. Por exemplo, se alterarmos o texto em `main.rs` para:

```rust
println!("Hello, Cargo!");
```

Ao rodar `cargo run` novamente, veremos no console:

```bash
cargo run
   Compiling hello_cargo v0.1.0 (/path/to/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/hello_cargo`
Hello, Cargo!
```

Assim, o `cargo run` combina eficiência e simplicidade, sendo ideal para o ciclo de desenvolvimento.

Agora que entendemos um pouco mais como criamos, e compilamos projetos Rust estamos aptos a continuar nossa jornada, para isso [clique aqui! COntinue aprendendo!](./../Aula_02/readme.md)