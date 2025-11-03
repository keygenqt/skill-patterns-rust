pub struct Post {
    content: String,
    state: Option<Box<dyn State>>,
}

impl Post {
    pub fn new(text: &str) -> Self {
        Self {
            content: text.to_string(),
            state: Some(Box::new(Draft::default())),
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str;
}

#[derive(Default)]
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview::default())
    }

    fn content<'a>(&self, _: &'a Post) -> &'a str {
        ""
    }
}

#[derive(Default)]
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published::default())
    }

    fn content<'a>(&self, _: &'a Post) -> &'a str {
        ""
    }
}

#[derive(Default)]
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

fn main() {
    let mut post = Post::new("I ate a salad for lunch today!");

    println!("> Draft: '{}';", post.content());

    post.request_review();
    println!("> Request: '{}';", post.content());

    post.approve();
    println!("> Approve: '{}';", post.content());
}
