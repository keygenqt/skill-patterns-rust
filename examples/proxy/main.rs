use std::sync::atomic::{AtomicUsize, Ordering};

pub static ID: AtomicUsize = AtomicUsize::new(1);

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct YouTube {
    id: u64,
    name: String,
}

///////////////////////////

trait ThirdPartyYouTubeLib {
    fn get_video_by_id(&mut self, id: u64) -> YouTube;
}

trait ThirdPartyYouTubeLibReset {
    fn reset(&mut self);
}

///////////////////////////

struct ThirdPartyYouTube;

impl ThirdPartyYouTube {
    fn new() -> Self {
        Self {}
    }
}

impl ThirdPartyYouTubeLibReset for ThirdPartyYouTube {
    fn reset(&mut self) {
        // api not have reset
    }
}

impl ThirdPartyYouTubeLib for ThirdPartyYouTube {
    fn get_video_by_id(&mut self, _: u64) -> YouTube {
        // A difficult request
        // id no use mock data
        YouTube {
            id: ID.fetch_add(1, Ordering::Relaxed) as u64,
            name: "Best video".to_string(),
        }
    }
}

///////////////////////////

struct CachedYouTube<T: ThirdPartyYouTubeLib + ThirdPartyYouTubeLibReset> {
    service: T,
    video: Option<YouTube>,
}

impl <T: ThirdPartyYouTubeLib + ThirdPartyYouTubeLibReset>CachedYouTube<T> {
    fn new(service: T) -> Self {
        Self { service, video: None }
    }
}

impl <T: ThirdPartyYouTubeLib + ThirdPartyYouTubeLibReset>ThirdPartyYouTubeLibReset for CachedYouTube<T> {
    fn reset(&mut self) {
        self.video = None
    }
}

impl <T: ThirdPartyYouTubeLib + ThirdPartyYouTubeLibReset>ThirdPartyYouTubeLib for CachedYouTube<T> {
    fn get_video_by_id(&mut self, id: u64) -> YouTube {
        if self.video.is_none() {
            self.video = Some(self.service.get_video_by_id(id))
        }
        self.video.clone().unwrap()
    }
}

///////////////////////////

struct YouTubeManager<T: ThirdPartyYouTubeLib + ThirdPartyYouTubeLibReset> {
    service: T,
}

impl <T: ThirdPartyYouTubeLib + ThirdPartyYouTubeLibReset>YouTubeManager<T> {
    fn new(service: T) -> Self {
        Self { service }
    }

    fn print_video_by_id(&mut self, id: u64) {
        let video: YouTube = self.service.get_video_by_id(id);
        println!("{:?}", video)
    }

    fn reset(&mut self) {
        self.service.reset();
    }
}

fn main() {
    let you_tube_service = ThirdPartyYouTube::new();
    let you_tube_proxy = CachedYouTube::new(you_tube_service);
    let mut manager = YouTubeManager::new(you_tube_proxy);

    println!("======================");
    println!("> Proxy get by id (1):");
    manager.print_video_by_id(1);
    manager.print_video_by_id(1);

    println!("======================");
    println!("> Proxy get by id (2):");
    manager.reset(); // clear data
    manager.print_video_by_id(2);
    manager.print_video_by_id(2);

    let you_tube_service = ThirdPartyYouTube::new();
    let mut manager = YouTubeManager::new(you_tube_service);

    println!("======================");
    println!("> Just get by id (3):");
    manager.print_video_by_id(3);
}
