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

void comb_sort(int *array, size_t size)
{
    unsigned char is_sorted = 0;
    size_t gap = size;

    while (gap > 1 || !is_sorted)
    {
        is_sorted = 1;
        gap = (size_t) ((double) gap / 1.3);
        if (gap < 1) gap = 1;

        for (int i = 0; i < size - gap; i++)
        {
            if (array[i] > array[i + gap])
            {
                swap(&array[i], &array[i + gap], sizeof(array[i]));
                is_sorted = 0;
            }
        }
    }
}