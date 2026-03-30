use std::collections::HashMap;

/// We implement a suffix automaton
struct Automaton {
    st: Vec<State>,
    last: usize,
}

#[derive(Clone)]
struct State {
    length: usize,
    link: Option<usize>,
    next: HashMap<char, usize>,
} 

impl Automaton {
    fn new() -> Self {
        let start = State {
            length: 0,
            link: None,
            next: HashMap::new(),
        };
        Self { 
            st: vec![start], 
            last: 0, 
        }
    }

    fn extend(&mut self, c: char) {
        let last = self.last;
        let curr = State::new(self.st[last].len() + 1);
        let curr_idx = self.st.len();

        self.st.push(curr);


        let mut p_idx = Some(last);
        while let Some(p) = p_idx {
            if self.st[p].next.contains_key(&c) {
                break;
            }
            self.st[p].next.insert(c, curr_idx);
            p_idx = self.st[p].link;
        }
        if p_idx.is_none() {
            self.st[curr_idx].set_link(0);
            self.last = curr_idx;
            return;
        }
        let p_idx = p_idx.unwrap();

        let q_idx = *self.st[p_idx].next.get(&c).unwrap();
        if self.st[p_idx].len() + 1 == self.st[q_idx].len() {
            self.st[curr_idx].set_link(q_idx);
            self.last = curr_idx;
            return;
        }
        let mut q_copy = self.st[q_idx].clone();
        q_copy.set_length(self.st[p_idx].len() + 1);
        let q_copy_idx = self.st.len();
        self.st.push(q_copy);
        self.st[curr_idx].set_link(q_copy_idx);
        self.st[q_idx].set_link(q_copy_idx);

        let mut p_idx: Option<usize> = Some(p_idx);
        while let Some(p) = p_idx {
            if *self.st[p].next.get(&c).unwrap() != q_idx {
                break;
            }
            self.st[p].next.insert(c, q_copy_idx);
            p_idx = self.st[p].link;
        }
        self.last = curr_idx;
    }
}


impl State {
    fn new(length: usize) -> Self {
        Self {
            length: length,
            link: None,
            next: HashMap::new(),
        }
    }

    fn len(&self) -> usize {
        self.length
    }

    fn set_link(&mut self, link: usize) {
        self.link = Some(link);
    }

    fn set_length(&mut self, length: usize) {
        self.length = length;
    }
}