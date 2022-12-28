pub struct MyQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

impl MyQueue {
    pub fn new()-> Self {
        MyQueue { stack1: vec![], stack2: vec![] }
    }

    pub fn push(&mut self, x: i32) {
        while let Some(v) = self.stack1.pop() {
            self.stack2.push(v);
        }
        self.stack1.push(x);
        while let Some(v) = self.stack2.pop() {
            self.stack1.push(v)
        } 
    }

    pub fn peek(&self) -> i32 {
        *self.stack1.last().unwrap()
    }
    pub fn pop(&mut self) -> i32 {
        self.stack1.pop().unwrap()
    }
    pub fn empty(&self) -> bool {
        self.stack1.is_empty()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_queue() {
        let mut queue = MyQueue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(queue.peek(), 1);
        assert_eq!(queue.pop(), 1);
        assert!(!queue.empty());
        assert_eq!(queue.pop(), 2);
        assert!(queue.empty());

    }
}