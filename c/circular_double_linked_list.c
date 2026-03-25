#include <stdlib.h>
typedef int elementType;

typedef	struct s_node
{
	elementType		data;
	struct s_node	*prev;
	struct s_node	*next;
}	t_node;

t_node	*CDDL_CreateNode(elementType data)
{
	t_node	*newNode = (t_node *)malloc(sizeof(t_node));

	if (newNode == NULL)
		return (NULL);
	newNode->data = data;
	newNode->prev = NULL;
	newNode->next = NULL;
	return (newNode);
}

void	CDDL_DestroyNode(t_node *node)
{
	free(node);
}

void	CDDL_AppendNode(t_node **head, t_node *newNode)
{
	if (*head == NULL)
	{
		*head = newNode;
		(*head)->next = *head;
		(*head)->prev = *head;
	}
	else
	{
		t_node *tail = (*head)->prev;
		tail->next->prev = newNode;
		tail->next = newNode;
		newNode->next = (*head);
		newNode->prev = tail;
	}
}

t_node	*CDDL_GetNodeAt(const t_node *head, int location)
{
	const t_node *current = head;

	while (current != NULL && location > 0)
	{
		current = current->next;
		location --;
	}
	return ((t_node *) current);
}

void	CDDL_RemoveNode(t_node **head, t_node *remove)
{
	if ((*head) == remove)
	{
		(*head)->prev->next = remove->next;
		(*head)->next->prev = remove->prev;
		*head = remove->next;
		remove->prev = NULL;
		remove->next = NULL;
	}
	else
	{
		remove->prev->next = remove->next;
		remove->next->prev = remove->prev;
		remove->prev = NULL;
		remove->next = NULL;
	}
}

void	CDDL_InsertAfter(t_node *current, t_node *newNode)
{
	newNode->prev = current;
	newNode->next = current->next;
	if (current->next != NULL)
	{
		current->next->prev = newNode;
		current->next = newNode;
	}
}

int	CDDL_GetNodeCount(const t_node *head)
{
	unsigned int count = 0;
	const t_node *current = head;

	while (current != NULL)
	{
		current = current->next;
		count ++;
	}
	return (count);
}
