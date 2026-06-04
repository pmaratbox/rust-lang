use std::path::Path;

fn main() {
    let joined = Path::new("/tmp").join("file.txt");

    let full = joined.to_string_lossy().replace('\\', "/");
    let basename = joined.file_name().unwrap().to_string_lossy();
    let ext = format!(".{}", joined.extension().unwrap().to_string_lossy());

    println!("{} {} {}", full, basename, ext);
}
