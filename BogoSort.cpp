#include "BogoSort.h"
#include <iostream>
#include <algorithm>
#include <random>

void BogoSort::sort(){
    std::random_device rd;
    std::mt19937 gen(rd());

    if(!isSorted(sortArray, sortArraySize)){
        std::shuffle(sortArray, sortArray+sortArraySize, gen);
    }
}

bool BogoSort::isSorted(const float* array, int n){
    if (n == 1 || n == 0)
        return 1;

    if(array[n-1] < array[n-2])
        return 0;

    return isSorted(array, n);

}