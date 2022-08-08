pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    /// Return a new, empty instance of Post
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    /// Add text to the post
    pub fn add_text(&mut self, text: &str) {
        if self.state.as_ref().unwrap().can_add_text() {
            self.content.push_str(text);
        }
    }

    /// Return the content of the post
    pub fn content(&self) -> &str {
        // We call the as_ref() method on the Option because
        // we want a reference to the value inside the Option
        // rather than ownership of the value

        // Without as_ref() we would get an error because we
        // cant move state out of the borrowed &self parameter
        // above
        self.state.as_ref().unwrap().content(self)
    }

    /// Request your post to go up for review
    pub fn request_review(&mut self) {
        // Take the value out of the option and put it in Some(s)
        if let Some(s) = self.state.take() {
            // Call request_review on the current state of Post
            // and set the current state to the return value
            self.state = Some(s.request_review());
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
}

/// Defines the behavior shared by different post states
trait State {
    fn can_add_text(&self) -> bool {
        false
    }
    // Takes a self: Box<Self>, this syntax means this method is
    // only valid when called on a Box holding the type. This
    // also takes ownership of Box<Self> invalidating the old
    // state so the state value of Post can transform into a new
    // state
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn can_add_text(&self) -> bool {
        true
    }
    // Box has not been reviewed, change state to pending 
    // review.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    // Dont change state; blog has not been reviewed
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    // Dont change state; a review request was already sent
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // Box has not been approved, change state to published
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    // Dont change state; blog was already reviewed
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // Dont change state; blog was already approved
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}