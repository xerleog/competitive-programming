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
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut ans = vec![];
        let mut temp = head.unwrap();
        while temp.next !=None
        {
            ans.push(temp.val);
            temp = temp.next.unwrap();
        }
        ans.push(temp.val);
        ans.into_iter().rev().fold((0,1),|(a,b),x| (a+(b*x),b*2)).0
    }
}
