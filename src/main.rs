struct Document {
    nodes: Vec<Node>,
}

impl Document {
    fn new() -> Document {
        Document { nodes: vec![] }
    }
    fn add_node<T: Into<String>>(&mut self, value: T) -> &mut Document {
        self.nodes.push(Node::new(value));
        self //for chaining
    }
    fn select_first(&self, value: &str) -> Option<pointer> {
        for i in 0..self.nodes.len() {
            if self.nodes[i].matches(value) {
                return Some(pointer {
                    history: vec![(&self.nodes, i)],
                });
            }
            match self.nodes[i].select_first_from_children(value, vec![(&self.nodes, i)]) {
                Some(p) => return Some(p),
                None => (),
            }
        }
        None
    }
}

struct Node {
    value: String,
    children: Vec<Node>,
}

impl Node {
    fn new<T: Into<String>>(value: T) -> Node {
        Node {
            value: value.into(),
            children: vec![],
        }
    }

    fn add_child<T: Into<String>>(&mut self, value: T) -> &mut Node {
        self.children.push(Node::new(value));
        self //for chaining
    }
    fn to_string(&self) -> String {
        let mut out = self.value.clone();
        for child in &self.children {
            out.push_str(&format!("\n{}", child.to_string_with_offset("-")))
        }
        out
    }
    fn to_string_with_offset(&self, offset: &str) -> String {
        let mut out = format!("{}{}", offset, self.value.clone());
        for child in &self.children {
            out.push_str(&format!(
                "\n{}",
                child.to_string_with_offset(&format!("{}{}", offset, offset))
            ));
        }
        out
    }
    fn matches(&self, value: &str) -> bool {
        self.value == value
    }
}
impl<'a> Node {
    fn select_first_from_children(
        &'a self,
        value: &str,
        mut history: Vec<(&'a Vec<Node>, usize)>,
    ) -> Option<pointer<'a>> {
        for i in 0..self.children.len() {
            if self.children[i].matches(value) {
                history.push((&self.children, i));
                return Some(pointer { history: history });
            }
        }
        None
    }
}

struct pointer<'a> {
    history: Vec<(&'a Vec<Node>, usize)>,
}

impl pointer<'_> {
    fn node(&self) -> Option<&Node> {
        if self.history.len() == 0 {
            return None;
        }
        let (v, i) = self.history[self.history.len() - 1];
        Some(&v[i])
    }
    fn parent(&self) -> Option<&Node> {
        if self.history.len() <= 1 {
            return None;
        }
        let (v, i) = self.history[self.history.len() - 2];
        Some(&v[i])
    }
}

fn main() {
    let mut doc = Document::new();
    doc.add_node("Top");

    let top = &mut doc.nodes[0];
    top.add_child("2nd").add_child("2nd-2");
    let nextgen = &mut top.children[0];
    nextgen.add_child("3rd").add_child("3rd2");
    println!("{}", top.to_string());
    println!("Selecting '2nd'");
    let p = doc.select_first("2nd").unwrap();
    println!("{}", p.node().unwrap().to_string());
    println!("Selecting its parent");
    println!("{}", p.parent().unwrap().to_string());
}
