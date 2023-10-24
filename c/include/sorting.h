#ifndef SORTING_H
#define SORTING_H

#include <stdlib.h>
#include "utils.h"

/**
 * @brief A simple sorting algorithm that builds the final sorted array one item
 * at a time by repeatedly picking one element and placing it at the correct
 * position. The complexity of this algorithm is O(n^2).
 * @param array A pointer to the array to be sorted.
 * @param size The size of the array.
 * @warning This function modifies the array.
 */
void insertion_sort(int *array, size_t size);

/**
 * @brief A variation of bubble sort where comparisons and swaps are performed
 * on alternating odd and even indexed elements. The complexity of this
 * algorithm is O(n^2).
 * @param array A pointer to the array to be sorted.
 * @param size The size of the array.
 * @warning This function modifies the array.
 */
void odd_even_sort(int *array, size_t size);

/**
 * @brief An improved bubble sort that uses a gap sequence to remove turtles
 * (small values near the end). The complexity of this algorithm is O(n^2).
 * @param array A pointer to the array to be sorted.
 * @param size The size of the array.
 * @warning This function modifies the array.
 */
void comb_sort(int *array, size_t size);

/**
 * @brief An optimization of bubble sort that sorts in both directions on each
 * pass through the list. The complexity of this algorithm is O(n^2).
 * @param array A pointer to the array to be sorted.
 * @param size The size of the array.
 * @warning This function modifies the array.
 */
void cocktail_sort(int *array, size_t size);

/**
 * @brief A divide and conquer algorithm that divides the array into two halves,
 * sorts them and then merges the two sorted halves. The complexity of this
 * algorithm is O(n log n).
 * @param array A pointer to the array to be sorted.
 * @param size The size of the array.
 * @warning This function modifies the array.
 */
void merge_sort(int *array, size_t size);

#endif // SORTING_H
