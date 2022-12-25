use ch17_3_oop_state_objects::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    assert_eq!("", post.content());
    println!("{}", post.content());

    post.request_review();

    assert_eq!("", post.content());
    println!("{}", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
    println!("{}", post.content());
}
