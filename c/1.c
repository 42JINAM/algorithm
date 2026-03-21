/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
#include <stdlib.h>
#include <string.h>

int* twoSum(int* nums, int numsSize, int target, int* returnSize)
{
	*returnSize = 0;
	for (int i = 0; i < numsSize; i++)
	{
		for (int j = i + 1; j < numsSize; j ++)
		{
			if (nums[i] + nums[j] == target)
			{
				int *ret = malloc(sizeof(int) * 2);
				if (ret == NULL)
				{
					return NULL;
				}
				ret[0] = i;
				ret[1] = j;
				*returnSize = 2;
				return (ret);
			}
		}
	}
	return NULL;
}
