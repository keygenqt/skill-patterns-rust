trait ParserVisitor {
    type Value;
    fn visit(&self, v: &str) -> Self::Value;
}

#[derive(Debug)]
struct BytesStruct {
    value: Vec<u8>,
}

impl BytesStruct {
    fn print(&self) {
        println!("> {}", String::from_utf8_lossy(&self.value))
    }
}

impl Default for BytesStruct {
    fn default() -> Self {
        Self { value: vec![] }
    }
}

impl ParserVisitor for BytesStruct {
    type Value = BytesStruct;

    fn visit(&self, v: &str) -> Self::Value {
        BytesStruct {
            value: v.as_bytes().to_vec(),
        }
    }
}

#[derive(Debug)]
struct StringStruct {
    value: String,
}

impl StringStruct {
    fn print(&self) {
        println!("> {}", self.value)
    }
}

impl Default for StringStruct {
    fn default() -> Self {
        Self {
            value: String::new(),
        }
    }
}

impl ParserVisitor for StringStruct {
    type Value = StringStruct;

    fn visit(&self, v: &str) -> Self::Value {
        StringStruct {
            value: v.to_string(),
        }
    }
}

trait Parser<T: ParserVisitor> {
    fn new(visitor: T) -> Self;
    fn parse_vec(&self, _: Vec<u8>) -> Result<T::Value, Box<dyn std::error::Error>> {
        Err("parse is unimplemented")?
    }
    fn parse_string(&self, _: String) -> Result<T::Value, Box<dyn std::error::Error>> {
        Err("parse is unimplemented")?
    }
}

struct VecParser<T: ParserVisitor> {
    visitor: T,
}

impl<T: ParserVisitor> Parser<T> for VecParser<T> {
    fn new(visitor: T) -> Self {
        Self { visitor }
    }

    fn parse_vec(&self, value: Vec<u8>) -> Result<T::Value, Box<dyn std::error::Error>> {
        Ok(self.visitor.visit(&String::from_utf8_lossy(&value)))
    }
}

struct StringParser<T: ParserVisitor> {
    visitor: T,
}

impl<T: ParserVisitor> Parser<T> for StringParser<T> {
    fn new(visitor: T) -> Self {
        Self { visitor }
    }

    fn parse_string(&self, value: String) -> Result<T::Value, Box<dyn std::error::Error>> {
        Ok(self.visitor.visit(&value))
    }
}

fn main() {
    let parser = VecParser::new(StringStruct::default());
    let result = parser.parse_vec("StringStruct from bytes vec.".as_bytes().to_vec());
    result.unwrap().print();

    let parser = StringParser::new(BytesStruct::default());
    let result = parser.parse_string("BytesStruct from string.".to_string());
    result.unwrap().print();
}
