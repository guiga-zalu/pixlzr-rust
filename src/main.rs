use crate::pixlzr::tree_process;
use image::{/*imageops::FilterType,*/ open, DynamicImage};
// use std::f32::consts::{FRAC_PI_2};
// use std::fs;

pub mod pixlzr;

fn processar(imagem: &DynamicImage, k: f32) -> DynamicImage {
	tree_process(
		&imagem,
		128,
		k,
		Some(|v| v),
	)
}

/*
fn processar_pasta(pasta: &str){
	// Ler pasta
	let caminhos = fs::read_dir(pasta).expect("Erro ao ler a pasta!");
	// Para cada caminho na pasta
	for caminho_ in caminhos {
		let caminho = caminho_.expect("Erro ao processar o caminho!").path();
		let nome: &str = caminho.to_str().unwrap();
		println!("{nome}");
		// Lê a imagem
		let imagem: DynamicImage = open(nome).expect("Arquivo não encontrado!");
		// A processa
		processar(&imagem)
			// E a salva
			.save(nome.replacen("img/", "dst/octree/", 1))
			.expect("Erro ao salvar");
	}
}
*/

fn main() {
	// let block_size: u32 = 128;
	// processar_pasta("./img");
	let nome = "./img/blur.jpg";
	let n: u16 = 600;
	// Lê a imagem
	let imagem: DynamicImage = open(nome).expect("Arquivo não encontrado!");
	for i in 0..n {
		let k = i as f32 / n as f32;
		println!("Quadro {i} / {n} ({k} %)");
		let m = format!("0000{i}");
		let len = m.len();
		let n = &m[(len - 4)..len];
		let j = format!("./dst/video/{n}.png");
		// Clona a imagem
		let img = imagem.clone();
		// A processa
		processar(&img, k)
			// E a salva
			.save(j)
			.expect("Erro ao salvar");
	}
}
