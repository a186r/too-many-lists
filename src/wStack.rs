// 实现栈结构的步骤
// 1.定义一个那结构Stack
// 2.定义组成栈的初始化函数StackNode
// 3.实现栈的初始化函数new()
// 4.实现进栈函数push()
// 5.实现退栈函数pop()

#[derive(Debug)] //方便调试
struct Stack<T>{
    top: Option<Box<StackNode<T>>>,
}

#[derive(Debug)]
struct StackNode<T>{
    val: T,
//    Box是Rust用来显式分配堆内存的类型：在这里相当于在堆空间里申请了一块内存保存StackNode<T>.
//    如果不用Box封装，rustc编译器会报错，Rust中，rustc默认使用栈空间，但是这里的StackNode定义的时候是使用了递归的
//    数据结构，next属性的类型是StackNode<T>，而这个类型是无法确定大小的类型，都不能保存在栈空间。所以需要Box来封装，
//    这样的话next的类型就是一个只想某一块堆空间的指针，而指针是可以确定大小的，因此能够保存在栈空间。

//  为什么还需要使用Optin来封装呢，Option是Rust里面的一个抽象类型，里面包括None和Some(T)，这样就很轻松的描述了next指向
//    栈尾的时候都是在Option类型下，方便了功能实现也方便了错误处理。
    next: Option<Box<StackNode<T>>>,
}

impl<T> StackNode<T> {
    fn new(val: T) -> StackNode<T>{
        StackNode{
            val,
            next: None,
        }
    }
}

impl<T> Stack<T> {
    fn new() -> Stack<T>{
        Stack{
            top: None,
        }
    }

    fn push(&mut self, val: T){
        let mut node = StackNode::new(val);
//      这里使用了Option类型的take方法，fn take(&mut self) -> Option<T> 它会把 Option 类型的值取走，并把它的元素改为 None
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<T>{
        let val = self.top.take();
        match val {
            None => None,
            Some(mut x) => {
                self.top = x.next.take();
                Some(x.val)
            }
        }
    }
}

fn main() {
    #[derive(PartialEq, Eq, Debug)]
    struct TestStruct{
        a: i32,
    }

    let a = TestStruct{a: 5};
    let b = TestStruct{a: 9};

    let mut s = Stack::<&TestStruct>::new();
    assert_eq!(s.pop(), None);

    s.push(&a);
    s.push(&b);
    println!("{:?}", s);

    assert_eq!(s.pop(), Some(b));
    assert_eq!(s.pop(), Some(a));
    assert_eq!(s.pop(), None);
}