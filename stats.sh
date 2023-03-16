original=$(stat -c "%b" img/antelope-canyon.jpg)
pix_pix=$(stat -c "%b" teste.pixlzr)
pix_png=$(stat -c "%b" image.png)

porcentagem_pix=$[$pix_pix * 100 / $original]
porcentagem_png=$[$pix_png * 100 / $original]

echo "Original: $original"
echo ".pix: $pix_pix ($porcentagem_pix %)"
echo ".png: $pix_png ($porcentagem_png %)"
