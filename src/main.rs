use bulut::github::GitHub;

fn main() {
    let git = GitHub::new("osmon-lang", "havo");
    println!("{:?}", git.releases());
}
