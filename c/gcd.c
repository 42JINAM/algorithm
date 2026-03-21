// gcd is the great number can devide both input numbers. 

int	gcd(int a, int b)
{
	if (b == 0)
		return (a);
	return (gcd(b, a % b));
}
