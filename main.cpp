#include "MergeSort.h"

int main(int argc, char *argv[]){
    float list[5] = {5,2,3,1,4}; 
    
    MergeSort algorithm(list, 5);
    algorithm.sort();

    return 0;
}