use std::collections::HashMap;

fn main() {
    let temps = vec![-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5];
    let map = group_temps(&temps);
    println!("{:?}", map)
}

fn group_temps(temps: &Vec<f32>) -> HashMap<(i32, i32), Vec<&f32>> {
    let mut map = HashMap::new();
    for temp in temps {
        let lower_bound = ((temp / 10.0).floor() * 10.0) as i32;
        let upper_bound = lower_bound + 10;
        map.entry((lower_bound, upper_bound))
            .or_insert(Vec::new())
            .push(temp);
    }
    map
}