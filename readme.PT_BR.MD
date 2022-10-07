Um algoritmo de compressão de imagens com perda, baseado em detecção de nível de detalhe por blocos, implementado em Rust.

# pixzlr

[![Vezes baixado](https://img.shields.io/crates/d/pixlzr?logo=rust&style=for-the-badge&label=Vezes%20baixado)](https://crates.io/crates/pixlzr)
[![Vezes baixado (recentemente)](https://img.shields.io/crates/dr/pixlzr?label=Vezes%20baixado%20%28recentemente%29&logo=rust&style=for-the-badge)](https://crates.io/crates/pixlzr)
[![Versão do pacote](https://img.shields.io/crates/v/pixlzr?label=Vers%C3%A3o%20do%20pacote&logo=rust&style=for-the-badge)](https://crates.io/crates/pixlzr)

![Última atualização no GitHub](https://img.shields.io/github/last-commit/guiga-zalu/pixlzr-rust?label=%C3%9Altima%20atualiza%C3%A7%C3%A3o%20no%20GitHub&logo=github&style=flat-square)

## Instalando

Apenas coloque `pixlzr = "0"` nas dependências (`[dependencies]`) em seu `Cargo.toml`.

## Utilização

```rust
use image::{open, DynamicImage};
use pixlzr::{process, tree_process, tree_full_process};

// ...

let image: DynamicImage = open("img.png").expect("Experava uma imagem!");

process(&image, 64, Some(|v| v / 4.0))
    .save("img-processada.png")
    .expect("Erro ao salvar a imagem processada!");

tree_process(&image, 64, 0.25, Some(|v| v / 6.0))
    .save("img-processada-tree.png")
    .expect("Erro ao salvar a imagem processada!");

tree_full_process(&image, 0.25, Some(|v| v / 6.0))
    .save("img-processada-tree-full.png")
    .expect("Erro ao salvar a imagem processada!");
```

## Conceito principal

Ainda a ser escrito...  
Favor conferir [GitHub:guiga-zalu/smart-pixelizer](https://github.com/guiga-zalu/smart-pixelizer), já que é a implementação em Node JS.
