// This is another way to implement a state pattern in rust; by
// encoding states into different types.
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

impl Post {
    /// Create a new blog post, blog posts will always start
    /// out as a draft, so we will return the seperate
    /// DraftPost type created above.
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    /// Return a &str containing the contents of the post
    pub fn content(&self) -> &str {
        &self.content
    }
}

// Implementation for methods that can only run on the draft
// of the blog.
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text)
    }
    
    /// This will return a instance of PendingReviewPost.
    /// Take ownership of self, this will consume the DraftPost
    /// instance and tranform it into a PendingReviewPost
    /// instance. This way we wont have any lingering DraftPost
    /// instances after request_review has been called on them.
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost { content: self.content }
    }
}

impl PendingReviewPost {
    /// This will return an instance of Post
    pub fn approve(self) -> Post {
        Post { content: self.content }
    }

    /// This will return an instance of DraftPost
    pub fn reject(self) -> DraftPost {
        DraftPost { content: self.content }
    }
}