#include <stdio.h> 

#define M 10

typedef struct no_b NO_B;

struct no_b {
	int qtd;
	int chaves[M-1];
	NO_B *filhos[M];
}

void imprime(NO_B *no) {
	if (no == NULL) return;
	
	int i;
	for (int i = 0; i < no->qtd; i++) {
		imprime(no->filhos[i]);
		printf("%d ", no->chaves[i]);
	}
	imprime(no->filhos[i]);
}

void split (NO_B *raiz, NO_B *filhos, int ch) {
	
}

// Redistrir as chaves de A para B
// assuma que todas chaves de A < todas as chaves de B
// portando as chaves de A devem ficar a esquerda das chaves de B
void redistribui(NO_B *raiz, NO_B *A, NO_B *B) {

}

//	Se A está sendo concatenado, ele tem apenas um filho !
void concatena(NO_B *raiz, NO_B *A, NO_B *B) {
	int chave = A->chaves[i];
	filho = A->filhos[i];	

	B->chaves[B->qtd] = chave;
	B->filhos[B->qtd] = filho;
	B->qtd += 1;
}

int main() {

	NO_B filho_1 = {4, {10, 20, 30, 40}, {NULL, NULL, NULL, NULL, NULL, }};
	NO_N filho_2 = {2, {120, 130}, {NULL, NULL, NULL, }};
	NO_B pai = {1, {1, {100}, {&filho_1, &filho_2}}};

	imprime(&pai);
	return 0;
}
