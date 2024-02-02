/* 
 * # Task :
 *   write a program that will print a histogram of lengths of words from input
 */

#include <stdio.h>


// Maximum amount of words with different length
// that can be handled by the program
// + 1 last will be the position for all other words which length is larger 
#define MAX_WORD_LENGTH 21
#define HISTOGRAM_HEIGHT 10


int main() {
    // we could use `char`, but there is no support for UTF-8
    short ch = 0;
    unsigned
        // Temporary counter for current word length
        last_word_len = 0,

        // List of counters of how many words with same length
        // does the user input contains
        // The (word length - 1) corresponds to a specific position in array
        // The latest position is reserved for counter for other words
        // which lengths are exceeds (MAX_WORD_LENGTH - 1)
        word_lengths[MAX_WORD_LENGTH] = {0},
        
        // This will allow us to establish the relation between histogram height
        // and the largest word count,
        // so we can evenly distribute all counts throughtout the histogram
        most_frequent_word_count = 0;


    printf("-------------------------------------------------------------\n"
           "Take into account that only words with length up to %d will "
           "be counted.\n"
           "Other words which length is larger than MAXIMUN mentioned above "
           "will be shown in the last column of histogram\n"
           "Please type your input and then press Ctrl+D to get the results:\n"
           "-------------------------------------------------------------\n",
           
           MAX_WORD_LENGTH - 1
    );


    // Perform length counting and storing the counters in the array
    // ------------------------------------------------------------------------
    while ((ch = getchar()) != EOF) {
        if (ch < 0 || ch > 127) {
            printf("\n\nONLY ASCII characters are allowed. CHECK YOUR INPUT");
        }

        // Only count lowercase and uppercase letters
        if ((ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z')) {
            ++last_word_len;
            continue;
        }
        else if (last_word_len > 0) {
            if (last_word_len > MAX_WORD_LENGTH) 
                last_word_len = MAX_WORD_LENGTH;
        
            // Index start from 0 so a corresponding word length is biggery by 1
            word_lengths[last_word_len - 1] += 1;

            // Detect most frequent word and store amount of times it was met
            // we'll need this to evenly distribute frequency on our histogram
            if (most_frequent_word_count < word_lengths[last_word_len - 1]) {
                most_frequent_word_count += 1;
            }
        }

        last_word_len = 0;
    }


    // Draw the histogram
    // ------------------------------------------------------------------------
    // Ratio between the maximum height of histogram
    // and the most frequent word count
    // if the (word count) is larger than height then recalculate the ratio
    float histoh_to_wc_ratio = 1;
    if (most_frequent_word_count > HISTOGRAM_HEIGHT) {
        histoh_to_wc_ratio =
            (float) HISTOGRAM_HEIGHT / most_frequent_word_count;
    }

    for (unsigned i = HISTOGRAM_HEIGHT; i > 0; --i) {
        // convert current histo row number to word count
        // according to ratio
        unsigned wlen = (float) (i / histoh_to_wc_ratio);

        printf("| %12u | %12s |", wlen, "->");
        for (unsigned j = 0; j < MAX_WORD_LENGTH; ++j) {
            if (word_lengths[j] >= wlen) {
                printf(" %3c |", '*');
            } else {
                printf(" %3c |", ' ');
            }
        }
        printf("\n");
    }


    // Print histogram notations at the bottom
    // ------------------------------------------------------------------------
    for (unsigned i = 0; i < 32 + MAX_WORD_LENGTH * 6; ++i) {
        printf("%c", '-');
    }
    printf("\n| Word Count ^ | Word length: |");

    for (unsigned i = 1; i < MAX_WORD_LENGTH; ++i) {
        printf(" %3d |", i);
    }
    printf(" %2d+ |\n\n", MAX_WORD_LENGTH);

}