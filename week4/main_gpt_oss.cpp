#include <iostream>
#include <iomanip>
#include <chrono>

int main() {
    const int iterations = 200000000;
    double result = 0.0;
    const int param1 = 4;
    const int param2 = 1;

    auto start = std::chrono::high_resolution_clock::now();

    for (int i = 0; i < iterations; ++i) {
        int j1 = i * param1 - param2;
        int j2 = j1 + 2 * param2;
        result -= 1.0 / j1;
        result += 1.0 / j2;
    }

    auto end = std::chrono::high_resolution_clock::now();
    double elapsed = std::chrono::duration<double>(end - start).count();

    std::cout << std::setprecision(12) << result << "\n";
    std::cout << "Execution Time: " << std::setprecision(6) << elapsed << " seconds\n";

    return 0;
}
