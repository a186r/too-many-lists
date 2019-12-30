//二叉树不是树的一种特殊情形，尽管其与树具有许多相似之处，但树和二叉树有两个主要区别：
//1。树中结点的最大度数没有限制，而二叉树结点的最大度数为2。
//2。树的结点无左右之分，而二叉树的结点有左右之分。
//二叉树的每个结点由键值与左右树组成

type TreeNode<K, V> = Option<Box<Node<K, V>>>;
#[derive(Debug)]
struct Node<K, V: std::fmt::Display>{
    left: TreeNode<K, V>,
    right: TreeNode<K, V>,
    key: K,
    value: V,
}

//由于二叉查找树要求键可排序，所以我们要求K实现PartialOrd
trait BinaryTree<K, V> {
    fn pre_order(&self);
    fn in_order(&self);
    fn pos_order(&self);
}

//自动派生出的 PartialEq 将会对你的类型中的所有部分做相等检查
//PartialOrd 定义了部分顺序, 并通过 Ordering 关系扩展了 PartialEq 的相等性. 这里的 Partial 意味着你的类型的某些实例可能不能被有意义地比较.
trait BinarySearchTree<K: PartialOrd, V>: BinaryTree<K, V>{
    fn insert(&mut self, key: K, value: V);
}

impl<K, V: std::fmt::Display> Node<K, V> {
    fn new(key: K, value: V) -> Node<K, V> {
        Node{
            left: None,
            right: None,
            key,
            value,
        }
    }
}

impl<K: PartialOrd, V: std::fmt::Display> BinarySearchTree<K, V> for Node<K, V> {
     fn insert(&mut self, key: K, value: V) {
        if self.key < key{
            if let Some(ref mut right) = self.right{
                right.insert(key, value);
            }else{
                self.right = Some(Box::new(Node::new(key, value)));
            }
        }else{
            if let Some(ref mut left) = self.left{
                left.insert(key, value);
            }else {
                self.left = Some(Box::new(Node::new(key, value)));
            }
        }
    }
}

//二叉树的遍历
impl<K, V:std::fmt::Display> BinaryTree<K, V> for Node<K, V> {
//    先序遍历：首先访问根，再先序遍历左(右)子树，最后先序遍历右(左)子树
    fn pre_order(&self) {
        println!("{}", self.value);

        if let Some(ref left) = self.left{
            left.pre_order();
        }
        if let Some(ref right) = self.right{
            right.pre_order();
        }
    }

//    中序遍历：首先中序遍历左(右)子树，再访问根，最后中序遍历右(左)子树
    fn in_order(&self) {
        if let Some(ref left) = self.left{
            left.in_order();
        }
        println!("{}", self.value);

        if let Some(ref right) = self.right{
            right.in_order();
        }
    }

//    后序遍历：首先后序遍历左(右)树，再遍历右(左)树，最后访问根
    fn pos_order(&self) {
        if let Some(ref left) = self.left{
            left.pos_order();
        }

        if let Some(ref right) = self.right{
            right.pos_order();
        }

        println!("{}", self.value);
    }
}

//测试代码

#[cfg(test)]
mod tests{
    use super::*;

    type BST<K, V> = Node<K, V>;

    #[test]
    fn test_insert(){
        let mut root = BST::<i32,i32>::new(3, 4);
        root.insert(2, 3);
        root.insert(4, 6);
        root.insert(5, 5);
        root.insert(6, 6);
        root.insert(1, 8);
        root.insert(9, 7);

        if let Some(ref left) = root.left{
            assert_eq!(left.value, 3);
        }

        if let Some(ref right) = root.right{
            assert_eq!(right.value, 6);
            if let Some(ref right) = right.right{
                assert_eq!(right.value, 5);
            }
        }
        println!("Pre Order traversal");
        root.pre_order();
        println!("In Order traversal");
        root.in_order();
        println!("Pos Order taversal");
        root.pos_order();
    }
}
