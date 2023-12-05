fn main() {
    let numv = num_cpus::get();
    let numf = num_cpus::get_physical();

    println!("num virtual cores: {}", numv);
    println!("num physical cores: {}", numf);
}
