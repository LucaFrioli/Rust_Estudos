**[Voltar](./readme.md)**

# Informações extras sobre a Linguagem

## Por que utilizar Rust?

Rust é amplamente adotado em ambientes que exigem **segurança e controle**, tais como sistemas críticos e infraestrutura de software, além de:

- **Segurança de memória**: Rust evita uma classe inteira de bugs comuns em C/C++, como buffer overflows e ponteiros nulos, que são a causa de muitos problemas de segurança.
- **Concorrência segura**: A linguagem facilita o desenvolvimento de programas concorrentes sem condições de corrida, usando o sistema de tipos para garantir que os dados compartilhados sejam acessados corretamente em um ambiente multithread.
- **Eficiência sem coleta de lixo**: Rust não possui um garbage collector como Java ou Go, e lida com a alocação e liberação de memória automaticamente através do sistema de ownership. Isso oferece controle de desempenho em tempo real, essencial para aplicações que não podem tolerar pausas de GC.

## Para que tipo de código Rust é compilada?

Rust é uma linguagem compilada e gera código de máquina nativo, o que significa que, ao final do processo de compilação, o código resultante é executado diretamente pelo sistema operacional sem a necessidade de uma máquina virtual ou interpretador. Isso torna Rust adequada para:

- **Desenvolvimento de sistemas operacionais**: devido ao controle de memória e acesso direto ao hardware.
- **Software embarcado**: como em dispositivos de Internet das Coisas (IoT), onde o uso eficiente de memória e a ausência de coleta de lixo são essenciais.
- **Aplicações de alta performance**: como motores de jogos, processamento de imagens, navegadores, criptografia e outros serviços que exigem baixo tempo de resposta.

## Vantagens do Rust em relação a C e C++

1. **Segurança e controle de memória**: Rust evita acessos inválidos e leaks de memória, onde o C e C++ requerem um gerenciamento manual complexo e propenso a erros. A verificação de segurança em tempo de compilação de Rust elimina muitos problemas que causariam falhas em C/C++.
2. **Concorrência sem condições de corrida**: Enquanto C e C++ oferecem recursos para concorrência, eles não garantem segurança na execução de múltiplas threads. Rust, por outro lado, oferece segurança com seu sistema de ownership e borrowing, que impede condições de corrida, algo muito difícil de conseguir em C/C++ sem bibliotecas externas.

3. **Ausência de garbage collector**: Embora o C/C++ também não possua coleta de lixo, Rust combina a ausência do GC com uma estratégia automatizada e segura de gerenciamento de memória, facilitando o desenvolvimento e a manutenção de código eficiente e sem pausas de execução.

## Segurança em Rust

Rust foi projetada desde o início para ser uma linguagem segura e minimamente propensa a falhas. A linguagem utiliza um modelo de **ownership** (propriedade) e **borrowing** (empréstimo) que assegura que o acesso à memória seja seguro e que o uso de ponteiros seja gerenciado automaticamente pelo compilador. O sistema de ownership garante que cada valor no programa tenha um único proprietário e controla o escopo em que o valor é válido. Já o sistema de borrowing permite que variáveis possam ser compartilhadas de maneira segura, com referências mutáveis e imutáveis que são controladas rigorosamente pelo compilador, impedindo problemas comuns de concorrência.

## Ausência de Garbage Collector

Rust não utiliza garbage collection, pois faz um gerenciamento de memória automático baseado no sistema de ownership. Em vez de liberar memória com um coletor de lixo, Rust libera a memória quando o escopo do proprietário de um recurso termina, ou seja, no momento em que a variável sai de escopo. Isso torna Rust uma linguagem eficiente em sistemas onde pausas para coleta de lixo são inaceitáveis.

## Como Rust é compilada?

O processo de compilação de Rust é conduzido pelo **rustc**, seu compilador oficial. O compilador realiza uma série de verificações de segurança e otimizacões, incluindo:

1. **Verificação de segurança de memória**: Durante a compilação, o rustc realiza uma análise do controle de empréstimos (ownership e borrowing), verificando se não há referências inválidas (dangling pointers), uso de memória fora de limites (out-of-bounds) ou condições de corrida em múltiplas threads. Essas verificações tornam o código compilado mais seguro e menos propenso a erros em tempo de execução.

2. **Análise de tipo avançada**: Rust implementa tipagem estática, ou seja, os tipos de dados devem ser conhecidos durante a compilação, o que ajuda o compilador a otimizar o código e evitar erros de tipo.

3. **Compilação cruzada**: Rust também possui suporte para compilação cruzada, permitindo compilar código em uma plataforma para ser executado em outra, o que é comum em sistemas embarcados.

## Curiosidades e características únicas de Rust

1. **Ferramenta Cargo**: Cargo é o gerenciador de pacotes e build system de Rust. Ele facilita a criação de projetos, gerencia dependências e configura o ambiente de desenvolvimento, tornando o processo de compilação, teste e execução mais eficiente.

2. **Foco em desenvolvimento seguro e colaborativo**: Rust é conhecida por sua comunidade ativa e cuidadosa, que adota princípios de segurança e de compartilhamento de conhecimento. É uma das poucas linguagens modernas que tem uma "RFC" (Request for Comments), onde qualquer membro da comunidade pode propor e discutir mudanças na linguagem.

3. **Popularidade crescente em áreas críticas**: Rust é uma das linguagens mais bem avaliadas em comunidades de desenvolvedores, como o Stack Overflow. Empresas como Amazon, Microsoft e Dropbox usam Rust em sistemas críticos e aplicações de alta performance, o que mostra a confiabilidade e estabilidade da linguagem.

4. **Rust FFI (Foreign Function Interface)**: Rust é compatível com bibliotecas em C, o que facilita a integração de código legado e o desenvolvimento em ambientes onde C já esteja consolidado, como drivers de dispositivos e sistemas operacionais.

5. **Ferramentas e suporte para desenvolvimento de bibliotecas**: A linguagem oferece extensas bibliotecas padrão e de terceiros, e o próprio Cargo faz a gestão dessas dependências, permitindo um ciclo de desenvolvimento eficiente e ágil.

Rust se tornou uma escolha de referência para desenvolvedores que buscam performance sem abrir mão de segurança.

**[Voltar](./readme.md)**
