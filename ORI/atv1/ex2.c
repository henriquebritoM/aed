#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void find_spaces(char * str, int n, int * vec, int m) {
    
    int spc_cnt = 0;

    for (int i = 0; i < n; i++) {

        if (str[i] == ' ') {
            spc_cnt++;
            if (spc_cnt > m) return;

            vec[spc_cnt - 1] = i;
        }
    }

}

int main() {
    double total_notas = 0;
    char aluno_maior_nota[20] = {'\0'};
    double maior_nota = 0;
    int qtd_alunos = 0;

    char input[50] = {'\0'};

    FILE *fd = fopen("notas.txt", "r");
    if (fd == NULL) exit(1);

    while (1) {
        if (fgets(input, 49, fd) == NULL) break;
        input[strlen(input)] = '\0';

        int spaces[3] = {0} ;

        find_spaces(input, strlen(input), spaces, 3);

        input[spaces[0]] = '\0'; 
        input[spaces[1]] = '\0';
        input[spaces[2]] = '\0'; 

        double nota = atof(input + spaces[2] + 1);
        
        total_notas += nota;
        qtd_alunos++;

        if (nota > maior_nota) {
            maior_nota = nota;
            strcpy(aluno_maior_nota, input + spaces[0] + 1);
        }
    }

    printf("A média de notas é %.2lf\n", total_notas / qtd_alunos);
    printf("O aluno com a maior nota é %s, com %.2lf\n", aluno_maior_nota, maior_nota);
}