#include <string.h>
typedef struct s_entry
{
	int key;
	int	idx;
	int	used;
}	t_entry;

int	hash_int(int key, int size)
{
	if (key < 0)
		key *= -1;
	return ((key * 31LL) % size);
}

int find_key(t_entry *table, int size, int key)
{
	int	idx = hash_int(key, size);
	int start = idx;

	while (table[idx].used)
	{
		if (table[idx].key == key)
			return (idx);
		idx = (idx + 1) % size;
		if (idx == start)
			break;
	}
	return (-1);
}

void insert_key(t_entry *table, int size, int key, int index)
{
	int idx = hash_int(key, size);

	while (table[idx].used)
	{
		idx = (idx + 1) % size;
	}
	table[idx].used = 1;
	table[idx].key	= key;
	table[idx].idx	= index;
}


bool containsDuplicate(int* nums, int numsSize)
{
	int size = numsSize * 2 + 1;
	t_entry	table[size];
	memset(table, 0, sizeof(table));

	for (int i = 0; i < numsSize; i++) {
		if (find_key(table, size, nums[i]) != -1)
			return (1);
		insert_key(table, size, nums[i], i);
	}
	return (0);
}
