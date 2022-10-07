# Max Value

## Resumo

Descrição:

- $p_{i, j}$: píxel na posição $i, j$
- $A, L$: altura e largura da imagem
- $M$: valor máximo para um subpíxel
- $\bar p$: média dos valores dos píxeis
- $\delta_{i, j}$: diferença média
- $\int\delta$: soma das diferenças

O valor máximo para $\in\delta$ ocorre quando* $p_{i, j} = M$ para metade dos valores de $i, j$, e $0$ para a outra metade.  
Então $\bar p = {M\over 2}$ e $\delta_{i, j} = {M\over 2}$.  
Logo, $\int\delta = A\cdot L\times \delta_{i, j} = {A\cdot L\cdot M\over 2}$.
