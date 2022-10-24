pub struct Post {
    state: Option<Box<dyn State>>,
    cintent: String,
}

impl Post {
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        
        }
    }
}

trait State {
    fn request_ review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft{}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}
struct PendingReview{}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

