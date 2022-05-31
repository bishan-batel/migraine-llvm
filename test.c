#include "stdlib.h"

#define StackNodeSize sizeof(struct StackNode)
struct StackNode {
  struct StackNode *next;
  struct StackNode *prev;
  int size;
  int *buffer;
};

struct StackNode *curr;

void push_stack(int size) {
  struct StackNode* prevNext = curr->next;
	curr->next = malloc(StackNodeSize);
  curr->next->next = NULL;
  curr->next->size = size;
  curr->next->buffer = calloc(0, 2);
}

int main() {
  curr->next = NULL;
  curr->prev = NULL;
  curr->size = 1;
	curr->buffer = calloc(0, 1);

	// pushing a new stack tape
	return 0;
}

