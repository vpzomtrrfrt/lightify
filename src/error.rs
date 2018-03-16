use std;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        IO(err: std::io::Error) {
            from()
        }
        Other(desc: String) {
            from()
        }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(inp: std::string::FromUtf8Error) -> Self {
        format!("Failed to parse string: {:?}", inp).into()
    }
}
