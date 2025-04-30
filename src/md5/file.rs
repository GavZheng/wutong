use md5::Context;
use pbr::ProgressBar;
use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};
use crate::Md5Error;

impl From<io::Error> for Md5Error {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

pub struct Md5Calculator {
    context: Context,
    progress: Option<ProgressBar<io::Stdout>>,
}

impl Md5Calculator {
    pub fn new(with_progress: bool, total_size: Option<u64>) -> Self {
        let progress = with_progress.then(|| {
            let mut pb = ProgressBar::new(total_size.unwrap_or(0));
            pb.set_units(pbr::Units::Bytes);
            pb.format("[█-░]");
            pb.show_speed = true;
            pb.show_time_left = true;
            pb
        });

        Self {
            context: Context::new(),
            progress,
        }
    }
    
    pub fn update(&mut self, data: &[u8]) {
        self.context.consume(data);
        if let Some(pb) = &mut self.progress {
            pb.add(data.len() as u64);
        }
    }
    
    pub fn finalize(self) -> String {
        format!("\n{:x}", self.context.compute())
    }
}

pub fn md5_file(path: &Path) -> Result<String, Md5Error> {
    let metadata = path.metadata().map_err(|_| Md5Error::FileNotFound)?;
    if !metadata.is_file() {
        return Err(Md5Error::FileNotFound);
    }

    let mut file = File::open(path)?;
    let mut calculator = Md5Calculator::new(true, Some(metadata.len()));
    let mut buffer = [0u8; 16384];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        calculator.update(&buffer[..bytes_read]);
    }

    Ok(calculator.finalize())
}
