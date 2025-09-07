use rand::Rng;

const BLOCKS: usize = 5;
const MAX_WEIGTH: f32 = 16.0;
const MAX_POPULATION: usize = 5;
const MUTATE_PROBABILITY: f32 = 0.8;
const GENERATION_RESTRICTION: usize = 8;


#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct Element {
    weight: f32,
    benefit: f32,
}

#[derive(Debug, Clone, Copy)]
struct Score {
    index: usize,
    total_benefit: f32,
    probability: f32,
}

// operador de cruce
fn select_parents(scores: &Vec<Score>) -> (usize, usize) { // scores is sorted and has almost 2 elements
    if scores.len() < 2 { panic!("Too few scores"); }

    let mut parents: (Option<usize>, Option<usize>) = (None, None);
    let mut was_selected: Vec<usize> = Vec::new();
    for score in scores.iter() {
        if was_selected.contains(&score.index) { continue }
        if probability(score.probability) {
            if parents.0.is_none() {
                parents.0 = Some(score.index);
                was_selected.push(score.index);
            } else if parents.1.is_none() {
                parents.1 = Some(score.index);
                was_selected.push(score.index);
            }
        }
    }

    match parents { //
        (Some(a), Some(b)) => { (a, b) }
        (None, None) => { (scores[0].index, scores[1].index) }
        (Some(a), None) => { (a, pick_unselected(&scores, &was_selected))}
        (None, Some(b)) => { (b, pick_unselected(&scores, &was_selected))}
    }
}

fn pick_unselected(scores: &Vec<Score>, selected_list: &Vec<usize>) -> usize {
    for s in scores.iter() {
        if !selected_list.contains(&s.index) { return s.index; }
    }
    panic!("Selected list not found");
}

fn mix(c1: &Vec<bool>, c2: &Vec<bool>) -> Vec<bool> { // mezclar homogéneamente
    c1.iter().enumerate().zip(c2.iter()).map(|((i, a), b)| {
        if i % 2 == 0 { *a } else {*b}
    }).collect()
  }

// Operadores de mutación
fn mutate(chromosome: &mut Vec<bool>) -> bool{
    if !probability(MUTATE_PROBABILITY) { return false; }
    let i = rand::random_range(0..BLOCKS);
    chromosome[i] = !chromosome[i];
    true
}

// helpers
fn random_chromosome() -> Vec<bool> {
    let mut rng = rand::rng();
    let mut chromosome: Vec<bool> = Vec::new();
    for _ in 0..BLOCKS {
        chromosome.push(rng.random_bool(0.3))
    }
    chromosome
}

fn selected_elements(chromosome: &Vec<bool>, blocks: &Vec<Element>) -> Vec<Element> {
    let mut sliced_elements: Vec<Element> = Vec::new();
    for (i, e) in chromosome.iter().enumerate() {
        if *e { sliced_elements.push(blocks[i].clone()); }
    }
    sliced_elements
}

fn total_weight(blocks: &Vec<Element>) -> f32 {
    blocks.iter().map(|e| e.weight).sum()
}

fn total_benefits(blocks: &Vec<Element>) -> f32 {
    blocks.iter().map(|e| e.benefit).sum()
}

fn probability(p: f32) -> bool {
    rand::rng().random_bool(p as f64)
}

fn scores(chromosomes: &[Vec<bool>; 5], blocks: &Vec<Element>, max_possible_benefit: f32) -> Vec<Score> {
    let mut scores: Vec<_> = chromosomes.iter().enumerate().map(|(index, c)| {
        let benefit = calculate_score(c, &blocks);
        Score {
            index,
            total_benefit: benefit,
            probability: benefit / max_possible_benefit
        }
    }).collect();
    scores.sort_by(|a, b| b.total_benefit.partial_cmp(&a.total_benefit).unwrap());
    scores
}
fn calculate_score(chromosome: &Vec<bool>, blocks: &Vec<Element>) -> f32 {
    let elements = selected_elements(chromosome, blocks);
    if total_weight(&elements) > MAX_WEIGTH {
        return 0.0;
    }
    total_benefits(&elements)
}

fn main() {
    // bloques a llevar en la mochila ( ! debe ser igual a la constante BLOCKS ! )
    let blocks: Vec<Element> = vec![
        Element { weight: 12.0, benefit: 4.0 },
        Element { weight: 2.0, benefit: 2.0 },
        Element { weight: 1.0, benefit: 2.0 },
        Element { weight: 1.0, benefit: 1.0 },
        Element { weight: 4.0, benefit: 10.0 },
    ];

    // verify constants
    if MAX_POPULATION < 2 { panic!("Population is too small. Two or more needed") } // mox_population > 2
    // max_blocks
    if blocks.len() != BLOCKS { panic!("Blocks size don't match with the constant BLOCKS") }

    // obtener el máximo beneficio posible
    let max_possible_benefit: f32 = total_benefits(&blocks);

    // generar cromosomas
    println!("[+] Generating random chromosome...");
    let mut chromosomes: [Vec<bool>; MAX_POPULATION] = [const { Vec::new() }; MAX_POPULATION];
    for i in 0..MAX_POPULATION {
        chromosomes[i] = random_chromosome();
    }
    println!("[+] Initial population of {} chromosomes", chromosomes.len());
    chromosomes.iter().enumerate().for_each(|(i, c)| {
        println!("\tChromosome {}: {:?}", i, c);
    });

    // bucle de vida
    for g in 0..GENERATION_RESTRICTION {
        println!("\n<====== [ GENERACIÓN {} ] ======>", g);

        // obtener mejores individuos
        let mut scores: Vec<Score> = scores(&chromosomes, &blocks, max_possible_benefit);
        //println!("[+] Scores: {:?}", scores);

        // ordenar por beneficio y seleccionar con base en una probabilidad (o los mejores, en caso de que no caiga ninguna probabilidad)
        let parents = select_parents(&scores);
        //println!("[+] Selected parents: {:?}", parents);

        // cruzarlos dos vectores
        let mut child = mix(&chromosomes[parents.0], &chromosomes[parents.1]);
        println!("[+] Mixed Chromosome \n\t{:?} +\n\t{:?} +\n\t{:?} =", chromosomes[parents.0], chromosomes[parents.1], child);

        // mutar los el hijo
        if mutate(&mut child) {
            println!("[!] Mutated Chromosome: {:?}", child);
        }

        // Remplazar el nuevo con el de menor beneficio de la población (Operador de remplazo)
        let last_chromosome_index = scores[scores.len()-1].index;
        chromosomes[last_chromosome_index] = child;
    }

    println!("\n[+] Final population of {} chromosomes", chromosomes.len());
    chromosomes.iter().enumerate().for_each(|(i, c)| {
        let score = calculate_score(c, &blocks);
        println!("\tChromosome {}: {:?} benefit: {}", i, c, score);
    });

    let best_chromosome_index = scores(&chromosomes, &blocks, max_possible_benefit)[0].index;
    let best_chromosome = &chromosomes[best_chromosome_index].clone();
    let mut blocks_chromosome: Vec<Element> = Vec::new();
    best_chromosome.iter().enumerate().for_each(|(i, c)| {
        if *c {
            blocks_chromosome.push(blocks[i].clone())
        }
    });
    println!("[+] Mejor combinación encontrada en {} generaciones: {:?}", GENERATION_RESTRICTION-1, blocks_chromosome);
}