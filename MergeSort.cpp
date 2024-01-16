#include "MergeSort.h"
#include <iostream>

void MergeSort::sort(){
    for(int i = 0; i < sortArraySize; i++){
        std::cout << sortArray[i] << " ";
    }
    std::cout << "\n";

    mergeSort(sortArray, 0, sortArraySize -1);
}

void MergeSort::mergeSort(float* array, int begin, int end){
    if(begin < end){

        int middle = (begin+end)/2;

        mergeSort(array, begin, middle);
        mergeSort(array, middle+1, end);
        merge(array, begin, middle, end);
    }
}

void MergeSort::merge(float* array, int begin, int middle, int end){
    int leftArraySize = middle - begin + 1;
    int rightArraySize = end - middle;

    float* leftArray = new float[leftArraySize];
    float* rightArray = new float[rightArraySize];

    for(int i = 0; i < leftArraySize; i++){
        leftArray[i] = array[begin + i];
    }

    for(int j = 0; j < rightArraySize; j++){
        rightArray[j] = array[middle + 1 + j];
    }

    int leftArrayIndex = 0;
    int rightArrayIndex = 0;
    int arrayIndex = begin;

    while (leftArrayIndex < leftArraySize && rightArrayIndex < rightArraySize){
        if (leftArray[leftArrayIndex] <= rightArray[rightArrayIndex]){
            array[arrayIndex] = leftArray[leftArrayIndex];
            leftArrayIndex++;
        } else {
            array[arrayIndex] = rightArray[rightArrayIndex];
            rightArrayIndex++;
        }
        arrayIndex++;
    }

    while (leftArrayIndex < leftArraySize){
        array[arrayIndex] = leftArray[leftArrayIndex];
        leftArrayIndex++;
        arrayIndex++;
    }

    while (rightArrayIndex < rightArraySize){
        array[arrayIndex] = rightArray[rightArrayIndex];
        rightArrayIndex++;
        arrayIndex++;
    }

    for(int i = 0; i < sortArraySize; i++){
        std::cout << array[i] << " ";
    }
    std::cout << "\n";

    delete[] leftArray;
    delete[] rightArray;
}