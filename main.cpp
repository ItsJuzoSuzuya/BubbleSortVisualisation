#include "BogoSort.h"

int main(int argc, char *argv[]){
    float list1[5] = {5,2,3,1,4}; 
    
    BogoSort algorithm = BogoSort(list, 5);
    algorithm.sort();



    return 0;
}