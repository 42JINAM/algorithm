#ifndef CALCULATOR_H
#define CALCULATOR_H

#include "ll_based_stack.h"

#define MAX_TOKEN_LEN 4096

typedef enum e_operator
{
	LEFT_PARENTEHSIS = '(',
	RIGHT_PARENTHESIS = ')',
	PLUS = '+',
	MINUS = '-',
	MULTIPLY = '*',
	DIVIDE = '/',
	SPACE = ' ',
	OPERAND

}	t_operator;

int				IsNumber(char c);
unsigned int	GetNextToken(const char *expression, char *token, int *type);
int				IsPrior(char op1, char op2);
void			GetPostfix(char *infixExpression, char *postfixExpression);
double			Calculate(char *postfixExpression);

#endif // !CALCULATOR_H
#define CALCULATOR_H

