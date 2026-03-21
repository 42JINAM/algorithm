
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



