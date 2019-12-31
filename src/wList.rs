use List::*;

enum List{
//    Cons：包含一个元素和一个指向下一个节点的指针的元组结构
    Cons(u32, Box<List>),
//    Nil：表示一个链表结点的末端
    Nil,
}

impl List{
//    创建一个空链表
    fn new() -> List{
        Nil
    }
//    在前面增加一个元素节点，并且链接旧的链表和返回新的链表
    fn prepend(self, elem: u32) -> List{
        Cons(elem, Box::new(self))
    }

//    返回链表的长度
    fn len(&self) -> u32{
//    self的类型是&List，*self的类型是List
        match *self {
//            因为self是借用的，所以不能转移tail的所有权。
//            因此使用tail的引用
            Cons(_, ref tail) => 1 + tail.len(),
//            所有空的链表长度都等于0
            Nil => 0,
        }
    }

    fn stringify(&self) -> String{
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use crate::wList::List;

    #[test]
    fn maint(){
        let mut list = List::new();

        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(4);

        println!("linked list has length: {}", list.len());
        println!("{}", list.stringify());
    }
}