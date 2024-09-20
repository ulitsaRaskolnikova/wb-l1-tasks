use std::collections::HashSet;

fn find_intersection(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    let mut intersection = HashSet::new();
    
    for &item in set1 {
        // Если элемент есть во втором множестве, добавляем его в результат
        if set2.contains(&item) {
            intersection.insert(item);
        }
    }
    
    intersection
}

fn main() {
    let set1: HashSet<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let set2: HashSet<i32> = vec![3, 4, 5, 6, 7].into_iter().collect();
    let intersection = find_intersection(&set1, &set2);
    println!("Intersection: {:?}", intersection);
}
