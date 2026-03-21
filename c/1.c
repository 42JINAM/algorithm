/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
#include <stdlib.h>
#include <string.h>

typedef struct s_entry
{
	int	key;
	int	idx;
	int	used;
}	t_entry;

int hash_int(int key, int size)
{
	if (key < 0)
		key *= -1;
	return (key % size);
}

static int find_key(t_entry *table, int size, int key)
{
	int idx = hash_int(key, size);
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

static void	insert_key(t_entry *table, int size, int key, int index)
{
	int idx = hash_int(key, size);

	while (table[idx].used)
	{
		idx = (idx + 1) % size;
	}
	table[idx].used = 1;
	table[idx].key = key;
	table[idx].idx = index;
}



int* twoSum(int* nums, int numsSize, int target, int* returnSize)
{
	*returnSize = 0;
	int size = numsSize * 2 + 1;
	t_entry *table = calloc(size, sizeof(t_entry));
	if (!table)
		return NULL;

	for (int i = 0; i < numsSize; i++)
	{
		int need = target - nums[i];
		int pos = find_key(table, size, need);
		if (pos == -1)
			insert_key(table, size, nums[i], i);
		else {
			int *ret = malloc(sizeof(int) * 2);
			if (!ret)
			{
				free(table);
				return (NULL);
			}
			ret[0] = table[pos].idx;
			ret[1] = i;
			*returnSize = 2;
			free(table);
			return (ret);
	  	}
	}
	return NULL;
}
