use std::{
    fs::{self, File},
    io::Write,
};

trait DataSource {
    fn save_data(&self, data: String);
    fn read_data(&self) -> String;
    fn remove(&self);
}

#[derive(Clone)]
struct FileSource {
    path: String,
}

impl FileSource {
    fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

impl DataSource for FileSource {
    fn save_data(&self, data: String) {
        let mut file = File::create(&self.path).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }

    fn read_data(&self) -> String {
        fs::read_to_string(&self.path).expect("Should have been able to read the file")
    }

    fn remove(&self) {
        let _ = fs::remove_file(&self.path);
    }
}

struct SourceDecorator {
    wrap: Box<dyn DataSource>,
}

impl SourceDecorator {
    fn new(source: Box<dyn DataSource>) -> Self {
        Self { wrap: source }
    }
}

impl DataSource for SourceDecorator {
    fn save_data(&self, data: String) {
        self.wrap.save_data(data)
    }

    fn read_data(&self) -> String {
        self.wrap.read_data()
    }

    fn remove(&self) {
        self.wrap.remove()
    }
}

struct EncryptSourceDecorator {
    wrap: SourceDecorator,
}

impl EncryptSourceDecorator {
    fn new(source: Box<dyn DataSource>) -> Self {
        Self {
            wrap: SourceDecorator::new(source),
        }
    }
}

impl DataSource for EncryptSourceDecorator {
    fn save_data(&self, data: String) {
        self.wrap.save_data(format!("encrypt -> {}", data))
    }

    fn read_data(&self) -> String {
        self.wrap.read_data()
    }

    fn remove(&self) {
        self.wrap.remove()
    }
}

struct CompressSourceDecorator {
    wrap: SourceDecorator,
}

impl CompressSourceDecorator {
    fn new(source: Box<dyn DataSource>) -> Self {
        Self {
            wrap: SourceDecorator::new(source),
        }
    }
}

impl DataSource for CompressSourceDecorator {
    fn save_data(&self, data: String) {
        self.wrap.save_data(format!("compress -> {}", data))
    }

    fn read_data(&self) -> String {
        self.wrap.read_data()
    }

    fn remove(&self) {
        self.wrap.remove()
    }
}

fn main() {
    let file_name = "my_file.temp";
    let source = FileSource::new(file_name);

    let encrypt = EncryptSourceDecorator::new(Box::new(source.clone()));
    encrypt.save_data("Encrypt data".to_string());

    let encrypt = CompressSourceDecorator::new(Box::new(source.clone()));
    encrypt.save_data("Compress data".to_string());

    // Result
    println!("{}", source.read_data());

    // Clean up
    source.remove();
}
