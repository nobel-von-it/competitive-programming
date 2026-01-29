#include <stdio.h>
struct ListNode {
  int val;
  struct ListNode *next;
};

struct ListNode *addTwoNumbers(struct ListNode *l1, struct ListNode *l2) {
  struct ListNode *res;
  int debug = l1->val;
  int mlt = 1;
  while (l1->next) {
    printf("number is %d", debug);
    debug += l1->val * mlt;
    l1 = l1->next;
    mlt *= 10;
  }

  return res;
}

int main() {
  struct ListNode *l1, *l2;
  l2->val = 6;
  l1->val = 4;
  l1->next = l2;
  addTwoNumbers(l1, l2);
}
