use draw::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("eat salad at lunch.");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("eat salad at lunch.", post.content());
}
