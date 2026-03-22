// 0 0 1 1 1 2 2 3 3 4
//                   1
//       1 o
//   1
//   1 2 3 4

int removeDuplicates(int* nums, int numsSize)
{
	if (numsSize <= 1) 
		return (numsSize);

	int offset = 1;
	for (int i = 1; i < numsSize; i ++)
	{
		if (nums[i] != nums[offset - 1])
		{
			nums[offset] = nums[i];
			offset ++;
		}

	}
	return offset;
}
