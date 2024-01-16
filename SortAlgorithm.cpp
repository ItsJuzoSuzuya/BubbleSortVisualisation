#include "SortAlgorithm.h"

SortAlgorithm::SortAlgorithm(const float* floatArray, int size){
    this->sortArray = new float[size];
    this->sortArraySize = size;

    for(int i = 0; i < sortArraySize; i++){
        sortArray[i] = floatArray[i];
    }
}

