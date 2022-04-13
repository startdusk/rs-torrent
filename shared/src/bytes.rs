#[derive(Debug)]
pub struct ByteBuffer<'a> {
    // 这里用数组，意味着如果数据很大，会占很大一块连续的内存
    // 可以考虑换linkedlist
    data: &'a [u8],
    cur_pos: usize,
    len: usize,
}

impl<'a> ByteBuffer<'a> {
    pub fn new(data: &[u8]) -> ByteBuffer {
        ByteBuffer {
            data,
            cur_pos: 0,
            len: data.len(),
        }
    }

    pub fn peek(&self) -> Option<&'a u8> {
        if self.is_empty() {
            None
        } else {
            Some(&self.data[self.cur_pos])
        }
    }

    pub fn advance(&mut self, step: usize) {
        self.cur_pos += step;
        if self.cur_pos > self.len {
            self.cur_pos = self.len;
        }
    }

    pub fn push_back(&mut self, step: usize) {
        if step > self.cur_pos {
            self.cur_pos -= self.cur_pos
        } else {
            self.cur_pos -= step
        }
    }

    pub fn pos(&self) -> usize {
        self.cur_pos
    }

    pub fn is_empty(&self) -> bool {
        self.cur_pos >= self.len
    }
}

impl<'a> Iterator for ByteBuffer<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<&'a u8> {
        if self.is_empty() {
            None
        } else {
            self.cur_pos += 1;
            Some(&self.data[self.cur_pos - 1])
        }
    }
}

#[cfg(test)]
mod byte_buffer_tests {
    use super::*;

    #[test]
    fn byte_buffer_sanity_test() {
        let bytes = vec![1, 2, 3];
        let mut buffer = ByteBuffer::new(&bytes);

        assert!(!buffer.is_empty());
        assert_eq!(buffer.peek(), Some(&1));
        assert_eq!(buffer.pos(), 0);
        buffer.advance(1);

        assert!(!buffer.is_empty());
        assert_eq!(buffer.peek(), Some(&2));
        assert_eq!(buffer.pos(), 1);
        buffer.advance(2);

        assert!(buffer.is_empty());
        assert_eq!(buffer.peek(), None);
        assert_eq!(buffer.pos(), 3);
        buffer.advance(1);

        assert!(buffer.is_empty());
        assert_eq!(buffer.peek(), None);
        assert_eq!(buffer.pos(), 3);
    }

    #[test]
    fn byte_buffer_iterator_test() {
        let bytes = vec![1, 2, 3];
        let mut buffer = ByteBuffer::new(&bytes);
        let mut output = Vec::new();

        for byte in &mut buffer {
            output.push(*byte);
        }

        assert!(buffer.is_empty());
        assert_eq!(buffer.peek(), None);
        assert_eq!(buffer.next(), None);
        assert_eq!(buffer.pos(), 3);
        assert_eq!(bytes, output);
    }

    #[test]
    fn byte_buffer_push_back() {
        let bytes = vec![1, 2, 3];
        let mut buffer = ByteBuffer::new(&bytes);

        assert_eq!(buffer.next(), Some(&1));
        buffer.push_back(1);
        assert_eq!(buffer.next(), Some(&1));
        assert_eq!(buffer.pos(), 1);
    }
}
