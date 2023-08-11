#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
  pub val: i32,
  pub children: Vec<Box<TreeNode>>,
  pub files: Vec<i32>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            children: vec![],
            files: vec![],
        }
    }

    pub fn add_child(&mut self, child: TreeNode) {
        self.children.push(Box::new(child));
        self.val = self.sum_files()
    }

    pub fn add_file(&mut self, val: i32) {
        self.files.push(val);
        self.val = self.sum_files()
    }

    pub fn sum_files(&self) -> i32 {
        let mut total = self.files.iter().sum();
        println!("{:?} - total = {}", self, total);
        total += self.children.iter().map(|child| child.clone().sum_files()).sum::<i32>();
        total
    }


}


pub fn puzzle1(_input: &str) -> i64 {
    0
}

pub fn puzzle2(_input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn node_add_child() {
        let mut node = TreeNode::new(1);
        let child = TreeNode::new(2);
        node.add_child(child.clone());
        assert_eq!(node.children[0], Box::new(child));
        assert_eq!(node.val, 0);
    }

    #[test]
    fn node_add_file() {
        let mut node = TreeNode::new(1);
        node.add_file(2);
        assert_eq!(node.files[0], 2);
        assert_eq!(node.val, 2);
    }

    #[test]
    fn node_sum_files() {
        let mut node = TreeNode::new(0);
        node.add_file(2);
        let mut child = TreeNode::new(0);
        child.add_file(3);
        node.add_child(child);
        assert_eq!(node.sum_files(), 5)
    }

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(""), 0);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(""), 0);
    }
}
