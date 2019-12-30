//优先队列，普通的队列是一种先进先出的数据结构，元素在队尾追加，而从队头删除。在优先队列中，元素被赋予优先级，当访问元素时，
// 具有最高优先级的元素最先被删除。优先队列具有最高级先出的特征

#[derive(Debug)]
//where之后的表明PriorityQueue存储的泛型T是满足ParialOrd和Clone trait约束的，意味着泛型T是可排序和克隆的
struct PriorityQueue<T> where T: PartialOrd + Clone{
    pq: Vec<T>,
}

impl<T> PriorityQueue<T> where T: PartialOrd + Clone{
    fn new() -> PriorityQueue<T>{
        PriorityQueue{
            pq: Vec::new(),
        }
    }

    fn len(&self) -> usize{
        self.len()
    }

    fn is_empty(&self) -> bool{
        self.pq.len() == 0
    }

    fn insert(&mut self, value: T) {
        self.pq.push(value);
    }

    fn max(&self) -> Option<T>{
        if self.is_empty(){return None}
        let max = self.max_index();
        Some(self.pq[max].clone())
    }

    fn min(&self) -> Option<T>{
        if self.is_empty(){return None}
        let min = self.min_index();
        Some(self.pq[min].clone())
    }

    fn delete_max(&mut self) -> Option<T>{
        if self.is_empty(){return None}
        let max = self.max_index();
        Some(self.pq.remove(max).clone())
    }

    fn delete_min(&mut self) -> Option<T>{
        if self.is_empty(){return None}
        let min = self.min_index();
        Some(self.pq.remove(min).clone())
    }

    fn max_index(&self) -> usize{
        let mut max = 0;
        for i in 1..self.pq.len() - 1{
            if self.pq[max] < self.pq[i]{
                max = i;
            }
        }
        max
    }

    fn min_index(&self) -> usize{
        let mut min = 0;
        for i in 1..self.pq.len() - 1{
            if self.pq[i] < self.pq[i + 1]{
                min = i;
            }
        }
        min
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_keep_min(){
        let mut pq = PriorityQueue::new();
        pq.insert(3);
        pq.insert(2);
        pq.insert(1);
        pq.insert(4);
        assert_eq!(pq.min().unwrap(), 1);
    }

    #[test]
    fn test_keep_max(){
        let mut pq = PriorityQueue::new();
        pq.insert(3);
        pq.insert(5);
        pq.insert(8);
        pq.insert(1);
        assert_eq!(pq.max().unwrap(), 8);
    }

    #[test]
    fn test_is_empty(){
        let mut pq = PriorityQueue::new();
        assert_eq!(pq.is_empty(), true);
        pq.insert(3);
        pq.insert(3);
        pq.insert(3);
        pq.insert(3);
        assert_eq!(pq.is_empty(), false);
    }

    #[test]
    fn test_len(){
        let mut pq = PriorityQueue::new();
        pq.insert(3);
        pq.insert(4);
        pq.insert(2);
        pq.insert(5);
        pq.insert(9);
        assert_eq!(pq.len(), 5);
    }

    #[test]
    fn test_delete_min(){
        let mut pq = PriorityQueue::new();
        pq.insert(3);
        pq.insert(4);
        pq.insert(2);
        pq.insert(5);
        pq.insert(9);
        assert_eq!(pq.len(), 5);
        assert!(pq.delete_min().unwrap() == 1);
    }

    #[test]
    fn test_delete_max(){
        let mut pq = PriorityQueue::new();
        pq.insert(2);
        pq.insert(10);
        pq.insert(1);
        pq.insert(6);
        pq.insert(3);
        assert!(pq.len() == 5);
    }
}