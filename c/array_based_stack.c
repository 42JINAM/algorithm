#include <stdlib.h>
#include <string.h>
typedef int ElementType ;

typedef struct s_node
{
	ElementType	data;
}	t_node;

typedef	struct	s_arrayStack
{
	int		capacity;
	int		top;
	t_node	*nodes;
}	t_arrayStack;

void	AS_Destroy(t_arrayStack *stack)
{
	free(stack->nodes);
	free(stack);
}

void	AS_CreateStack(t_arrayStack **stack, int capacity)
{
	(*stack) = (t_arrayStack *)malloc(sizeof(t_arrayStack));
	if (*stack == NULL)
		return ;

	(*stack)->nodes = (t_node *)malloc(sizeof(t_node) * capacity);
	if ((*stack)->nodes == NULL)
	{
		free(*stack);
		return ;
	}

	(*stack)->capacity = capacity;
	(*stack)->top = -1;
}

void	AS_Push(t_arrayStack *stack, ElementType data)
{
	if (stack->top + 1 == stack->capacity)
	{
		stack->capacity *= 1.3;
		t_node *temp = (t_node *)malloc(sizeof(t_node) * stack->capacity);
		if (temp == NULL)
		{
			AS_Destroy(stack);
			exit(1);
		}
		memmove(temp, stack->nodes, sizeof(t_node) * (stack->top + 1));
		free(stack->nodes);
		stack->nodes = temp;
	}

	stack->top ++;
	stack->nodes[stack->top].data = data;
}

ElementType	AS_Pop(t_arrayStack *stack)
{
	if (stack->top < stack->capacity * 0.7)
	{
		int new_cap = stack->capacity * 7 / 10;

		if (new_cap < 10)
			new_cap = 10;
		t_node *temp = realloc(stack->nodes, new_cap * sizeof(t_node));
		if (temp == NULL)
		{
			AS_Destroy(stack);
			exit(1);
		}
		stack->nodes = temp;
		stack->capacity = new_cap;
	}
	int	position = stack->top--;
	return stack->nodes[position].data;
}

ElementType AS_Top(t_arrayStack *stack)
{
	return stack->nodes[stack->top].data;
}

int	AS_IsEmpty(const t_arrayStack *stack)
{
	return(stack->top != -1);
}


int		AS_GetSize(const t_arrayStack *stack)
{
	return (stack->top + 1);
}

int		AS_IsFull(const t_arrayStack *stack)
{
	return (stack->capacity == AS_GetSize(stack));
}
