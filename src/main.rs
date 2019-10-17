use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    let mut post2 = Post::new();

    post2.add_text("Hi I'm William");

    let post2 = post2.request_review();

    let mut post2 = post2.reject();

    post2.add_text(" Legal");

    let post2 = post2.request_review().approve();
    assert_eq!("Hi I'm William Legal", post2.content());

}
