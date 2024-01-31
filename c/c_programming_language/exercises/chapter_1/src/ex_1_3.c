/* Print celsius to fahrenheit conversion table
 * ============================================================================
 */

#include <stdio.h>



int main() {
    float fahr, celsius;
    int lower, upper, step;

    lower = 0;
    upper = 300;
    step = 20;

    fahr = lower;

    printf("%3s %6s\n", "Fahr", "Celsius");
    while (fahr <= upper) {
        celsius = (5.0 / 9.0) * (fahr - 32);
        printf("%3.0f %6.1f\n", fahr, celsius);
        fahr = fahr + step;
    }

    return 0;
}
