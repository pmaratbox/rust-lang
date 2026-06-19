fn hexs(b: &[u8]) -> String {
    b.iter().map(|x| format!("{:02x}", x)).collect()
}

fn main() {
    // MessagePack-encode the nested array [[1, 2], [3, 4]]. The outer fixarray
    // (0x92) holds two inner fixarrays, each (0x92) holding two positive
    // fixints, so the encoding is 92 92 01 02 92 03 04.
    let bytes = rmp_serde::to_vec(&vec![vec![1, 2], vec![3, 4]]).unwrap();
    println!("{}", hexs(&bytes));
}
