#include "SortAlgorithm.h"

SortAlgorithm::SortAlgorithm(const float* floatArray, int size){
    this->sortArray = new float(5);
    this->sortArraySize = size;

    for(int i = 0; i < size; i++){
        sortArray[i] = floatArray[i];
    }
}