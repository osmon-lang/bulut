use bulut::download::Download;
use bulut::github::GitHub;

fn main() {
    let git = GitHub::new("osmon-lang", "havo");
    println!("Releases: {:?}", git.releases().unwrap());

    let url = "https://github.com/osmon-lang/havo/releases/download/v0.0.3/libhavo.h";
    println!("Downloading: {}", url);

    // git.download_file("v0.0.3", "libhavo.h").expect("TODO: panic message");
    let mut downloader = Download::new(url.to_string());
    downloader.download().expect("Couldn't download");
}
