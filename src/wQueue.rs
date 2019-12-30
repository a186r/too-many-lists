//队列是一种特殊的线性表，特殊之处在于它只允许在表的前端进行删除操作，而在表的后端进行插入操作，和栈一样，队列是一种受限制的
//线性表。进行插入操作的端称为队尾，进行删除操作的端称为队头。队列中没有元素时，称为空队列。
#[derive(Debug)]
struct Queue<T>{
    qdata: Vec<T>,
}

impl <T> Queue<T> {
    fn new() -> Queue<T>{
        Queue{
            qdata: Vec::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.qdata.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        let qdata = self.qdata.remove(0);
        match qdata {
            None => None,
            Some(mut x) => {
                Some(x)
            },
            _ => {}
        }
    }
}