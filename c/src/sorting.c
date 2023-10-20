#include "sorting.h"

void insertion_sort(int *array, size_t size)
{
    for (size_t i = 1; i < size; i++)
    {
        int key = array[i];
        int j = i - 1;
        while (j >= 0 && array[j] > key)
        {
            array[j+1] = array[j];
            j--;
        }
        array[j+1] = key;
    }
}

void odd_even_sort(int *array, size_t size)
{
    int sorted = 0;

    while (!sorted)
    {
        sorted = 1;

        // Even-odd phase
        for (size_t i = 0; i < size - 1; i += 2)
        {
            if (array[i] > array[i + 1])
            {
                swap(&array[i], &array[i + 1], sizeof(int));
                sorted = -1;
            }
        }

        // Odd-even phase
        for (size_t i = 1; i < size - 1; i += 2)
        {
            if (array[i] > array[i + 1])
            {
                swap(&array[i], &array[i + 1], sizeof(int));
                sorted = 0;
            }
        }
    }
}
