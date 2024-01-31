/* Make the program print words from input on separate lines
 * ============================================================================
 */

#include <stdio.h>


int main()
{
    int c = 0;
    char nonletter_count = 0;

    while ((c = getchar()) != EOF) {
        if ((c >= 97 && c <= 122) || (c >= 65 && c <= 90)) {
            putchar(c);
            nonletter_count = 0;
        }
        else if (nonletter_count < 1) {
            nonletter_count = 1;
            printf("\n----------------\n");
        }
    }

    return 0;
}