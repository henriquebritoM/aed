#include <chrono>
#include <cmath>
#include <iostream>
#include <vector>

using namespace std;
using namespace std::chrono;

// Estrutura do Node
struct Node {
  int id;       // Único por postagem
  double score; // Menor = Mais relevante
  vector<Node *> forward;
  Node(int i, double s, int level)
      : id(i), score(s), forward(level + 1, nullptr) {}
};

class SkipList {
private:
  int max_level;
  int current_level;
  double p;
  Node *head;

  int randomLevel() {
    int lvl = 0;
    while ((double)rand() / RAND_MAX < p && lvl < max_level)
      lvl++;
    return lvl;
  }

public:
  SkipList(int max_lvl, double prob)
      : max_level(max_lvl), p(prob), current_level(0) {
    head = new Node(-1, -1.0, max_level);
  }

  // POST: Insere mantendo a ordenação por score (crescente)
  void insert(int id, double score) {
    vector<Node *> update(max_level + 1, nullptr);
    Node *curr = head;

    for (int i = current_level; i >= 0; i--) {
      while (curr->forward[i] != nullptr && curr->forward[i]->score < score)
        curr = curr->forward[i];
      update[i] = curr;
    }

    int lvl = randomLevel();
    if (lvl > current_level) {
      for (int i = current_level + 1; i <= lvl; i++)
        update[i] = head;
      current_level = lvl;
    }

    Node *newNode = new Node(id, score, lvl);
    for (int i = 0; i <= lvl; i++) {
      newNode->forward[i] = update[i]->forward[i];
      update[i]->forward[i] = newNode;
    }
  }

  // Deleta o node e retorna seu score
  double remove(int id) {
    // Vetor de ponteiros que precisam ser atualizados
    vector<Node *> update(max_level + 1, nullptr);
    Node *current = head;

    // Percorre do nível mais alto ao nível 0
    for (int i = current_level; i >= 0; i--) {
      while (current->forward[i] != nullptr && current->forward[i]->id < id) {
        current = current->forward[i];
      }
      update[i] = current;
    }

    // Candidato ao nó a ser removido
    current = current->forward[0];

    // Verifica se o nó com o id existe
    if (current == nullptr || current->id != id) {
      return -1.0; // id não encontrado
    }

    double removed_score = current->score;

    // Atualiza os ponteiros em cada nível
    for (int i = 0; i <= current_level; i++) {
      if (update[i]->forward[i] != current)
        break;
      update[i]->forward[i] = current->forward[i];
    }

    delete current;

    // Reduz current_level se os níveis superiores ficaram vazios
    while (current_level > 0 && head->forward[current_level] == nullptr) {
      current_level--;
    }

    return removed_score;
  }

  double getScore(int id) {
    Node *current = head;

    for (int i = current_level; i >= 0; i--) {
      while (current->forward[i] != nullptr && current->forward[i]->id < id) {
        current = current->forward[i];
      }
    }

    current = current->forward[0];

    if (current == nullptr || current->id != id) {
      return -1.0; // id não encontrado
    }

    return current->score;
  }

  // Retorna um vetor com os top k menores scores
  vector<double> get_top_k(int k) {
    Node *curr = head->forward[0];
    vector<double> top_k;

    while (curr != nullptr && top_k.size() < k) {
      top_k.push_back(curr->score);

      curr = curr->forward[0];
    }

    return top_k;
  }

  // Quantos tem score menor que x
  int countLessThan(double x) {
    int count = 0;
    Node *current = head->forward[0];

    while (current != nullptr && current->score < x) {
      count++;
      current = current->forward[0];
    }

    return count;
  }
};

// Boost - Remove, atualiza e depois reinsere:
void boost(int id, SkipList sl) {
  double old_score = sl.remove(id);

  double new_score = ((sqrt(5) - 1) / 2) * old_score;

  sl.insert(id, new_score);
}

int main() {

  SkipList sl(32,
              0.5); // Incia a skip list com 32 níveis e 1/2 de probabilidade

  auto t0 = std::chrono::high_resolution_clock::now();

  for (int i = 1; i <= 2000000; i++) {
    double score = sqrt(i);

    sl.insert(i, score);
  }

  auto t1 = high_resolution_clock::now();
  cout << "Inserir 2M de nós: " << duration_cast<milliseconds>(t1 - t0).count()
       << "ms" << endl;

  t0 = std::chrono::high_resolution_clock::now();

  for (int i = 1000000; i < 1200000; i++) {
    sl.remove(i);
  }

  t1 = high_resolution_clock::now();
  cout << "Remover 200 mil nodes: "
       << duration_cast<milliseconds>(t1 - t0).count() << "ms" << endl;

  t0 = std::chrono::high_resolution_clock::now();

  int ks[] = {100000, 200000, 300000, 400000, 500000, 1000000};
  for (int j = 0; j < 6; j++) {
    vector<double> top_k = sl.get_top_k(ks[j]);
    // Não acho uma boa ideia printar isso não hein
  }

  t1 = high_resolution_clock::now();
  cout << "Contar os top k: " << duration_cast<milliseconds>(t1 - t0).count()
       << "ms" << endl;

  t0 = std::chrono::high_resolution_clock::now();

  int cnts[] = {200, 300, 400, 500, 1000};
  for (int j = 0; j < 5; j++) {
    sl.countLessThan(cnts[j]);
  }

  t1 = high_resolution_clock::now();

  cout << "Contar scores: " << duration_cast<milliseconds>(t1 - t0).count()
       << "ms" << endl;

  t0 = std::chrono::high_resolution_clock::now();

  for (int i = 500000; i < 600000; i++) {
    boost(i, sl);
  }

  t1 = high_resolution_clock::now();

  cout << "Boosts: " << duration_cast<milliseconds>(t1 - t0).count() << "ms"
       << endl;

  return 0;
}
