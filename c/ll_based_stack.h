#ifndef LL_BASED_STACK
# define LL_BASED_STACK

#include <stdlib.h>
typedef struct s_node
{
	char			*data;
	struct s_node	*next;
	struct s_node	*prev;
}	t_node;

typedef struct s_stack {
	t_node	*list;
	t_node	*top;
}	t_stack;

void	LLS_CreateStack(t_stack **stack);
t_node	*LLS_CreateNode(const char *data);
int		LLS_IsEmpty(const t_stack *stack);
void	LLS_Push(t_stack *stack, t_node *node);

t_node	*LLS_Pop(t_stack *stack);

void	LLS_DestroyNode(t_node *node);
void	LLS_DestroyStack(t_stack *stack);
int		LLS_GetSize(const t_stack *stack);
t_node *LLS_Top(t_stack *stack);
#endif // !LL_BASED_STACK
# define LL_BASED_STACK
