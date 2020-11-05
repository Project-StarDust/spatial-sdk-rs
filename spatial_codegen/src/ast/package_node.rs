use crate::ASTNode;

#[derive(Debug, Eq, PartialEq)]
pub struct PackageNode {
    pub name: String,
    pub inner: Vec<Box<ASTNode>>,
}

impl PackageNode {
    pub fn add_node(self, node: ASTNode) -> Self {
        let mut inner = self.inner;
        inner.push(Box::new(node));
        Self {
            name: self.name,
            inner,
        }
    }

    pub fn has_path<S: AsRef<str>>(&self, path: S) -> bool {
        self.inner
            .iter()
            .map(|node| match &**node {
                ASTNode::SchemaNode(_) => false,
                ASTNode::PackageNode(pn) => pn.name == path.as_ref().to_string(),
            })
            .fold(false, |acc, val| acc | val)
    }

    pub fn get_exports(&self) -> Vec<String> {
        vec![self.name.clone()]
    }
}
