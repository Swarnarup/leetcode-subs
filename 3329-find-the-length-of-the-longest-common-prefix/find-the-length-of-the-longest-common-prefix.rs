use std::collections::HashMap;
impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut root = newNode();
        for x in arr1 {
            let mut crawler = &mut root;
            for i in digits(x) {
                crawler = crawler.children.entry(i).or_insert(newNode());
            }
        }
        let mut ans = 0;
        for x in arr2 {
            let mut crawler = &root;
            let mut tmp = 0;
            for i in digits(x) {
                match crawler.children.get(&i) {
                    Some(next) => {
                        crawler = next;
                        tmp += 1;
                    },
                    None => break,
                }
            }
            ans = ans.max(tmp);
        }
        ans
    }
}

struct Node {
    children: HashMap<i32, Node>
}

fn newNode() -> Node {
    Node{
        children: HashMap::new()
    }
}

fn digits(mut x: i32) -> Vec<i32> {
    let mut arr = vec![];
    while x > 0 {
        arr.push(x%10);
        x = x / 10;
    }
    arr.reverse();
    arr
}