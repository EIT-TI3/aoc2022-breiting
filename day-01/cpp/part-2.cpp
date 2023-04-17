#include <iostream>
#include "lib.h"
#include <fstream>

using namespace std;

int main() {
    std::ifstream file ("../input.txt");
    std::string input( (std::istreambuf_iterator<char>(file) ),
                       (std::istreambuf_iterator<char>()    ) );

    int day2 = day_02(input);
    return 0;
}