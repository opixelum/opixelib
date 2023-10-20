#ifndef SORTING_H
#define SORTING_H

#include <stdlib.h>
#include "utils.h"

/**
 * @brief Complexity: O(n^2)
 * @param array Pointer to the array to be sorted
 * @param size Size of the array
 * @warning Modifies the array
 */
void insertion_sort(int *array, size_t size);

/**
 * @brief Complexity: O(n^2)
 * @param array Pointer to the array to be sorted
 * @param size Size of the array
 * @warning Modifies the array
 */
void odd_even_sort(int *array, size_t size);

/**
 * @brief Complexity: O(n^2)
 * @param array Pointer to the array to be sorted
 * @param size Size of the array
 * @warning Modifies the array
 */
void comb_sort(int *array, size_t size);

#endif // SORTING_H
