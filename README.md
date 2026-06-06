# mrk-browser

O **mrk-browser** é um mini-browser modular, híbrido e de alta performance desenvolvido em Rust. O projeto foi projetado especificamente para rodar nativamente no **Omarchy Linux**, oferecendo duas interfaces de visualização independentes acionadas por inversão de controlo em tempo de execução, além de um pipeline especializado de transformação de conteúdo focado em conversão estática de dados.

## 🚀 Arquitetura do Sistema

O repositório está estruturado sob o conceito de um espaço de trabalho (Workspace) multi-crate em Rust, segregando estritamente as responsabilidades do motor lógico das camadas de apresentação visual:

```text
mrk-browser/
├── Cargo.toml                  # Manifesto de workspace e dependências globais
├── mrk-core/                   # Biblioteca Core (Camada de Rede e Parser)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── network.rs          # Cliente HTTP assíncrono (Reqwest + Tokio)
│       └── parser.rs           # Motor de tokenização DOM (HTML -> MDX)
├── src/                        # Orquestrador Híbrido Principal (GUI / TUI)
│   ├── main.rs                 # Ponto de entrada e despacho de Runtimes
│   ├── tauri_app.rs            # Inicialização e bindings da interface gráfica
│   └── ratatui_app.rs          # Ciclo de renderização e eventos de terminal
└── deploy/                     # Artefactos de empacotamento nativo
    ├── PKGBUILD                # Manifesto de compilação Arch/Omarchy Linux
    └── mrk-browser.desktop     # Integração com lançadores de aplicações XDG
```

## 🛠️ Modos de Operação (Mecanismo de Despacho)

O binário compilado unificado utiliza as primitivas de processamento do ecossistema Rust para alternar o chassi de execução com base em argumentos CLI de milissegundo:

### 1. Modo Gráfico (GUI - Padrão)

Se invocado sem argumentos no terminal ou disparado através do lançador de aplicações desktop, o core delega a renderização para a runtime do **Tauri**.

```bash
mrk-browser
```

- Fornece uma janela nativa leve e otimizada baseada no motor Webview do sistema operacional.
- Inclui barra de navegação superior síncrona.
- Exibe o MDX convertido utilizando injeção controlada de componentes web.

### 2. Modo Terminal (TUI)

Se invocado explicitamente com a flag `--tui`, o ciclo de vida do Tauri é suspenso antes da alocação de memória gráfica e o controle é assumido pelo painel modular do **Ratatui**.

```bash
mrk-browser --tui
```

- Renderização integral baseada em texto diretamente no terminal ativo da sua sessão.
- Interface orientada a atalhos de teclado (Keybindings otimizados para Helix/Vim).
- Conversão avançada de tags Markdown estruturadas para estilos de texto nativos (`ratatui::text::Span`), aplicando recuos lógicos e cores deduzidas do DOM.

## 🔀 Pipeline de Conversão: HTML para MDX

O coração tecnológico do `mrk-core` reside no seu fluxo assíncrono de processamento de dados (Pipeline Parsing):

1. **Ingestão Assíncrona:** A camada de rede (`network.rs`) utiliza o `reqwest` sob o runtime do `tokio` para buscar a árvore de dados crua via HTTP/HTTPS.
2. **Tokenização DOM:** O módulo `parser.rs` consome a string utilizando árvores de nós estáveis (crates `tl` ou `scraper`), isolando o corpo estrutural (`<body>`) e eliminando scripts ou tags de rastreamento maliciosas.
3. **Compilação MDX:** O parser reconstrói a árvore transformando elementos semânticos HTML em Markdown limpo estruturado, injetando suporte a blocos JSX/Componentes dinâmicos customizados inline.

## 📦 Instalação e Desinstalação (Omarchy Linux / Pacman)

O projeto está totalmente em conformidade com as diretivas de empacotamento do Arch Linux, permitindo rastreamento nativo e remoção limpa de dependências pelo gerenciador de pacotes global.

### Compilação e Instalação Local

Para gerar o pacote instalador nativo utilizando a receita contida na pasta de deploy, execute:

```bash
cd deploy
makepkg -si
```

**O que o instalador faz por baixo do capô:**

1. Invoca o compilador Rust para buildar o workspace em modo otimizado de produção (`cargo build --release`).
2. Move o binário resultante para o caminho global do sistema (`/usr/bin/mrk-browser`).
3. Copia o arquivo descritor `.desktop` para o diretório de aplicações do sistema (`/usr/share/applications/`), disponibilizando o atalho gráfico instantaneamente no menu do seu ambiente Linux.

### Desinstalação Limpa

Para remover completamente o browser e limpar todos os links simbólicos ou arquivos de atalho gerados, chame o gerenciador de pacotes:

```bash
sudo pacman -R mrk-browser
```

## 🔒 Diretrizes de Desenvolvimento Defensivo

- **Tratamento de Erros:** É terminantemente proibido o uso de macros de interrupção abrupta de runtime (`unwrap()`, `expect()`). Todos os fluxos operacionais de rede e IO de arquivos devem propagar falhas de forma limpa através da estrutura idiomática `Result<T, E>`.
- **Isolamento de Segurança:** Qualquer script de teste de integração que simule requisições HTTP complexas ou manipulação de disco deve ser executado obrigatoriamente dentro de caixas de areia (sandboxes) configuradas de forma rootless via **Podman**.