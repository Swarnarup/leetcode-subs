// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // there's no provision for slow and fast pointers...
        // hence go with counting number..
        let mut len = 0;
        let mut crawler = &head;
        while let Some(nd) = crawler {
            crawler = &nd.next;
            len += 1;
        }
        let mut crawler = &mut head;
        for _ in 0..(len/2) {
            crawler = &mut crawler.as_mut().unwrap().next;
        }
        *crawler = (*crawler).as_mut().unwrap().next.take();
        head
    }
}