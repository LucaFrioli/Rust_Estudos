[**Voltar**](../readme.md)

# Configurando a Ferramenta de Linting para Documentações

## Explicando por que utilizá-la

**Obs: caso queira ir direto para a configuração, [clique aqui para a parte de configuração](#configurando-o-linter)**

Uma ferramenta de _linting_ é uma ferramenta de análise de código que verifica erros e inconsistências em arquivos de texto (como código, documentação e configurações) para garantir que eles seguem um padrão de estilo e estrutura predefinidos. _Linters_ ajudam a identificar problemas como erros de sintaxe, inconsistências de formatação, práticas inadequadas e até problemas de acessibilidade em textos. Ferramentas de _linting_ são muito populares em programação, onde ajudam a manter o código consistente e fácil de ler, mas também podem ser aplicadas em outros tipos de arquivos, como documentações em Markdown.

### Aplicação em Arquivos Markdown para Documentação

No contexto de documentação em repositórios, o _linting_ para arquivos Markdown serve para:

1. **Garantir Consistência no Estilo de Escrita**: Markdown é amplamente usado para documentação devido à sua simplicidade, mas a forma como ele é escrito pode variar de pessoa para pessoa. Uma ferramenta de _linting_ pode ajudar a padronizar elementos como títulos, listas, links e blocos de código, promovendo uma formatação uniforme.

2. **Aumentar a Clareza e a Acessibilidade**: _Linters_ para Markdown frequentemente incluem verificações de acessibilidade, como garantir que imagens tenham textos alternativos ou que links sejam claros e relevantes. Isso torna a documentação mais fácil de entender e mais acessível para um público mais amplo.

3. **Prevenir Erros Comuns**: Ferramentas de _linting_ podem detectar problemas como links quebrados, títulos ausentes, espaçamentos incorretos e erros de sintaxe, prevenindo que esses problemas se propaguem e dificultem o uso da documentação.

4. **Simplificar Revisões de Documentação**: Em projetos colaborativos, o _linting_ automatiza a verificação de formatação e estilo, economizando tempo nas revisões e garantindo que a equipe possa se concentrar no conteúdo em si.

### Como uma Ferramenta de Linting Funciona com Markdown

Em Markdown, ferramentas de _linting_ como `markdownlint` e o ESLint com plugins específicos para Markdown (como `eslint-plugin-markdown`) verificam o conteúdo conforme as regras de estilo configuradas, que podem incluir:

- **Regras de Formatação**: Controlam a estrutura dos arquivos, como limitar o comprimento das linhas, adicionar quebras de linha após títulos e formatar listas corretamente.
- **Regras de Sintaxe Markdown**: Asseguram que o Markdown esteja sendo usado conforme o esperado, evitando uso incorreto de cabeçalhos ou listas aninhadas.
- **Regras de Conteúdo**: Algumas ferramentas verificam detalhes do conteúdo, como garantir que links estejam válidos, que palavras-chave estejam escritas corretamente e que o conteúdo siga um padrão consistente.

### Benefícios na Documentação de Repositórios

Para repositórios, especialmente em projetos open source ou de equipes, uma ferramenta de linting para Markdown:

- **Facilita a Manutenção**: Garante que a documentação permaneça clara, atualizada e fácil de entender.
- **Aprimora a Qualidade e Legibilidade**: Padrões consistentes fazem com que a documentação pareça profissional e mais acessível para novos colaboradores.
- **Automatiza a Verificação**: Em ambientes CI/CD, ferramentas de linting podem ser integradas para verificar automaticamente cada alteração feita na documentação, evitando que erros ou problemas de formatação se acumulem ao longo do tempo.

## Configurando o Linter

Primeiro, é interessante que você já tenha o **NodeJS** instalado em seu computador. Além disso, é útil ter conhecimento básico do npm, pois estamos utilizando como linter a ferramenta **ESLint**.

Caso não tenha o NodeJS instalado, [clique aqui para o guia de como instalá-lo](./nodejsIntall.md).

Após isso, abra o repositório e, na raiz do projeto, execute o seguinte comando:

```bash
npm i
```

Em seguida, instale a extensão do VSCode **ESLint** (com selo de verificação da Microsoft).

Após concluir a instalação das dependências e da extensão, adicione as seguintes configurações no arquivo JSON de configurações do VSCode:

```json
{
  "editor.codeActionsOnSave": {
    "source.fixAll.eslint": "always",
    "source.fixAll": "always"
  },
  "eslint.validate": [
    "javascript",
    "javascriptreact",
    "typescript",
    "typescriptreact"
  ],
  "editor.formatOnSave": true
}
```

[**Voltar**](../readme.md)
