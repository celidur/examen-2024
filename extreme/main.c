#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define RED     "\x1B[31m"
#define GREEN   "\x1B[32m"
#define RESET   "\x1B[0m"

static char *colors[] = {
    "\x1B[31m",
    "\x1B[33m",
    "\x1B[34m",
    "\x1B[35m",
    "\x1B[36m"
};

void printTree(int height) {
    srand(time(NULL));
    int i, j, k, randNum;

    for (i = 1; i <= height; i++) {
        for (j = i; j < height; j++) {
            printf(" ");
        }
        for (k = 0; k < (2 * i - 1); k++) {
            randNum = rand() % 20;
            if (randNum < 4 && i != height) {
                int randColor = rand() % (sizeof(colors) / sizeof(colors[0]));
                printf("%s" "o" RESET, colors[randColor]);
            } else {
                printf(GREEN "*" RESET);
            }
        }
        printf("\n");
    }

    int trunkHeight = height / 6;
    trunkHeight = trunkHeight < 1 ? 1 : trunkHeight;
    for (i = 0; i < height / 3; i++) {
        for (j = 0; j < height - 1; j++) {
            printf(" ");
        }
        printf(RED "|\n" RESET);
    }
}

void printMessage() {
    printf("\n Merry Christmas!\n");
}

int main() {
    int height;
    printf("Enter the height of the tree: ");
    scanf("%d", &height);

    printTree(height);

    return 0;
}
