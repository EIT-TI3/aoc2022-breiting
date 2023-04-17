#include <string>
#include <iostream>
#include <fstream>


using namespace std;

int day_01(const std::string input) {
    std::string in = input;
    string string_buffer;
    int elf_buffer = 0;
    int max = 0;

    for (std::string::iterator it=in.begin(); it!=in.end(); ++it)
        if (*it == '\n') {
            if (!string_buffer.empty()) {
                int val = std::stoi(string_buffer);
                string_buffer = "";
                elf_buffer += val;
            } else {
                if (elf_buffer > max) max = elf_buffer;
                elf_buffer = 0;
            }
        } else {
            string_buffer += *it;
        }
    return max;
}

int day_02(const std::string input) {
    std::string in = input;
    string string_buffer;
    int elf_buffer = 0;
    int max[3] = {0, 0, 0};

    for (std::string::iterator it=in.begin(); it!=in.end(); ++it)
        if (*it == '\n') {
            if (!string_buffer.empty()) {
                int val = std::stoi(string_buffer);
                string_buffer = "";
                elf_buffer += val;
            } else {
                if (max[0] < elf_buffer) {
                    if (max[1] < elf_buffer) {
                        if (max[2] < elf_buffer) {
                            max[0] = max[1];
                            max[1] = max[2];
                            max[2] = elf_buffer;
                        } else {
                            max[0] = max[1];
                            max[1] = elf_buffer;
                        }
                    } else {
                        max[0] = elf_buffer;
                    }
                }
                elf_buffer = 0;
            }
        } else {
            string_buffer += *it;
        }
        
    return max[0] + max[1] + max[2];
}