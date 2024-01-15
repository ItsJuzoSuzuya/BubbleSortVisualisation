#include "BogoSort.h"
#include <iostream>
#include <algorithm>
#include <random>

void BogoSort::sort(){
    std::random_device rd;
    std::mt19937 gen(rd());

    while (!isSorted(sortArray, sortArraySize)){
        std::shuffle(sortArray, sortArray+sortArraySize, gen);
        for(int i = 0; i < sortArraySize; i++){
            std::cout << sortArray[i] << " ";
        }
        std::cout << "\n";
    }
}

bool BogoSort::isSorted(const float* array, int n){
    if (n == 1 || n == 0)
        return true;

    if(array[n-1] < array[n-2])
        return false;

    return isSorted(array, n-1);
}