#include <iostream>
#include <stdint.h>

extern "C" {
void foo_rs(uint32_t a, uint32_t b);
}

int main()
{
    std::cout << "Cpp Hello World" << std::endl;
    foo_rs(5, 7);
    return 0;
}