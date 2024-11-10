[**Voltar**](./configLintingToll.md)

# Instalando o NodeJS

- [Windows](#no-windows)
- [MacOS](#no-macos)
- [Linux](#no-linux)

## No Windows

1. Acesse o site do NodeJS: [https://nodejs.org/en/](https://nodejs.org/en/) e baixe a versão LTS mais recente (recomendada para estabilidade).
2. Siga o processo de instalação do assistente, permitindo a instalação de todas as dependências e arquivos necessários.
3. Teste a instalação no terminal com os comandos:

```powershell
node -v # exibe a versão do NodeJS
npm -v # exibe a versão do npm (gerenciador de pacotes)
```

4. Instale o nvm para gerenciar diferentes versões do NodeJS:

- Acesse o GitHub do NVM-Windows: [https://github.com/coreybutler/nvm-windows/releases](https://github.com/coreybutler/nvm-windows/releases) e baixe o instalador `nvm-setup.exe`.
- Execute o instalador e siga os passos do assistente.
- Verifique a instalação com o comando `nvm -v`.

5. Aprenda comandos básicos do nvm:

- `nvm ls`: lista as versões instaladas do NodeJS.
- `nvm use x.x.x`: altera para a versão `x.x.x` do NodeJS.
- `nvm install x.x.x`: instala a versão `x.x.x` do NodeJS.
- `nvm uninstall x.x.x`: desinstala a versão `x.x.x` do NodeJS.

**Exemplo de uso do nvm:**

```powershell
nvm ls # lista as versões instaladas

# instala uma versão antiga
nvm install 16.14.0

# altera para a versão antiga
nvm use 16.14.0

# verifica a versão atual
nvm ls

# volta para a versão LTS mais recente
nvm use lts

# desinstala a versão antiga
nvm uninstall 16.14.0
```

**Lidando com políticas de execução no Windows:**

Ao desenvolver com NodeJS, scripts de frameworks ou pacotes externos podem precisar ser executados. Para permitir isso, configure a ExecutionPolicy no Powershell como administrador:

```powershell
Set-ExecutionPolicy RemoteSigned
```

[**Voltar**](./configLintingToll.md)

**Observações importantes:**

- **RemoteSigned**: Permite scripts assinados digitalmente por editores confiáveis (recomendado).
- **Unrestricted**: Permite qualquer script, mas **não é recomendado** por motivos de segurança.

# No MacOS

1. Verifique se o nvm está instalado:

```bash
nvm -v
```

2. Se o nvm não estiver instalado:

- Execute o seguinte comando no terminal:

```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
```

- Cole a saída do comando `export NVM_DIR` no terminal.
- Reinicie o terminal.
- Verifique a instalação com `nvm -v`.

---

##### Utilizando o nvm:

1. Instalando o NodeJS:

- Verifique as opções disponíveis:

```bash
nvm ls-remote
```

- Utilize o nvm para instalar a versão LTS:

```bash
nvm install node
```

- Verifique se é a versão LTS:

```bash
node -v
```

2. Gerencie as versões do NodeJS com os comandos `nvm`:

- `nvm use`: altera para a versão desejada.
- `nvm ls`: lista as versões instaladas.
- `nvm uninstall`: desinstala a versão desejada.
- `nvm ls-remote`: lista as versões disponíveis para instalação (não disponível no Windows).

[**Voltar**](./configLintingToll.md)

# No Linux

## Instalação linha a linha

1. Remova o NodeJS se já estiver instalado:

```bash
sudo apt purge --auto-remove nodejs -y
sudo rm /etc/apt/sources.list.d/nodesource.list*
```

2. Instale o curl e o nvm:

```bash
sudo apt install curl -y

curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash # sempre verifique a última versão do nvm e troque o 39.7 pela versão mais recente
source ~/.bashrc
```

## Crie um script para instalar o nvm (recomendado)

```txt
# REMOVE O NODE SE ELE EXISTIR
sudo apt purge --auto-remove nodejs -y
sudo rm /etc/apt/sources.list.d/nodesource.list*

# INSTALA O CURL
sudo apt install curl -y

# INSTALA O NVM
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
```

**4. Rode o Script:**

- Abra um terminal com privilégios administrativos.
- Navegue até o diretório onde salvou o script (por exemplo, `cd ~/scripts`).
- Execute o script: `/bin/bash nomedoarquivo.txt`.

**Após a execução do script, utilize o `nvm` normalmente. Para isso, [clique aqui e veja.](#utilizando-o-nvm)**

[**Voltar**](./configLintingToll.md)
