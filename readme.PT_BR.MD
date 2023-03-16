Um algoritmo de compressão de imagens com perda, baseado em detecção de nível de detalhe por blocos, implementado em Rust.

# pixzlr

[![Vezes baixado](https://img.shields.io/crates/d/pixlzr?logo=rust&style=for-the-badge&label=Vezes%20baixado)](https://crates.io/crates/pixlzr)
[![Vezes baixado (recentemente)](https://img.shields.io/crates/dr/pixlzr?label=Vezes%20baixado%20%28recentemente%29&logo=rust&style=for-the-badge)](https://crates.io/crates/pixlzr)
[![Versão do pacote](https://img.shields.io/crates/v/pixlzr?label=Vers%C3%A3o%20do%20pacote&logo=rust&style=for-the-badge)](https://crates.io/crates/pixlzr)

![Última atualização no GitHub](https://img.shields.io/github/last-commit/guiga-zalu/pixlzr-rust?label=%C3%9Altima%20atualiza%C3%A7%C3%A3o%20no%20GitHub&logo=github&style=flat-square)

## Instalando

Apenas coloque `pixlzr = "0"` nas dependências (`[dependencies]`) em seu `Cargo.toml`.

## Utilização

### Antes de usar

Agora, a `caixa pixlzr` tem duas APIs paralelas, com diferentes usos e funcionalidades.

O uso básico, de reduzir / encolher um elemento `image::DynamicImage`, é presente em ambos. Mas no futuro, a API antiga será ou apagada ou reescrita baseada na nova, que suporta salvar / ler arquivos.

### Usando a API antiga

```rust
use image::{open, DynamicImage};
use pixlzr::{process, tree_process, tree_full_process};

// ...

let image: DynamicImage = open("img.png")?;

process(&image, 64, Some(|v| v / 4.0))
    .save("img-processada.png")?;

tree_process(&image, 64, 0.25, Some(|v| v / 6.0))
    .save("img-processada-tree.png")?;

tree_full_process(&image, 0.25, Some(|v| v / 6.0))
    .save("img-processada-tree-full.png")?;
```

### Usando a nova API

```rust
// Importação
use image::{open, DynamicImage};
use pixlzr::{FilterType, Pixlzr};
```

```rust
// Converter para
let png: DynamicImage = open("img.png");

let mut pix = Pixlzr::from_image(&img, 64, 64u32);
pix.shrink_by(FilterType::Gaussian, 0.8);

pix.save("imagem pix-lizada.pixlzr")?;
```

```rust
// Converter de volta
let pix = Pixlzr::open("imagem pix-lizada.pixlzr")?;
let img = pix.to_image(FilterType::Nearest)?;
img.save("imagem-reduzida.png");
```

## CLI

A <abbr title="Command Line Interface / Interface de Linha de Comando">CLI</abbr> pode ser entendida com um simples uso de `pixlzr -h`.

```txt
Pixlzr - A rust lib and CLI for the pixlzr image format

Usage: pixlzr [OPTIONS] --input <INPUT> --output <OUTPUT>

Options:
  -i, --input <INPUT>                The input image file
  -o, --output <OUTPUT>              The output image file
  -b, --block-width <BLOCK_WIDTH>    The width of each block [default: 64]
      --block-height <BLOCK_HEIGHT>  The height of each block
  -k, --shrink-by <SHRINK_BY>        The shrinking factor [default: 1]
      --filter <FILTER>              The filter used when resizing the image blocks [default: lanczos3] [possible values: nearest, triangle, catmull-rom, gaussian, lanczos3]
  -h, --help                         Print help (see more with '--help')
  -V, --version                      Print version
```

O programa converte _de_ e _para_ o formato `pixlzr`, com uso da `caixa image`.

## Conceito principal

Ainda a ser escrito...  
Favor conferir [GitHub:guiga-zalu/smart-pixelizer](https://github.com/guiga-zalu/smart-pixelizer), já que é a implementação em Node JS.
