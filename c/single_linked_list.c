#include <stdlib.h>
typedef int ElementType;

typedef struct s_node
{
	ElementType		data;
	struct s_node	*next;
}	t_node;

/* What is the difference between sizeof(Node) and sizeof(Node *) ? */

/*
 * sizeof(Node) is the size of entire structure. Node * means it's pointer. the pointer size is 8 bytes. 
 * So sizeof(Node *) is 8 bytes.
 *
 * */

t_node*	SSL_CreateNode(ElementType	NewData)
{
	t_node	*newNode = (t_node *)malloc(sizeof(t_node));
	if (!newNode)
		return (NULL);

	newNode->data = NewData;
	newNode->next = NULL;

	return (newNode);
}

void	SSL_DestroyNode(t_node *node)
{
	free(node);
}

t_node	*SSL_getTail(t_node *head)
{
	if (head == NULL)
		return (NULL);
	t_node *tail = head;	

	while (tail->next != NULL)
	{
		tail = tail->next;
	}
	return(tail);
}

void	SSL_AppendNode(t_node **head, t_node *node)
{
	if ((*head) == NULL)
	{
		*head = node;
	}
	else 
	{
		t_node	*tail = SSL_getTail(*head);
		tail->next = node;
		node->next = NULL;
	}
}

t_node	*SLL_GetNodeAt(t_node* head, int location)
{
	t_node	*current = head;

	while (current != NULL && (location > 0))
	{
		current = current->next;
		location --;
	}
	return (current);
}

void	SSL_RemoveNode(t_node **head, t_node *remove)
{
	if ((*head) == remove)
	{
		*head = remove->next;
	}
	else
	{
		t_node	*current = *head;
		while (current != NULL && current->next != remove)
		{
			current = current->next;
		}
		if (current != NULL)
			current->next = remove->next;
	}
}

/* Why we can use const t_node * in here.
 	* the const pointer means we cannot modify the data pointed to by the pointer.
 	* but we can modify the pointer itself.
*/

int	SSL_GetNodeCount(const t_node* head)
{
	int				count = 0;
	const t_node	*current = head;

	while (current != NULL)
	{
		current = current->next;
		count ++;
	}
	return (count);
}
