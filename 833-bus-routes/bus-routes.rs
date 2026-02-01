use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
// impl Solution {
//     pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
//         // vanilla BFS
//         let mut adj: HashMap<i32, HashSet<i32>> = HashMap::new();
//         for rt in routes {
//             for i in 1..rt.len() {
//                 let (from, to) = (rt[i-1], rt[i]);
//                 adj.entry(from).or_insert(HashSet::new()).insert(to);
//             }
//             if rt.len() > 1{
//                 adj.entry(rt[rt.len() - 1]).or_insert(HashSet::new()).insert(rt[0]);
//             }
//         }
//         println!("{:?}", adj);
//         let mut visited = HashSet::from([source]);
//         let mut q = VecDeque::from([source]);
//         let mut hops = 0;
//         while q.len() > 0 {
//             let mut sz = q.len();
//             while sz > 0{
//                 hops += 1;
//                 sz -= 1;
//                 let node = q.pop_front().unwrap();
//                 if let Some(neighbors) = adj.get(&node) {
//                     for next in neighbors {
//                         if *next == target {
//                             return hops;
//                         }
//                         if !visited.contains(next){
//                             q.push_back(*next);
//                             visited.insert(*next);
//                         }
//                     }
//                 }

//             }
//         }

//         -1
//     }
// }

// This was calculating number of HOPS
impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target { return 0; }
        // {busID : {stops}}
        let mut stops: HashMap<i32, HashSet<i32>> = HashMap::new();
        // {stop: {buses}
        let mut buses: HashMap<i32, HashSet<i32>> = HashMap::new();
        for (i, rt) in routes.iter().enumerate() {
            let bus_id = i as i32;
            stops.entry(bus_id).or_insert_with(|| rt.iter().cloned().collect::<HashSet<_>>());
            for &stop in rt {
                buses.entry(stop).or_insert(HashSet::new()).insert(bus_id);
            }
        }
        // println!("{:?}", buses);
        // println!("{:?}", stops);
        let mut bus_visited : HashSet<i32> = buses.get(&source).cloned().unwrap_or_default();
        let mut q: VecDeque<i32> = buses.get(&source).into_iter().flatten().cloned().collect();
        let mut hops = 0;
        while q.len() > 0 {
            let mut sz = q.len();
            hops += 1;
            while sz > 0 {
                sz -= 1;
                let curr_bus = q.pop_front().unwrap();
                if let Some(set) = stops.get(&curr_bus) {
                    if set.contains(&target) {
                        return hops;
                    }
                }
                for av_stop in stops.get(&curr_bus).into_iter().flatten() {
                    for av_bus in buses.get(av_stop).into_iter().flatten() {
                        if !bus_visited.contains(av_bus) {
                            bus_visited.insert(*av_bus);
                            q.push_back(*av_bus);
                        }
                    }
                }
            }
        }

        -1
    }
}