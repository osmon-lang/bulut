use reqwest::blocking::Response;
use std::fs::File;
use std::io::Cursor;

#[derive(Debug)]
pub struct Download {
    url: String,
    file: File,
}

#[derive(Debug, Copy, Clone)]
pub enum DownloadError {
    ResponseError,
    CantCreateFile,
    IOCopyError,
}

impl Download {
    pub fn new(url: String) -> Self {
        Self {
            file: Download::stream_file(url.clone()).unwrap(),
            url,
        }
    }

    fn get_response(&self) -> Result<Response, DownloadError> {
        let response = reqwest::blocking::get(&self.url).unwrap();

        if response.status().is_success() {
            Ok(response)
        } else {
            Err(DownloadError::ResponseError)
        }
    }

    pub fn get_filename(url: String) -> String {
        let url = url.split('/').collect::<Vec<&str>>();
        url[url.len() - 1].to_string()
    }

    pub fn stream_file(url: String) -> Result<File, DownloadError> {
        Ok(match File::create(Download::get_filename(url)) {
            Ok(p) => p,
            Err(_) => return Err(DownloadError::CantCreateFile),
        })
    }

    pub fn download(&mut self) -> Result<(), DownloadError> {
        let response = self.get_response().unwrap();

        if response.content_length().unwrap() == 0 {
            return Err(DownloadError::ResponseError);
        }

        let mut content = Cursor::new(response.bytes().unwrap());
        std::io::copy(&mut content, &mut self.file).unwrap();

        Ok(())
    }
}
