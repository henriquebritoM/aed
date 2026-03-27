#include <stdio.h>
#include <string.h>

struct compras{
    char nome[50];
    int qtd;
    float valor;
};

int main(){
    struct compras dados[7] = {{"manteiga",1,4.5},
                                {"pao",1,7.99},
                                {"arroz",2,10.95},
                                {"feijao",3,9.4},
                                {"carne",2,21.90},
                                {"detergente",3,1.65},
                                {"sabao",2,3.45}};
    int k;
    char str[100], str1[100];
    FILE *f = fopen("compras.bin","wb");
    int tam;

    for(k = 0; k < 7; k++){
        tam = sprintf(str,"%s|%d|%.2f",dados[k].nome,dados[k].qtd,dados[k].valor);
        sprintf(str1,"%c%s",tam,str);
        fwrite(str1, sizeof(char),strlen(str1), f);
	}

	fclose(f);

    return 0;
}

