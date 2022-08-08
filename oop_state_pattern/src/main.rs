// 1.) A blog post starts as an empty draft.
// 2.) When the draft is done, a review of the post is requested.
// 3.) When the post is approved, it gets published.
// 4.) Only published blog posts return content to print, so 
//     unapproved posts can’t accidentally be published.

use oop_state_pattern::Post;

fn main() {
    // Create new post, state: Draft
    let mut post = Post::new();

    // Add text to post, state: Draft
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    // Request a review, state: PendingReview
    post.request_review();
    assert_eq!("", post.content());

    // Post rejected, state: Draft
    post.reject();
    assert_eq!("", post.content());

    // Request a review, state: PendingReview
    post.request_review();
    assert_eq!("", post.content());

    // Approve post, state: Published
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}