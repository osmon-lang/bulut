use bulut::github::GitHub;

fn main() {
    let git = GitHub::new("osmon-lang", "havo");
    println!("Releases: {:?}", git.releases().unwrap());

    let release = git.release("v0.0.1".to_string()).expect("oopsie woopsie");
}
