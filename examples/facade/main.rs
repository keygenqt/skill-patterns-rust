use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Clone)]
struct VideoFile {
    path: PathBuf
}

impl VideoFile {
    fn new(path: PathBuf) -> VideoFile {
        VideoFile {
            path
        }
    }
}

struct OggCompressionCodec;

impl OggCompressionCodec {
    fn new() -> OggCompressionCodec {
        OggCompressionCodec {}
    }

    fn convert(&self, _ : VideoFile) -> bool {
        // convert file to OGG
        return true;
    }
}

struct Mp4CompressionCodec;

impl Mp4CompressionCodec {
    fn new() -> Mp4CompressionCodec {
        Mp4CompressionCodec {}
    }

    fn convert(&self, _ : VideoFile) -> bool {
        // convert file to MP4
        return true;
    }
}

#[derive(PartialEq)]
#[derive(Clone, Copy)]
enum Codec {
    OGG,
    MP4,
}

struct ConverterFacade {
    codec : Codec,
    video_file : VideoFile,
    ogg_compression : Option<OggCompressionCodec>,
    mp4_compression : Option<Mp4CompressionCodec>,
}

impl ConverterFacade {
    fn new(path: PathBuf, codec: Codec) -> ConverterFacade {
        ConverterFacade {
            codec,
            video_file: VideoFile::new(path),
            ogg_compression: if codec == Codec::OGG { Some(OggCompressionCodec::new()) } else { None },
            mp4_compression: if codec == Codec::MP4 { Some(Mp4CompressionCodec::new()) } else { None },
        }
    }

    fn convert(&self) -> bool {
        match self.codec {
            Codec::OGG => match &self.ogg_compression {
                Some(compression) => compression.convert(self.video_file.clone()),
                None => false,
            },
            Codec::MP4 => match &self.mp4_compression {
                Some(compression) => compression.convert(self.video_file.clone()),
                None => false,
            },
        }
    }
}

fn main() {
    let converter = ConverterFacade::new(PathBuf::from("/path/to/file.avi"), Codec::MP4);
    println!("Facade convert video, result: {}", converter.convert());
}
