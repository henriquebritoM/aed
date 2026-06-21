/*
Grupo XXX
Integrantes:
Henrique de Brito Melo Silva - 845562
Ciclano de Tal - matrícula
Beltrano de Tal - matrícula
*/

#include <stdio.h>
#include <string.h>

typedef struct {
  char researcher_name[100];
  char paper_name[200];
} reg;

reg read_to_register(FILE *fd, long *err) {

  reg tmp_reg;
  char buffer[301];

  const char *success = fgets(buffer, 300, fd);
  *err = success == NULL;

  char *tab = strchr(buffer, '\t');
  *tab = '\0';

  strcpy(tmp_reg.researcher_name, buffer);
  strcpy(tmp_reg.paper_name, tab + 1);

  return tmp_reg;
}

int main() {

  FILE *fd = fopen("./dadosPesquisadores.txt", "r");
  if (fd == NULL)
    return 1;

  long err = 0;
  while (!err) {
    reg tmp = read_to_register(fd, &err);
    /* do something */

    printf("Researcher: %s\nPaper: %s\n", tmp.researcher_name, tmp.paper_name);
  }

  fclose(fd);

  return 0;
}
