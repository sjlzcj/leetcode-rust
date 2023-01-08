// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // solution1(l1, l2)
    solution2(l1, l2)
}

pub fn solution2(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;
    let mut tmp = (l1, l2, 0, 0); // l1, l2, carry, sum
    loop {
        tmp = match tmp {
            (None, None, 0, _) => break,
            (None, None, _carry, _) => (None, None, 0, 1),
            (Some(l), None, carry, _) | (None, Some(l), carry, _) if l.val + carry >= 10 => {
                (l.next, None, 1, l.val + carry - 10)
            }
            (Some(l), None, carry, _) | (None, Some(l), carry, _) => {
                (l.next, None, 0, l.val + carry)
            }
            (Some(v1), Some(v2), carry, _) if v1.val + v2.val + carry >= 10 => {
                (v1.next, v2.next, 1, v1.val + v2.val + carry - 10)
            }
            (Some(v1), Some(v2), carry, _) => (v1.next, v2.next, 0, v1.val + v2.val + carry),
        };
        *tail = Some(Box::new(ListNode::new(tmp.3)));
        tail = &mut tail.as_mut().unwrap().next;
    }
    head
}

fn _solution1(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    _carried(l1, l2, 0)
}

pub fn _carried(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    mut carry: i32,
) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() && carry == 0 {
        None
    } else {
        Some(Box::new(ListNode {
            next: _carried(
                l1.and_then(|x| {
                    carry += x.val;
                    x.next
                }),
                l2.and_then(|x| {
                    carry += x.val;
                    x.next
                }),
                carry / 10,
            ),
            val: carry % 10,
        }))
    }
}
