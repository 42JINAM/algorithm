#include "ll_based_stack.h"

void	LLS_CreateStack(t_stack **stack)
{
	(*stack) = malloc(sizeof(t_stack));
	if (*stack == NULL)
		exit(1);
	(*stack)->list = NULL;
	(*stack)->top = NULL;
}

t_node	*LLS_CreateNode(const char *data)
{
	size_t i = 0;
	t_node *new_node = malloc(sizeof(t_node));

	if (new_node == NULL)
		return NULL;

	while(data[i] != '\0')
		i++;

	char *tmp = malloc(i + 1);
	if (tmp == NULL)
	{
		free(new_node);
		return NULL;
	}
	for (int j = 0; j < i; j++)
		((char *)tmp)[j] = data[j];
	((char *)tmp)[i] = '\0';
	new_node->data = tmp;
	new_node->next = NULL;
	new_node->prev = NULL;

	return (new_node);
}

int	LLS_IsEmpty(const t_stack *stack)
{
	return (stack->top == NULL);
}

void	LLS_Push(t_stack *stack, t_node *node)
{
	if (stack->list == NULL)
	{
		stack->list = node;
	}
	else
	{
		stack->top->next = node;
		node->prev = stack->top;
	}
	stack->top = node;

}

t_node	*LLS_Pop(t_stack *stack)
{
	if (stack->top == NULL)
		return (NULL);
	t_node	*current_top = stack->top;
	stack->top = current_top->prev;
	if (stack->top != NULL)
	{
		stack->top->next = NULL;
	}
	return (current_top);
}

void	LLS_DestroyNode(t_node *node)
{
	free(node->data);
	free(node);
}

void	LLS_DestroyStack(t_stack *stack)
{
	while (!LLS_IsEmpty(stack))
	{
		t_node *popped = LLS_Pop(stack);
		LLS_DestroyNode(popped);
	}
	free(stack);
}

int	LLS_GetSize(const t_stack *stack)
{
	int				cnt = 0;
	const t_node	*current = stack->list;

	while (current)
	{
		current = current->next;
		cnt ++;
	}
	return (cnt);
}

t_node *LLS_Top(t_stack *stack)
{
	return (stack->top);
}
