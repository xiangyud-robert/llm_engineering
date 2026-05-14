#include <cstdio>
#include <chrono>

static inline double calculate(long iterations, int param1, int param2) {
    double r = 1.0;

    int j1 = param1 - param2;
    int j2 = param1 + param2;

    const int step = param1;

    long i = 0;
    const long unroll = 4;
    long limit = iterations - (iterations % unroll);

    for (; i < limit; i += unroll) {
        r -= 1.0 / (double)j1;
        asm volatile("" : "+r"(r) :: "memory");
        r += 1.0 / (double)j2;
        j1 += step; j2 += step;

        r -= 1.0 / (double)j1;
        asm volatile("" : "+r"(r) :: "memory");
        r += 1.0 / (double)j2;
        j1 += step; j2 += step;

        r -= 1.0 / (double)j1;
        asm volatile("" : "+r"(r) :: "memory");
        r += 1.0 / (double)j2;
        j1 += step; j2 += step;

        r -= 1.0 / (double)j1;
        asm volatile("" : "+r"(r) :: "memory");
        r += 1.0 / (double)j2;
        j1 += step; j2 += step;
    }

    for (; i < iterations; ++i) {
        r -= 1.0 / (double)j1;
        asm volatile("" : "+r"(r) :: "memory");
        r += 1.0 / (double)j2;
        j1 += step; j2 += step;
    }

    return r;
}

int main() {
    using clock_t = std::chrono::steady_clock;
    auto start = clock_t::now();

    double result = calculate(200000000L, 4, 1) * 4.0;

    auto end = clock_t::now();
    double seconds = std::chrono::duration_cast<std::chrono::duration<double>>(end - start).count();

    std::printf("Result: %.12f\n", result);
    std::printf("Execution Time: %.6f seconds\n", seconds);
    return 0;
}