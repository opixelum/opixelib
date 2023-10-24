#ifndef SORTING_H
#define SORTING_H

#include <stdlib.h>
#include "utils.h"

/**
 * @brief Simple sorting algorithm that builds the final sorted array one item
 * at a time by repeatedly picking one element and placing it at the correct
 * position. Complexity: O(n^2)
 * @param array Pointer to the array to be sorted
 * @param size Size of the array
 * @warning This function modifies the array
 */
void insertion_sort(int *array, size_t size);

/**
 * @brief Variation of bubble sort where comparisons and swaps are performed on
 * alternating odd and even indexed elements. Complexity: O(n^2)
 * @param array Pointer to the array to be sorted
 * @param size Size of the array
 * @warning This function modifies the array
 */
void odd_even_sort(int *array, size_t size);

/**
 * @brief Improved bubble sort that uses a gap sequence to remove turtles (small
 * values near the end). Complexity: O(n^2)
 * @param array Pointer to the array to be sorted
 * @param size Size of the array
 * @warning This function modifies the array
 */
void comb_sort(int *array, size_t size);

/**
 * @brief Optimization of bubble sort that sorts in both directions on each pass
 * through the list. Complexity: O(n^2)
 * @param array Pointer to the array to be sorted
 * @param size Size of the array
 * @warning This function modifies the array
 */
void cocktail_sort(int *array, size_t size);

#endif // SORTING_H
