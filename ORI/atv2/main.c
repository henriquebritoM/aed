#include <stdio.h>
#include <string.h>
#include <stdlib.h>

struct compra {
    int qtd;
    float valor;
    char name[50];
};

int main() {

    FILE *fp = fopen("compras.bin", "rb");
    if (fp == NULL) exit(1);

    struct compra c;
    char tam;

    char buffer[150] = {'\0'};
    while (1) {

        if (fread(&tam, sizeof(char), 1, fp) == 0) break; 

        fread(buffer, sizeof(char), tam, fp);

        int n_field = 0;
        for (int i = tam - 1; i >= 0; i--) {

            if (buffer[i] == '|') {

                buffer[i] = '\0';

                switch (n_field) {
                case 0:
                    c.valor = atof(buffer + i + 1);
                    break;
                
                case 1:
                    c.qtd = atoi(buffer + i + 1);
                    break;
                }

                n_field++;
            }

            strcpy(c.name, buffer);
        }

        printf("nome: %s quantidade: %d valor: %.2f\n", c.name, c.qtd, c.valor);
        
    }



    return 0;
}