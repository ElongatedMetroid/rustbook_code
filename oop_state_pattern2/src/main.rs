use oop_state_pattern2::Post;

fn main() {
    let mut post = Post::new();
    
    post.add_text("It is 11:50 currently");

    let post = post.request_review();

    let post =  post.approve();

    assert_eq!("It is 11:50 currently", post.content());
}
