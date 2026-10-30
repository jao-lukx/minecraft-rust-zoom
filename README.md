# Zoom Rust — 0.2.0

Zoom suave com C, roda para ajustar e sensibilidade proporcional.

Agora é um arquivo **`.hnmod` pronto para colocar na pasta de mods**. Não use os instaladores manuais de fonte da versão 0.1.0.

## Instalação

1. Atualize uma vez para o [launcher offline com mods na pasta](https://github.com/jao-lukx/minecraft-rust-offline/blob/main/MODS-NA-PASTA.md). O launcher antigo não faz essa detecção automática.
2. [Baixe `rust_zoom-0.2.0.hnmod`](https://raw.githubusercontent.com/jao-lukx/minecraft-rust-zoom/main/downloads/rust_zoom-0.2.0.hnmod). **Não descompacte o arquivo.**
3. Coloque-o em `mods/`, ao lado do `horizon-launcher`, e clique em **Jogar**. Também é aceita `mods/` ou `horizon-native-mods/` da instância.

A primeira combinação de mods é compilada automaticamente pelo launcher, que baixa seu próprio compilador quando necessário. Nas próximas aberturas, reutiliza o executável. Adicionar/remover/atualizar mods escolhe uma compilação própria; desligar em **Opções → Mods** mantém o código inativo e permite ligá-lo de novo sem compilar. Mundos e perfis ficam na mesma instância.

Requer **Horizon Offline Drop-in, Minecraft 26.3**, com recursos preparados do próprio usuário. Os pacotes são de código nativo `.hnmod`: contêm Rust, e o executável resultante é guardado pelo launcher; não são bibliotecas Java ou plugins binários universais. Combinações fornecidas com um cliente pré-compilado correspondente podem usá-lo diretamente.

## Controles

Segure **C** para zoom suave; ajuste com a roda. A sensibilidade acompanha a ampliação. A tecla pode ser alterada na página do mod. A luneta tem prioridade.

## Desenvolvimento e validação

```sh
cargo test
cargo clippy --all-targets -- -D warnings
python3 tools/package.py
```

O pacote é gerado em `target/`. `native/` contém os módulos e ganchos do mod. A compatibilidade é fixada ao snapshot offline; o launcher atualizado fornece esse código, preservando perfis offline. [Resultados e limites dos testes](VALIDATION.md). Implementação independente, não afiliada ao mod Java original.
