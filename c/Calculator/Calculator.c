#include "Calculator.h"
#include "ll_based_stack.h"
#include <_stdlib.h>
#include <string.h>

int				IsNumber(char c)
{
	return ((c >= '0' && c <= '9') || (c == '.'));
}

unsigned int	GetNextToken(const char *expression, char *token, int *type)
{
	if (!*expression) return NULL;

	unsigned int i = 0;

	while (expression[i] == ' ')
		i++;

	if (IsNumber(expression[i]))
	{
		unsigned int len = 0;
		*type = OPERAND;
		while (expression[i] && IsNumber(expression[i]) && len < MAX_TOKEN_LEN - 1)
		{
			token[len ++] = expression[i];
			i ++;
		}
		token[len] = '\0';
	} else
	{
		*type = (int) expression[i];
		token[0] = expression[0];
		token[1] = '\0';
		i++;
	}
	return (i);
}

int	GetPriority(char operator, int inStack)
{
	int	priority = -1;

	switch (operator)
	{
		case LEFT_PARENTEHSIS:
			if (inStack)
				priority = 3;
			else
				priority = 0;
			break ;
		case MULTIPLY:
		case DIVIDE:
			priority = 1;
			break ;
		case PLUS:
		case MINUS:
			priority = 2;
			break;
	}
	return priority;
}

int				IsPrior(char op1, char op2)
{
	return (GetPriority(op1, 1) > GetPriority(op2, 0));
}

void			GetPostfix(char *infixExpression, char *postfixExpression)
{
	t_stack			*stack;
	char			token[32];
	int				type = -1;
	unsigned int	position = 0;
	unsigned int	length =  strlen(infixExpression);

	LLS_CreateStack(&stack);

	while (position < length)
	{
		position += GetNextToken(&infixExpression[position], token, &type);
		if (type == OPERAND)
		{
			strcat(postfixExpression, token);
			strcat(postfixExpression, " ");
		}
		else if (type == RIGHT_PARENTHESIS)
		{
			while (!LLS_IsEmpty(stack))
			{
				t_node *popped = LLS_Pop(stack);
				if (popped->data[0] == LEFT_PARENTEHSIS)
				{
					LLS_DestroyNode(popped);
					break;
				}
				else
				{
					strcat(postfixExpression, popped->data);
					LLS_DestroyNode(popped);
				}
			}
		}
		else
		{
			while (!LLS_IsEmpty(stack) && !IsPrior(LLS_Top(stack)->data[0], token[0]))
			{
				t_node	*popped = LLS_Pop(stack);

				if (popped->data[0] != LEFT_PARENTEHSIS)
					strcat(postfixExpression, popped->data);
				LLS_DestroyNode(popped);
			}
			LLS_Push(stack, LLS_CreateNode(token));
		}
	}
	while (!LLS_IsEmpty(stack))
	{
		t_node	*popped = LLS_Pop(stack);

		if (popped->data[0] != LEFT_PARENTEHSIS)
		{
			strcat(postfixExpression, popped->data);
		}
		LLS_DestroyNode(popped);
	}
	LLS_DestroyStack(stack);
}

double			Calculate(char *postfixExpression)
{
	t_stack			*stack;
	t_node			*result_node;

	double			result; 
	char			token[32];
	int				type = -1;
	unsigned int	read = 0;
	unsigned int	length = strlen(postfixExpression);

	LLS_CreateStack(&stack);
	while (read < length)
	{
		read += GetNextToken(&postfixExpression[read], token, &type);
		if (type == SPACE)
			continue;
		if (type == OPERAND)
		{
			t_node	*new_node = LLS_CreateNode(token);
			LLS_Push(stack, new_node);
		}
		else
		{
			char	result_string[32];
			double	op1, op2, temp_result;
			t_node	*op_node;

			op_node	= LLS_Pop(stack);
			op2 = atof(op_node->data);
			LLS_DestroyNode(op_node);

			op_node	= LLS_Pop(stack);
			op1 = atof(op_node->data);
			LLS_DestroyNode(op_node);

			switch (type)
			{
				case PLUS:
					temp_result = op1 + op2;
					break;
				case MINUS:
					temp_result = op1 - op2;
					break;
				case MULTIPLY:
					temp_result = op1 * op2;
					break;
				case DIVIDE:
					temp_result = op1 / op2;
					break;
			}
			gcvt(temp_result, 10, result_string);
			LLS_Push(stack, LLS_CreateNode(result_string));
		}
	}
	result_node = LLS_Pop(stack);
	result = atof(result_node->data);
	LLS_DestroyNode(result_node);
	LLS_DestroyStack(stack);
	return (result);
}

