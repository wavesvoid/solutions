#include <stdio.h>



int main() {
    float fahr, celsius;
    int lower, upper, step;

    lower = 0;
    upper = 300;
    step = 20;

    celsius = lower;

    printf("%3s %6s\n", "Fahr", "Celsius");
    while (celsius <= upper) {
        fahr = 9.0 / 5 * celsius + 32;
        printf("%3.0f %6.1f\n", fahr, celsius);
        celsius = celsius + step;
    }
}
