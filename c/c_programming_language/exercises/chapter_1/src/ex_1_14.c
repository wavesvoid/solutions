/* Write a program that prints a histogram
 * describing how often each separate ASCII symbol was met in a user input
 */

#include <stdio.h>


// One more for other, non-ASCII symbols
#define MAX_SYMBOLS_COUNT 129
#define HISTOGRAM_LENGTH 10



int main() {
    unsigned symbol_counters[MAX_SYMBOLS_COUNT] = {0};
    short tmp_char = 0;
    unsigned most_frequent_count = 0,
             last_counter_index = 0;


    printf("Please press (CTRL+D) or (CTRL+Z on windows) when you are done\n"
           "--------------------------------------------------------------\n");

    // Count the symbols in the user input
    // ------------------------------------------------------------------------
    while ((tmp_char = getchar()) != EOF) {
        // Ye-ye, we know UTF-8 can have variable length
        // for what is called "character" but we do not care about that in this case
        if (tmp_char < 0 || tmp_char > 127) {
            // last position contains counter for all non-ASCII symbols
            last_counter_index = MAX_SYMBOLS_COUNT - 1;
            symbol_counters[last_counter_index] += 1;
        } else {
            last_counter_index = tmp_char;
            symbol_counters[last_counter_index] += 1;
        }

        if (symbol_counters[last_counter_index] > most_frequent_count) {
            most_frequent_count = symbol_counters[last_counter_index];
        }
    }

    
    // Convert current histo row number to word count
    // according to ratio
    // ------------------------------------------------------------------------
    printf("\n----------------------------------------------------------------"
           "\nTHIS IS A HORIZONTAL HISTOGRAM"
           "\n----------------------------------------------------------------\n");
    float histoh_to_cc_ratio = 1;
    if (most_frequent_count > HISTOGRAM_LENGTH) {
        histoh_to_cc_ratio =
            (float) HISTOGRAM_LENGTH / most_frequent_count;
    }
    unsigned histo_thresholds[HISTOGRAM_LENGTH] = {0};
    printf("| %5s |", "CHAR");
    for (unsigned j = 0; j < HISTOGRAM_LENGTH; ++j) {
        histo_thresholds[j] = (float) ((j+1) / histoh_to_cc_ratio);
        printf(" %5d |", histo_thresholds[j]);
    }
    printf("\n--------------------------------------------------------------\n");

    // Print the histogram
    // ------------------------------------------------------------------------
    for (unsigned i = 0; i < MAX_SYMBOLS_COUNT - 1; ++i) {
        // Skip symbols that are missing in the user input at all
        // as well as all the non-printable characters (we'll deal later with it)
        if (symbol_counters[i] == 0 || i < 32) {
            continue;
        }

        printf("| %5c |", i);
        for (unsigned j = 0; j < HISTOGRAM_LENGTH; ++j) {
            if (symbol_counters[i] >= histo_thresholds[j]) {
                printf(" %5c |", '*');
            } else {
                printf(" %5c |", ' ');
            }
        }
        printf("\n");
    }

    return 0;
}