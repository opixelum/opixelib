#ifndef UTILS_H
#define UTILS_H

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

#endif // UTILS_H
