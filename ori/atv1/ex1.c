#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void find_spaces(char * str, int n, int *first, int *second) {
    
    *first = 0;
    *second = 0;

    for (int i = 0; i < n; i++) {
        if (str[i] == ' ') {
            if (*first == 0) {
                *first = i;
            }
            else {
                *second = i;
                return;
            }
        }
    }

}

int main() {
    double total = 0;
    int qtd;
    double price;

    char input[50] = {'\0'};

    FILE *fd = fopen("compra.txt", "r");
    if (fd == NULL) exit(1);

    while (1) {
        if (fgets(input, 49, fd) == NULL) break;
        input[strlen(input)] = '\0';

        int first_space;
        int second_space;

        find_spaces(input, strlen(input), &first_space, &second_space);
        input[first_space] = '\0';
        input[second_space] = '\0';

        qtd = atoi(input + first_space + 1);
        price = atof(input + second_space + 1);
        
        total += (double) qtd * price;
    }

    printf("O preço total é R$: %.2lf\n", total);
}