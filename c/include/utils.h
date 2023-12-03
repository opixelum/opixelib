#ifndef UTILS_H
#define UTILS_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/**
 * @brief Swaps two variables of any type.
 * @param a A pointer to the first variable.
 * @param b A pointer to the second variable.
 * @param size The size of the variables.
 * @warning Both variables need to be of the same type.
 */
void swap(void *a, void *b, size_t size);

/**
 * @brief Merges two sorted arrays into one sorted array.
 * @param array_a A pointer to the first array to merge.
 * @param size_a The size of the first array.
 * @param array_b A pointer to the second array to merge.
 * @param size_b The size of the second array.
 * @return A pointer to the sorted array.
 * @warning This function doesn't modify passed arrays, but creates another one.
 */
int *merge_sorted_array
(
    int *array_a,
    size_t size_a,
    int *array_b,
    size_t size_b
);

#endif // UTILS_H
