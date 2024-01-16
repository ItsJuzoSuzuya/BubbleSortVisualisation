#include "SortAlgorithm.h"

class MergeSort: public SortAlgorithm {
    using SortAlgorithm::SortAlgorithm;

    private:
        void mergeSort(float* array, int begin, int end);
        void merge(float* array, int begin, int middle, int end);
    
    public:
        void sort();
};