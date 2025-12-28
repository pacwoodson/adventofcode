use std::collections::HashMap;

use crate::utils::lines_to_vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Junction {
    circuit_id: usize,
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Clone, Copy)]
struct JunctionDistance {
    junction_a_id: usize,
    junction_b_id: usize,
    distance: f64,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Circuit {
    junctions_ids: Vec<usize>,
}

fn parse_input(input: &String) -> (HashMap<usize, Junction>, HashMap<usize, Circuit>) {
    let junctions: HashMap<usize, Junction> = lines_to_vec(input)
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let positions: Vec<i64> = line.split(',').map(|p| p.parse().unwrap()).collect();
            (
                i,
                Junction {
                    circuit_id: i,
                    x: positions[0],
                    y: positions[1],
                    z: positions[2],
                },
            )
        })
        .collect();

    let circuits: HashMap<usize, Circuit> = junctions
        .keys()
        .map(|i| {
            (
                *i,
                Circuit {
                    junctions_ids: vec![*i],
                },
            )
        })
        .collect();

    (junctions, circuits)
}

fn distance(a: &Junction, b: &Junction) -> f64 {
    (((b.x - a.x).pow(2) + (b.y - a.y).pow(2) + (b.z - a.z).pow(2)) as f64).sqrt()
}

fn move_junction_into_circuit(
    junctions: &mut HashMap<usize, Junction>,
    circuits: &mut HashMap<usize, Circuit>,
    junction_id_to_move: usize,
    target_circuit_id: usize,
    source_circuit_id: usize,
) {
    let junction = junctions.get_mut(&junction_id_to_move).unwrap();
    junction.circuit_id = target_circuit_id;

    let target_circuit = circuits.get_mut(&target_circuit_id).unwrap();
    target_circuit.junctions_ids.push(junction_id_to_move);

    let source_circuit = circuits.get_mut(&source_circuit_id).unwrap();
    source_circuit
        .junctions_ids
        .retain(|&id| id != junction_id_to_move);

    if source_circuit.junctions_ids.is_empty() {
        circuits.remove(&source_circuit_id);
    }
}

fn connect_junctions(
    junctions: &mut HashMap<usize, Junction>,
    circuits: &mut HashMap<usize, Circuit>,
    junction_distance: &JunctionDistance,
) {
    let circuit_id_a = junctions
        .get(&junction_distance.junction_a_id)
        .unwrap()
        .circuit_id;
    let circuit_id_b = junctions
        .get(&junction_distance.junction_b_id)
        .unwrap()
        .circuit_id;

    if circuit_id_a == circuit_id_b {
        return;
    }

    let junctions_ids_to_move = circuits.get(&circuit_id_b).unwrap().junctions_ids.clone();
    for junction_id_to_move in junctions_ids_to_move {
        move_junction_into_circuit(
            junctions,
            circuits,
            junction_id_to_move,
            circuit_id_a,
            circuit_id_b,
        );
    }
}

pub fn solve(input: &String, test: bool) -> (String, String) {
    let (mut junctions, mut circuits) = parse_input(&input);

    let mut junction_distances: Vec<JunctionDistance> = vec![];
    for (j_id_a, junction_a) in junctions.iter() {
        for (j_id_b, junction_b) in junctions.iter() {
            if *j_id_a <= *j_id_b {
                continue;
            }
            junction_distances.push(JunctionDistance {
                junction_a_id: *j_id_a,
                junction_b_id: *j_id_b,
                distance: distance(junction_a, junction_b),
            })
        }
    }
    junction_distances.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    let n_junctions = if test { 10 } else { 1000 };
    for junction_distance in junction_distances[0..n_junctions].iter() {
        connect_junctions(&mut junctions, &mut circuits, junction_distance);

        // println!(
        //     "distances_[{}]: {} {} {}",
        //     i,
        //     junction_distances[i].j_a_id,
        //     junction_distances[i].j_b_id,
        //     junction_distances[i].distance,
        // );
    }

    // println!("n distances: {}", junction_distances.len());
    // println!("n circuits: {}", circuits_sorted.len());
    // for circuit in circuits_sorted.iter() {
    //     print!("circuit {} js ", circuit.junctions_ids.len());
    //     for junction_id in circuit.junctions_ids.iter() {
    //         print!("{} ", junction_id);
    //     }
    //     println!();
    // }

    let mut circuits_sorted: Vec<Circuit> = circuits
        .iter()
        .map(|(_, c)| c.clone())
        .collect::<Vec<Circuit>>();
    circuits_sorted.sort_by(|a, b| a.junctions_ids.len().cmp(&(b.junctions_ids.len())));
    circuits_sorted.reverse();

    let res_pt1 = circuits_sorted[0..3]
        .iter()
        .map(|s| s.junctions_ids.len())
        .fold(1, |acc, s| acc * s);

    let mut res_pt2 = 0;

    for junction_distance in junction_distances[n_junctions..junction_distances.len()].iter() {
        connect_junctions(&mut junctions, &mut circuits, junction_distance);

        let alone_circuits: u32 = circuits
            .iter()
            .filter_map(|(_, c)| {
                if c.junctions_ids.len() == 1 {
                    Some(1)
                } else {
                    None
                }
            })
            .sum();

        if alone_circuits == 0 {
            let junction_a = junctions.get(&junction_distance.junction_a_id).unwrap();
            let junction_b = junctions.get(&junction_distance.junction_b_id).unwrap();
            res_pt2 = junction_a.x * junction_b.x;
            break;
        }
    }

    (res_pt1.to_string(), res_pt2.to_string())
}
