use std::io;
use std::fmt;
use std::fs;
use std::path::Path;

#[derive(Clone, Copy)]
pub enum Indent {
    Blank,  //
    Uplink, // |
    Split,  // +
    Last,   // `
}

impl fmt::Display for Indent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Indent::Blank  => write!(f, "   "),
            Indent::Uplink => write!(f, "|  "),
            Indent::Split  => write!(f, "+--"),
            Indent::Last   => write!(f, "`--"),
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub children: Vec<Node>
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix = vec![];
        f.write_str(self.render(&prefix, true).as_ref())
    }
}

impl Node {
    // a few examples:
    //
    // node
    // `- node1
    //
    // node
    // +- node1
    // `- node2
    //
    // node
    // +- node1
    // |  +- node11
    // |  `- node12
    // `- node2

    fn render(&self, prefix: &Vec<Indent>, is_last: bool) -> String {
        let mut result = String::new();

        // add current prefix
        for indent in prefix.iter() {
            result.push_str(&indent.to_string());
            result.push(' ');
        }

        // render the name
        result.push_str(&self.name);

        // render children
        for (i, child) in self.children.iter().enumerate() {
            // introduce new prefix for children
            let mut child_prefix = prefix.clone();

            if let Some(last) = child_prefix.last_mut() {
                *last = if is_last { Indent::Blank } else { Indent::Uplink };
            }

            // add another indentation to the prefix
            let is_last_inner = i == self.children.len() - 1;
            let next = if is_last_inner { Indent::Last } else { Indent::Split };
            child_prefix.push(next);

            // and recurse
            result.push('\n');
            result.push_str(&child.render(&child_prefix, is_last_inner))
        }

        result
    }

    pub fn singleton(name: String) -> Node {
        Node { name, children: vec![] }
    }

    pub fn flat(name: String, children: Vec<String>) -> Node {
        if children.is_empty() {
            Self::singleton(name)
        }
        else {
            let nodes = children.iter()
                                .map(|x| Self::singleton(x.to_string()))
                                .collect();
            Node { name, children: nodes }
        }
    }

    fn traverse(path: &Path, node: &mut Node) -> io::Result<()> {
        if path.is_dir() {
            let children = fs::read_dir(path)?;
            for child in children {
                let child_entry = child?;
                let child_path = child_entry.path();
                let child_name = child_path.file_name()
                                           .map_or_else(|| child_path.display().to_string(),
                                                        |n| n.to_string_lossy().to_string());

                let mut child_node = Self::singleton(child_name);
                Self::traverse(&child_path, &mut child_node)?;
                node.children.push(child_node);
            }
        }

        Ok(())
    }

    pub fn from_path(path: &Path) -> io::Result<Node> {
        let s = path.display();
        let mut node = Self::singleton(s.to_string());
        Self::traverse(path, &mut node)?;

        Ok(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singleton() {
        let singleton = Node::singleton("name".to_string());
        assert_eq!(singleton.to_string(), "name".to_string());
    }

    #[test]
    fn four_levels() {
        let tree = Node {
            name: "parent".to_string(),
            children: vec![
                Node { name: "child 1".to_string(), children: vec![
                    Node{ name: "grandkid 1 1".to_string(), children: vec![
                        Node{ name: "greatgrandkid 1 1 1".to_string(), children: vec![]}
                    ]},
                    Node{ name: "grandkid 1 2".to_string(), children: vec![]},
                    Node{ name: "grandkid 1 3".to_string(), children: vec![]}
                ]},
                Node { name: "child 2".to_string(), children: vec![]},
                Node { name: "child 3".to_string(), children: vec![
                    Node{ name: "grandkid 3 1".to_string(), children: vec![]}
                ]}
            ]
        };

        let rendered =
            "parent
+-- child 1
|   +-- grandkid 1 1
|   |   `-- greatgrandkid 1 1 1
|   +-- grandkid 1 2
|   `-- grandkid 1 3
+-- child 2
`-- child 3
    `-- grandkid 3 1";
        assert_eq!(tree.to_string(), rendered.to_string());
    }
}
