# Especification of the PIXLZR' codification

- [Especification of the PIXLZR' codification](#especification-of-the-pixlzr-codification)
  - [History](#history)
  - [Especification](#especification)

## History

Friday, `10` of February of `2023` (`10/02/2023`), my second teoric FeMec class in the year.  
As I had a little of free time, I drafted an specification:  

![.](.)

Decidi, no momento, terceirizar a codificação do bloco para o modelo QOI (Quite Ok Image Format / Formato de Imagem Bem Ok).

Na semana seguinte, investiguei que necessitaria realizar uma estruturação baseada em objetos para efetivamente criar uma API decente.  
Do sábado `18/02` a terça-feira `21/02`, consegui implementar com sucesso a especificação.

Na manhã do dia seguinte (Quarta-feira de Cinzas, `22/02`), pesquisei por um visualizador de imagens aberto feito em Rust, e cheguei ao [Emulsion](.).

Em cerca de cinco minutos explorando o GitHub do projeto, descobri quais linhas deveria alterar (de duas funções em `src/image_cache/image_loader.rs`, `detect_format` e `simple_load_image`).  
Após algumas horas de trabalho, incluindo alterações na `crate image` (que resultarão em uma ramificação da mesma, porém inclusa na ramificação do *emulsion*), tive sucesso.

## Especification

| # | Size (oc) | Accumulated (oc) | Group | Element | Value |
| - | --------- | ---------------- | :---: | :------ | ----- |
| 0 | 6 | 6 | Header | Magic Numbers| `b"PIXLZR"` |
| 1 | 3 | 9 | Header | Version | `0`, `0`, `1` |
| 2 | 4 | 13 | Header | Width | `u32` |
| 3 | 4 | 17 | Header | Height | `u32` |
| 4 | 4 | 21 | Header | Block's Width | `u32` |
| 5 | 4 | 25 | Header | Block's Height | `u32` |
| 6 | ? | 6 + ? | Header |  | `[u32; ..]` |
| . | . | .. | .. | .. |
| n | 5 | n + 5 | Block | Magic Numbers | `b"block"` |
| n | 4 | n + 9 | Block | Value of the block | `f32` |
| n | 4 | n + 13 | Block | Size of the QOI block | `u32 =: m` |
| n | m | n + m | Block | QOI Block |

```txt
PIXLZR := HEADER + BLOCKS

HEADER := PIXLZR_MAGIC_NUMBERS + PIXLZR_VERSION + WIDTH + HEIGHT + BLOCK_WIDTH + BLOCK_HEIGHT + LINE_LENGTHS

PIXLZR_MAGIC_NUMBERS = b"PIXLZR"
PIXLZR_VERSION = [0, 0, 1]
WIDTH, HEIGHT, BLOCK_WIDTH, BLOCK_HEIGHT := unsigned int of 4 octets
LINE_LENGTHS := array do unsigned ints of 4 octets each; size is the vertical amount of blocks

BLOCKS := BLOCK[]

BLOCK := PIXLZR_BLOCK_MAGIC_NUMBERS + BLOCK_VALUE + QOI_IMAGE_LENGTH + QOI_IMAGE

PIXLZR_BLOCK_MAGIC_NUMBERS = b"block"
BLOCK_VALUE := float of 4 octets
QOI_IMAGE_LENGTH := unsigned int of 4 octets
QOI_IMAGE := list of octets
```

And, finally, the most recent version is:

```txt
PIXLZR :=
    HEADER :=
        b"PIXLZR"
        [0, 0, 1]
        WIDTH
        HEIGHT
        BLOCK_WIDTH
        BLOCK_HEIGHT
        LINE_LENGTHS
    BLOCKS := [
        b"block"
        BLOCK_VALUE
        QOI_IMAGE_LENGTH
        QOI_IMAGE
    ]
```

`LINE_LENGTHS` is used to encode and decode in parallel.
