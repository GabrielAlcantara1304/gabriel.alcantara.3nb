//NIVEL 1




//NIVEL 2

struct Lista {
    dados: [i32; 10],
}

impl Lista {
    fn novo() -> Self {
        let dados: [i32; 10] = [34, 7, 23, 32, 5, 62, 31, 12, 43, 3];
        Lista { dados }
    }

    fn exibir(&self) {
        println!("{:?}", self.dados);
    }

    fn ordenar(&mut self) {
        Self::quicksort(&mut self.dados);
    }

    fn quicksort(lista: &mut [i32]) {
        let len: usize = lista.len();
        if len <= 1 {
            return;
        }

        let indice_pivo: usize = len / 2;
        let pivo: i32 = lista[indice_pivo];

        let mut elemento_esquerda: usize = 0;

        let mut elemento_direita: usize = len - 1;

        while elemento_esquerda <= elemento_direita {
            while lista[elemento_esquerda] < pivo {
                elemento_esquerda += 1;
            }
            while lista[elemento_direita] > pivo {
                elemento_direita = elemento_direita.saturating_sub(1);
            }
            if elemento_esquerda <= elemento_direita {
                lista.swap(elemento_esquerda, elemento_direita);
                elemento_esquerda += 1;
                elemento_direita = elemento_direita.saturating_sub(1);
            }
        }

        if elemento_direita > 0 {
            Self::quicksort(&mut lista[0..=elemento_direita]);
        }
        if elemento_esquerda < len {
            Self::quicksort(&mut lista[elemento_esquerda..]);
        }
    }
}

fn main() {

    //NIVEL 1
    let mut listanivel1: [i32; 10] = [34, 7, 23, 32, 5, 62, 31, 12, 43, 3];
    println!("Array original (nivel1): {:?}", listanivel1);
    listanivel1.sort();
     println!("Array ordenado (nivel1): {:?}", listanivel1);
    //FIM DO NIVEL 1


    let mut lista: Lista = Lista::novo();

    println!("Array original (nivel2):");
    lista.exibir();

    lista.ordenar();

    println!("Array ordenado (nivel2):");
    lista.exibir();
}