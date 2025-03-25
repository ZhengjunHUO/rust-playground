struct CustomIter<'a> {
    input: &'a str,
    delim: char,
    index: usize,
}

impl<'a> CustomIter<'a> {
    fn new(input: &'a str, delim: char) -> Self {
        Self {
            input,
            delim,
            index: 0,
        }
    }
}

impl<'a> Iterator for CustomIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.input.len() - 1 {
            return None;
        }

        let str_left = &self.input[self.index..];

        match str_left.find(self.delim) {
            Some(idx) => {
                self.index += idx + 1;
                Some(&str_left[..idx])
            }
            None => {
                self.index = self.input.len() - 1;
                Some(str_left)
            }
        }
    }
}

fn main() {
    let input = String::from(
        "heap allocation, file I/O, threading, networking, and collections (Vec, HashMap, etc.)",
    );
    let expect = input.split(',').collect::<Vec<_>>();

    let custom = CustomIter::new(&input, ',');
    assert_eq!(custom.collect::<Vec<_>>(), expect);
}
