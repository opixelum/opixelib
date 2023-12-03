#include "utils.h"

void swap(void *a, void *b, size_t size)
{
    char *temp = malloc(size * sizeof *temp);
    if (!temp)
    {
        fprintf(stderr, "ERROR: utils: swap: malloc failed\n");
        exit(EXIT_FAILURE);
    }
    memcpy(temp, a, size);
    memcpy(a, b, size);
    memcpy(b, temp, size);
    free(temp);
}

int *merge_sorted_array
(
    int *array_a,
    size_t size_a,
    int *array_b,
    size_t size_b
) {
    int *merge_array = malloc((size_a + size_b) * sizeof *merge_array);
    size_t i = 0; // Tracks array_a index
    size_t j = 0; // Tracks array_b index
    size_t k = 0; // Tracks merged_array index

    // Merge arrays by picking the smallest element until one is empty
    while (i < size_a && j < size_b)
    {
        if (array_a[i] < array_b[j]) merge_array[k++] = array_a[i++];
        else merge_array[k++] = array_b[j++];
    }

    // Copy remaining elements of array_a or array_b
    while (i < size_a) merge_array[k++] = array_a[i++];
    while (j < size_b) merge_array[k++] = array_b[j++];

    return merge_array;
}
