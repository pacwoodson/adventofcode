use std::collections::{HashMap, HashSet};

use crate::utils::lines_to_vec;

type ID = [u8; 3];
type Connections = HashMap<ID, Vec<ID>>;

pub fn solve(input: &String, _: bool) -> (String, String) {
    let connections: Connections = lines_to_vec(input)
        .iter()
        .map(|row| {
            let from = parse_id(&row[0..3]);
            let tos: Vec<ID> = row[5..row.len()].split(" ").map(|t| parse_id(t)).collect();
            (from, tos)
        })
        .collect();
    let reverse_connections = reverse_connection(&connections);

    let n_paths_part_1 = paths(&connections, &vec![parse_id("you")], &parse_id("out"), None);

    let can_reach_out = precomp_accessibility(&parse_id("out"), &reverse_connections);
    let can_reach_fft = precomp_accessibility(&parse_id("fft"), &reverse_connections);
    let can_reach_dac = precomp_accessibility(&parse_id("dac"), &reverse_connections);

    let n_paths_part_2 = paths_with_pruning(
        &connections,
        &can_reach_out,
        &can_reach_fft,
        &can_reach_dac,
        &vec![parse_id("svr")],
        &parse_id("out"),
    );

    (n_paths_part_1.to_string(), n_paths_part_2.to_string())
}

fn reverse_connection(connections: &Connections) -> Connections {
    let mut reverse_connections: Connections = Connections::new();

    for (device, connected_devices) in connections {
        for connected_device in connected_devices {
            if !reverse_connections.contains_key(connected_device) {
                reverse_connections.insert(*connected_device, vec![]);
            }
            let conns = reverse_connections.get_mut(connected_device).unwrap();
            conns.push(*device);
        }
    }

    reverse_connections
}

fn parse_id(s: &str) -> ID {
    let bytes = s.as_bytes();
    [bytes[0], bytes[1], bytes[2]]
}
fn id_str(id: &ID) -> &str {
    std::str::from_utf8(id).unwrap()
}

fn print_path(path: &Vec<ID>) {
    for p in path {
        print!("{}, ", std::str::from_utf8(p).unwrap());
    }
    println!();
}

fn paths(connections: &Connections, path: &Vec<ID>, to: &ID, req: Option<&Vec<ID>>) -> u32 {
    let mut result: u32 = 0;

    let from = path[path.len() - 1];
    let connected_devices = connections.get(&from);

    if let Some(c) = connected_devices {
        for device in c {
            // dbg!(id_str(device));
            if device == to {
                if let Some(required_devices) = req {
                    let mut ok = true;
                    for r_device in required_devices {
                        if !path.contains(r_device) {
                            ok = false;
                            break;
                        }
                    }
                    if ok {
                        // print_path(path);
                        result += 1;
                    }
                } else {
                    // print_path(path);
                    result += 1;
                }
            } else {
                let mut next_path = path.clone();
                next_path.push(device.clone());
                // print_path(path);
                // let set: HashSet<ID> = HashSet::from_iter(next_path.clone());
                // if set.len() != next_path.len() {
                //     println!("noooo");
                // }
                result += paths(connections, &next_path, to, req);
            }
        }
    } else {
        print_path(path);
        println!("Found device without connected devices {}", id_str(&from));
    }

    result
}

fn precomp_accessibility(target: &ID, reverse_connections: &Connections) -> HashSet<ID> {
    let mut accesses: HashSet<ID> = HashSet::new();
    let mut stack: Vec<ID> = vec![*target];

    while let Some(node) = stack.pop() {
        if let Some(predecessors) = reverse_connections.get(&node) {
            for predecessor in predecessors {
                if !accesses.contains(predecessor) {
                    accesses.insert(*predecessor);
                    stack.push(*predecessor);
                }
            }
        }
    }

    accesses
}

fn paths_with_pruning(
    connections: &Connections,
    can_reach_out: &HashSet<ID>,
    can_reach_fft: &HashSet<ID>,
    can_reach_dac: &HashSet<ID>,
    path: &Vec<ID>,
    to: &ID,
) -> u32 {
    let mut result: u32 = 0;

    let dac_id = parse_id("dac");
    let fft_id = parse_id("fft");

    let current = path[path.len() - 1];
    let connected_devices = connections.get(&current);

    print_path(path);

    if let Some(devices) = connected_devices {
        for device in devices {
            if device == to {
                if path.contains(&dac_id) && path.contains(&fft_id) {
                    result += 1;
                }
            } else {
                if !can_reach_out.contains(device) {
                    continue;
                }

                let has_dac = path.contains(&dac_id) || *device == dac_id;
                if !has_dac && !can_reach_dac.contains(device) {
                    continue;
                }

                let has_fft = path.contains(&fft_id) || *device == fft_id;
                if !has_fft && !can_reach_fft.contains(device) {
                    continue;
                }

                let mut next_path = path.clone();
                next_path.push(*device);
                result += paths_with_pruning(
                    connections,
                    can_reach_out,
                    can_reach_fft,
                    can_reach_dac,
                    &next_path,
                    to,
                );
            }
        }
    }

    result
}

fn traverse_reverse_o(
    connections: &Connections,
    path: &Vec<ID>,
    from: &ID,
    to: &ID,
    req: Option<&Vec<ID>>,
) -> u32 {
    let mut result: u32 = 0;

    for (device, connected_devices) in connections.iter() {
        if connected_devices.contains(to) {
            if device == from {
                if let Some(required_devices) = req {
                    let mut ok = true;
                    for r_device in required_devices {
                        if !path.contains(r_device) {
                            ok = false;
                            break;
                        }
                    }
                    if ok {
                        print_path(path);
                        result += 1;
                    }
                } else {
                    print_path(path);
                    result += 1;
                }
            } else {
                let mut next_path = path.clone();
                next_path.push(device.clone());
                // print_path(path);
                // let set: HashSet<ID> = HashSet::from_iter(next_path.clone());
                // if set.len() != next_path.len() {
                //     println!("noooo");
                // }
                result += traverse_reverse_o(connections, &next_path, from, device, req);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path() {
        let connections: Connections = HashMap::from([
            (parse_id("aaa"), vec![parse_id("bbb"), parse_id("ccc")]),
            (parse_id("bbb"), vec![parse_id("ccc")]),
            (parse_id("ccc"), vec![parse_id("ddd")]),
        ]);

        assert_eq!(
            paths(&connections, &vec![parse_id("aaa")], &parse_id("ddd"), None),
            2
        );
    }
    #[test]
    fn test_reverse() {
        let connections: Connections = HashMap::from([
            (parse_id("aaa"), vec![parse_id("bbb"), parse_id("ccc")]),
            (parse_id("bbb"), vec![parse_id("ccc")]),
            (parse_id("ccc"), vec![parse_id("ddd")]),
        ]);

        assert_eq!(
            reverse_connection(&connections),
            HashMap::from([
                (parse_id("ccc"), vec![parse_id("aaa"), parse_id("bbb")]),
                (parse_id("bbb"), vec![parse_id("aaa")]),
                (parse_id("ddd"), vec![parse_id("ccc")]),
            ])
        );
    }
    #[test]
    fn test_accessibility() {
        let connections: Connections = HashMap::from([
            (parse_id("aaa"), vec![parse_id("bbb"), parse_id("ccc")]),
            (parse_id("bbb"), vec![parse_id("ccc")]),
            (parse_id("ccc"), vec![parse_id("ddd")]),
        ]);

        assert_eq!(
            precomp_accessibility(&parse_id("ddd"), &reverse_connection(&connections)),
            HashSet::from([parse_id("aaa"), parse_id("bbb"), parse_id("ccc")])
        );
        assert_eq!(
            precomp_accessibility(&parse_id("ccc"), &reverse_connection(&connections)),
            HashSet::from([parse_id("aaa"), parse_id("bbb")])
        );
        assert_eq!(
            precomp_accessibility(&parse_id("bbb"), &reverse_connection(&connections)),
            HashSet::from([parse_id("aaa")])
        );
        assert_eq!(
            precomp_accessibility(&parse_id("aaa"), &reverse_connection(&connections)),
            HashSet::from([])
        );
    }
}
