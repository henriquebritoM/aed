#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define BUFF_LEN 256

struct indice_secundario {
	long id;
	char sigla[2];
};

struct indice_secundario fill_indices(char* buffer, long ftell, long* indices) {

	char* token = strtok(buffer, ";"); 

	int id = atoi(buffer);

	indices[id] = ftell; 

	strtok(NULL, ";");

	struct indice_secundario is;
	is.id = id;
	strcpy(is.sigla, buffer);

	return is;
}

// Indice secundário -> Indice primário -> Arquivo

int main() {

	FILE *fp = fopen("./DadosEnem.txt", "r");
	if (fp == NULL) 
		exit(1);

	long indice_escolas[30228 + 1];
	struct indice_secundario indice_secundarios[30228];

	char buffer[BUFF_LEN];
	int i = 0;
	while (1) {

		long ftell_n = ftell(fp);
		char *result = fgets(buffer, sizeof(buffer), fp);
		if (result == NULL)
			break;

		struct indice_secundario is;
		is = fill_indices(buffer, ftell_n, indice_escolas);

		indice_secundarios[i] = is;
		i += 1;
	}

	fseek(fp, indice_escolas[30228], SEEK_SET);
	printf("%ld\n", indice_escolas[30228]);

	char *result = fgets(buffer, sizeof(buffer), fp);
	if (result == NULL)
		exit(3);

	printf("%s", buffer);

	fclose(fp);

	return 0;
}
