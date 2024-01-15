#include "SortAlgorithm.h"

class BogoSort: public SortAlgorithm{
    using SortAlgorithm::SortAlgorithm;
    
    public:
        void sort();
        bool isSorted(const float* array, int n);
};