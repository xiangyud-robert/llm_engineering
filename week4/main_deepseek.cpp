 
#include <iostream>
#include <chrono>

double calculate(int iterations, int param1, int param2) {
    double result = 1.0;
    for (int i = 1; i <= iterations; ++i) {
        double j_minus = i * static_cast<double>(param1) - param2;
        double j_plus = i * static_cast<double>(param1) + param2;
        result -= 1.0 / j_minus;
        result += 1.0 / j_plus;
    }
    return result;
}

int main() {
    auto start_time = std::chrono::high_resolution_clock::now();
    double result = calculate(200000000, 4, 1) * 4;
    auto end_time = std::chrono::high_resolution_clock::now();

    std::cout.precision(12);
    std::cout << "Result: " << result << '\n';
    std::cout << "Execution Time: " << std::chrono::duration<double>(end_time - start_time).count() << " seconds" << std::endl;
    
    return 0;
}
