use core::ptr::NonNull;

pub struct QueueNode {
    pub next: Option<NonNull<QueueNode>>,
}

impl QueueNode {
    pub const fn new() -> Self {
        QueueNode { next: None }
    }
}

pub struct QueueList {
    head: Option<NonNull<QueueNode>>,
    tail: Option<NonNull<QueueNode>>,
}
impl QueueList {
    pub const fn new() -> Self {
        QueueList {
            head: None,
            tail: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    #[inline]
    pub unsafe fn enqueue(&mut self, mut node: NonNull<QueueNode>) {
        unsafe {
            node.as_mut().next = None;
        }
        match self.tail {
            Some(mut tail) => unsafe {
                tail.as_mut().next = Some(node)
            },
            None => self.head = Some(node),
        }
        self.tail = Some(node);
    }

    #[inline]
    pub unsafe fn dequeue(&mut self) -> Option<NonNull<QueueNode>> {
        let node = self.head?;
        unsafe {
            self.head = node.as_ref().next;
        }
        if self.head.is_none() {
            self.tail = None;
        }
        Some(node)
    }
}
