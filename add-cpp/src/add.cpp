#include <iostream>

#ifdef __cplusplus
extern "C" {
#endif

int add(int a, int b) {
    return a + b;
}

int print_add(int a, int b) {
    int result = add(a, b);
    std::cout << "The result of add is: " << result << std::endl;
    return result;
}

#ifdef __cplusplus
}
#endif