# GOAL
- Understand the concept of lists
- Understand the difference between lists and arrays
- Understand the linked lists
- Understand the double linked lists
- Understand circle linked lists

# List ADT

## The concept

This data structure contains 'nodes'. 

The first node is called 'head' and the last one is 'tail'.

The length of a list is the number of nodes from head to tail.

We have to implement several ADT functions, like insert, remove, getAt ... 

## Array VS list comparison

Arrays are easier to use, but their size is fixed (we must specify the size at creation time and cannot easily resize them).
So we can't use it for dynamic inputs.

# Single Linked list

Linked list is a linear data structure where elements are connected using pointers.
Each node contains data and a pointer to the next node.

```c
typedef int ElementType;

typedef struct Node
{
  ElementType data;
  struct Node* next;
}

```

head -> [data | next] -> [data | next] -> [data | NULL]

- dynamic size
- O(1) insert/delete at known position
- O(n) random access (must traverse)
- Extra memory for pointers

## disadvantages 
- It requires extra memory for next node pointers 
- We can not access the specific node directly (no random access). 

## advantages 
- Adding and deleting nodes are easy and fast (o(1)) at known position .
- the operation getting the next node is free.
