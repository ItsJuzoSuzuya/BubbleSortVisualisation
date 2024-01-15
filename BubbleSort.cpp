#include <chrono>
#include <iostream>
#include "BubbleSort.h"

void BubbleSort::sort(){
    typedef std::chrono::milliseconds ms;
    typedef std::chrono::duration<float> fsec;

    auto start = std::chrono::system_clock::now();
    bool listSorted = false;

    int temp;

    while (!listSorted){
        bool check = true;
        for (int i = 0; i < sortArraySize - 1; i++){
            if (sortArray[i] > sortArray[i+1]){
                std::swap(sortArray[i], sortArray[i+1]);
                check = false;

                for(int i = 0; i < sortArraySize; i++){
                    std::cout << sortArray[i] << " ";
                }
                std::cout << "\n";
            }    
        }
        if (check) listSorted = true;
    }

    auto end = std::chrono::system_clock::now();
    fsec time = end - start;
    ms d = std::chrono::duration_cast<ms>(time);
    
    std::cout << d.count() << "ms\n";
}
