pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost { content: String::new()}
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}



pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}



pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_state_pattern_blog() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
    
        let post = post
            .request_review()
            .approve();
    
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}