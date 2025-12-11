use std::collections::HashMap;

pub fn puzzle_1(input: &str) -> String {
    let device_map = handle_input(input);
    get_paths("you", "out", &device_map, &mut HashMap::new()).to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let device_map = handle_input(input);

    // Get paths from svr-->fft-->dac-->out
    let svr_to_fft = get_paths("svr", "fft", &device_map, &mut HashMap::new());
    let fft_to_dac = get_paths("fft", "dac", &device_map, &mut HashMap::new());
    let dac_to_out = get_paths("dac", "out", &device_map, &mut HashMap::new());

    // Get paths from svr-->dac-->fft-->out
    let svr_to_dac = get_paths("svr", "dac", &device_map, &mut HashMap::new());
    let dac_to_fft = get_paths("dac", "fft", &device_map, &mut HashMap::new());
    let fft_to_out = get_paths("fft", "out", &device_map, &mut HashMap::new());

    ((svr_to_fft * fft_to_dac * dac_to_out) + (svr_to_dac * dac_to_fft * fft_to_out)).to_string()
}

fn get_paths(
    from: &str,
    to: &str,
    device_map: &HashMap<&str, Vec<&str>>,
    path_counts: &mut HashMap<String, usize>,
) -> usize {
    let mut paths = 0;
    if let Some(map) = device_map.get(from) {
        for &mapping in map.iter() {
            if let Some(p) = path_counts.get(mapping) {
                paths += p;
            } else {
                if mapping == to {
                    paths += 1;
                } else if device_map.contains_key(mapping) {
                    paths += get_paths(mapping, to, device_map, path_counts);
                }
            }
        }
    }
    path_counts.insert(from.to_string(), paths);
    paths
}

fn handle_input(input: &str) -> HashMap<&str, Vec<&str>> {
    let device_mapping = input
        .lines()
        .map(|line| {
            let splits = line.split(": ").collect::<Vec<&str>>();
            let outputs: Vec<&str> = splits[1].split_whitespace().collect::<Vec<&str>>();
            (splits[0], outputs)
        })
        .collect::<Vec<(&str, Vec<&str>)>>();

    let mut device_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for (device, mappings) in device_mapping.into_iter() {
        for mapping in mappings.iter() {
            device_map
                .entry(device)
                .and_modify(|v| v.push(&mapping))
                .or_insert(vec![mapping]);
        }
    }
    device_map
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const INPUT_2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "5");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT_2), "2");
    }
}
