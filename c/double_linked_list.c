#include <stdlib.h>
typedef int elementType;

typedef	struct s_node
{
	elementType		data;
	struct s_node	*prev;
	struct s_node	*next;
}	t_node;

t_node	*DDL_CreateNode(elementType data)
{
	t_node	*newNode = (t_node *)malloc(sizeof(t_node));

	if (newNode == NULL)
		return (NULL);
	newNode->data = data;
	newNode->prev = NULL;
	newNode->next = NULL;
	return (newNode);
}

void	DDL_DestroyNode(t_node *node)
{
	free(node);
}

void	DDL_AppendNode(t_node **head, t_node *newNode)
{
	if (*head == NULL)
		*head = newNode;
	else
	{
		t_node *tail = *head;
		while (tail->next != NULL)
		{
			tail = tail->next;
		}
		tail->next = newNode;
		newNode->prev = tail;
	}
}

t_node	*DDL_GetNodeAt(const t_node *head, int location)
{
	const t_node *current = head;

	while (current != NULL && location > 0)
	{
		current = current->next;
		location --;
	}
	return ((t_node *) current);
}

void	DDL_RemoveNode(t_node **head, t_node *remove)
{
	if ((*head) == remove)
	{
		*head = remove->next;
		if (*head != NULL)
		{
			(*head)->prev = NULL;
		}
		remove->prev = NULL;
		remove->next = NULL;
	}
	else
	{
		if (remove->prev != NULL)
			remove->prev->next = remove->next;
		if (remove->next != NULL)
			remove->next->prev = remove->prev;
		remove->prev = NULL;
		remove->next = NULL;
	}
}

void	DDL_InsertAfter(t_node *current, t_node *newNode)
{
	newNode->prev = current;
	newNode->next = current->next;
	if (current->next != NULL)
	{
		current->next->prev = newNode;
		current->next = newNode;
	}
}

int	DDL_GetNodeCount(const t_node *head)
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
