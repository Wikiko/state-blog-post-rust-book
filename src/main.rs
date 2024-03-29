use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    let mut post2 = Post::new();

    post2.add_text("Hi I'm William");
    assert_eq!("", post2.content());

    post2.request_review();
    assert_eq!("", post2.content());

    post2.approve();
    post2.approve();
    assert_eq!("Hi I'm William", post2.content());

    let mut post3 = Post::new();

    post3.request_review();
    assert_eq!("", post3.content());

    post3.add_text("LOL");
    assert_eq!("", post3.content());

    post3.approve();
    post3.approve();
    assert_eq!("", post3.content());

}
