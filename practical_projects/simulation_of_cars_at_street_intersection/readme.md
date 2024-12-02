# Simulador de carros em cruzamento de rua:

Este projeto será desenvolvido com base no **Curso Básico de Linguagem Rust**, ministrado pelo professor _Rômulo Silveira de Oliveira_, disponível na plataforma Udemy.

**[Voltar](./../../readme.md)**

---

- **[Explicação do projeto](#explicação-do-projeto)**
- **...**

---

# Explicação do projeto

O simulador será construído ao longo das seções do curso, com cada seção culminando em uma aula prática que aplica progressivamente os conhecimentos adquiridos. Dessa forma, ao final do curso, teremos implementado todas as funcionalidades previstas no projeto.

Dentro deste projeto, iremos simular tanto o comportamento dos carros quanto o processo de cruzamento no tráfego e um semáfro. Os veículos, ao entrarem no perímetro delimitado, se tornarão automaticamente autônomos. A partir desse ponto, nosso programa assumirá o controle total, gerenciando aceleração e frenagem até que os veículos completem o cruzamento e saiam das margens da área simulada.

Durante a simulação, o programa deverá garantir que nenhum acidente ocorra entre os carros, promovendo um fluxo seguro e ordenado no cruzamento. O cenário a ser considerado é um cruzamento urbano com as seguintes características e dimensões, representado pela visualização abaixo:

```plaintext          
                     6 metro de largura
                         __I__     
                        |     |

                   _    |     |
                  |     |     |   Margem 30m...
     Margem 30m --|     |     |   ___I___  
                  |_    |     |  |       |                                   _  
------------------------+ + + +------------                                   | 
 [  ]   [  ]  [  ]      +  º  +              <-- via 2(direção da via ->>)    | -- 6 metros de largura
------------------------+ + + +------------                                  _| 
                        |  _  |       _
|___________________|   | | | |        |
          |             |  -  |        |
   150 m perímerto      |  _  |        | -- 150 m perímetro (Onde os carros se tronarão autônomos dentro do sistema)  
 (Onde os carros        | | | |        |  
  se tornarão           |  -  |        |   
  autônomos dentro      |     |       _|    
  do sistema)
                           ^
                           |           ^ 
                 via 1 (direção da via ^ )
                                       î  
```

Os carros na simulação seguirão as seguintes dimensões padrão:

- 4 metros de comprimento
- 2 metros de largura

A simulação ocorrerá em tempo real, o que significa que a velocidade dos carros será calculada em metros por segundo, com base em uma conversão de sua velocidade em quilômetros por hora (km/h). Assim, a passagem do tempo na simulação corresponderá diretamente ao tempo do mundo real.

Por exemplo, se um carro estiver a 20 m/s na simulação, ele deverá percorrer exatamente 20 metros virtuais em um segundo na vida real.