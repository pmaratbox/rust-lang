use itertools::Itertools;

fn main() {
    // Group [1,2,3,4,5,6] by parity using itertools' `into_group_map_by`,
    // which builds a HashMap<key, Vec<value>> from the grouping function.
    let groups = (1..=6).into_group_map_by(|x| if x % 2 == 0 { "even" } else { "odd" });

    // Sort keys (even before odd) and render each as `key:v1,v2,...`,
    // joining groups with ';'.
    let out = groups
        .keys()
        .sorted()
        .map(|k| format!("{}:{}", k, groups[k].iter().join(",")))
        .join(";");

    println!("{}", out);
}
