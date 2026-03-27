#include <stdlib.h>
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
	stack->top ++;
	stack->nodes[stack->top].data = data;
}

ElementType	AS_Pop(t_arrayStack *stack)
{
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

void	AS_Destroy(t_arrayStack *stack)
{
	free(stack->nodes);
	free(stack);
}

int		AS_GetSize(const t_arrayStack *stack)
{
	return (stack->top + 1);
}

int		AS_IsFull(const t_arrayStack *stack)
{
	return (stack->capacity == AS_GetSize(stack));
}
