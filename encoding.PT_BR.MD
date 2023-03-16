# Especificação de codificação PIXLZR

- [Especificação de codificação PIXLZR](#especificação-de-codificação-pixlzr)
  - [História](#história)
  - [Especificação](#especificação)

## História

Sexta-feira, dia `10/02/2023`, minha segunda aula teórica de FeMec no ano.  
Eu tinha um tempinho livre, então rabisquei uma especificação:  

![.](.)

Decidi, no momento, terceirizar a codificação do bloco para o modelo QOI (Quite Ok Image Format / Formato de Imagem Bem Ok).

Na semana seguinte, investiguei que necessitaria realizar uma estruturação baseada em objetos para efetivamente criar uma API decente.  
Do sábado `18/02` a terça-feira `21/02`, consegui implementar com sucesso a especificação.

Na manhã do dia seguinte (Quarta-feira de Cinzas, `22/02`), pesquisei por um visualizador de imagens aberto feito em Rust, e cheguei ao [Emulsion](.).

Em cerca de cinco minutos explorando o GitHub do projeto, descobri quais linhas deveria alterar (de duas funções em `src/image_cache/image_loader.rs`, `detect_format` e `simple_load_image`).  
Após algumas horas de trabalho, incluindo alterações na `crate image` (que resultarão em uma ramificação da mesma, porém inclusa na ramificação do *emulsion*), tive sucesso.

## Especificação

| # | Tamanho (oc) | Acumulado (oc) | Grupo | Elemento | Valor |
| - | ------------ | -------------- | :---: | :------- | ----- |
| 0 | 6 | 6 | Cabeçalho | Números Mágicos| `b"PIXLZR"` |
| 1 | 3 | 9 | Cabeçalho | Versão | `0`, `0`, `1` |
| 2 | 4 | 13 | Cabeçalho | Largura | `u32` |
| 3 | 4 | 17 | Cabeçalho | Altura | `u32` |
| 4 | 4 | 21 | Cabeçalho | Largura de Bloco | `u32` |
| 5 | 4 | 25 | Cabeçalho | Altura de Bloco | `u32` |
| . | . | .. | .. | .. |
| n | 5 | n + 5 | Bloco | Números Mágicos | `b"block"` |
| n | 4 | n + 9 | Bloco | Valor do bloco | `f32` |
| n | 4 | n + 13 | Bloco | Tamanho do bloco QOI | `u32` |
| n | m | n + m | Bloco | Bloco QOI |

```txt
PIXLZR := HEADER + BLOCKS

HEADER := PIXLZR_MAGIC_NUMBERS + PIXLZR_VERSION + WIDTH + HEIGHT + BLOCK_WIDTH + BLOCK_HEIGHT

PIXLZR_MAGIC_NUMBERS = b"PIXLZR"
PIXLZR_VERSION = [0, 0, 1]
WIDTH, HEIGHT, BLOCK_WIDTH, BLOCK_HEIGHT := inteiro sem sinal de 4 octetos

BLOCKS := BLOCK[]

BLOCK := PIXLZR_BLOCK_MAGIC_NUMBERS + BLOCK_VALUE + QOI_IMAGE_LENGTH + QOI_IMAGE

PIXLZR_BLOCK_MAGIC_NUMBERS = b"block"
BLOCK_VALUE := flutuante de 4 octetos
QOI_IMAGE_LENGTH := inteiro sem sinal de 4 octetos
QOI_IMAGE := lista de octetos
```

E, finalmente:

```txt
PIXLZR :=
    HEADER :=
        b"PIXLZR"
        [0, 0, 1]
        WIDTH
        HEIGHT
        BLOCK_WIDTH
        BLOCK_HEIGHT
    BLOCKS := [
        b"block"
        BLOCK_VALUE
        QOI_IMAGE_LENGTH
        QOI_IMAGE
    ]
```
