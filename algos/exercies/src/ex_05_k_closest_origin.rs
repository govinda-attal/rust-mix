use std::cmp::Reverse;

#[derive(Debug, PartialEq)]
struct Point {
    distance: f32,
    point: Vec<i32>,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Point {}

struct Heap<T: Ord> {
    heap: Vec<T>,
}

impl<T: Ord> Heap<T> {
    fn new() -> Self {
        Self { heap: Vec::new() }
    }

    fn push(&mut self, val: T) {
        self.heap.push(val);
        self.bubble_up();
    }

    fn pop(&mut self) -> Option<T> {
        let len = self.heap.len();
        if len == 1 {
            self.heap.pop()
        } else if len == 0 {
            None
        } else {
            self.heap.swap(0, len - 1);
            let max = self.heap.pop();
            self.bubble_down(0);
            max
        }
    }

    fn bubble_up(&mut self) {
        let i = self.heap.len() - 1;

        while i > 0 {
            let elem = &self.heap[i];
            let pi = (i - 1) / 2;
            let parent = &self.heap[pi];
            if parent > elem {
                break;
            }
            self.heap.swap(pi, i)
        }
    }

    fn bubble_down(&mut self, i: usize) {
        let l = i * 2 + 1;
        let r = i * 2 + 2;
        let mut largest = i;
        let len = self.heap.len();
        if l < len && self.heap[l] > self.heap[largest] {
            largest = l;
        }
        if r < len && self.heap[r] > self.heap[largest] {
            largest = r;
        }
        if largest != i {
            self.heap.swap(i, largest);
            self.bubble_down(largest);
        }
    }
}

fn k_nearest(points: Vec<Vec<i32>>, k: usize) -> Vec<Vec<i32>> {
    let mut heap = Heap::new();

    for point in points {
        let tmp = point.iter().map(|p| p.pow(2)).sum::<i32>();
        let point = Point {
            distance: (tmp as f32).sqrt(),
            point,
        };
        heap.push(Reverse(point));
    }

    let mut results = vec![];

    for i in 0..k {
        if let Some(Reverse(v)) = heap.pop() {
            results.push(v.point);
        } else {
            break;
        }
    }
    results
}
#[cfg(test)]
mod tests {
    use super::Heap;

    #[test]
    fn validate_heap() {
        let mut h = Heap::new();

        h.push(10);
        h.push(5);
        h.push(3);
        h.push(6);
        h.push(7);

        assert_eq!(h.pop(), Some(10));
        assert_eq!(h.pop(), Some(7));
        assert_eq!(h.pop(), Some(6));
        assert_eq!(h.pop(), Some(5));
        assert_eq!(h.pop(), Some(3));
        assert_eq!(h.pop(), None);
    }
}
