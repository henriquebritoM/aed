#include <string>
#include <iostream>
#include <vector>
#include <stack>

using namespace std;

class Node {
    public:
    int id;
    string memoria;
    Node* next;
    
    //  Construtor
    Node(int id, string memoria) {
        this->id = id;
        this->memoria = memoria;
        this->next = nullptr;
    }
};

class LinkedList {
    Node* head;
    Node* tail;

    public:
    //  Construtor
    LinkedList() {
        head = nullptr;
        tail = nullptr;
    }
    
    //  Adiciona um elemento ao final da lista
    void push(int id, string memoria) {
        Node* next = new Node(id, memoria);
        
        if (head == nullptr) {
            head = next;
            tail = next;
            return;
        }

        tail->next = next;
        tail = next;
    }
    
    //  inverte a ordem da lista in place
    void inverte() {

        Node* past = head;
        if (past == nullptr) return;    //  Não há nada para inverter

        Node* current = head->next;
        if (current == nullptr) return; //  Não há como inverter um elemento

        Node* next = current->next;     //  pode ser nulo (lista com 2 elementos)

        past->next = nullptr;           
        tail = past;

        while (next != nullptr) {
            current->next = past;
            past = current;
            current = next;
            next = next->next;
        }

        current->next = past;
        head = current;
    }
    
    //  inverte a ordem da lista com um stack helper
    void inverte_stack() {
        
        if (head == nullptr) return;
        
        stack<Node*> s;         
        Node* current = head;
        tail = current;

        while (current != nullptr) {
            s.push(current);
            current = current->next;
        }

        head = s.top();
        current = s.top();
        s.pop();

        while (!s.empty()) {
            current->next = s.top();
            s.pop();
            current = current->next;
        }

        current->next = nullptr;
    }
    
    //  Inverte a ordem da lista de forma recursiva
    void inverte_recursiva() {
        Node* past = head;
        if (past == nullptr) return;    //  Não há nada para inverter

        Node* current = head->next;
        if (current == nullptr) return; //  Não há como inverter um elemento

        Node* next = current->next;     //  pode ser nulo (lista com 2 elementos)

        past->next = nullptr;           
        tail = head;
        head = inverte_recursiva_helper(past, current, next);
    }
    
    //  Função recursiva que inverte a lista

    Node* inverte_recursiva_helper(Node* past, Node* current, Node* next) {         
        current->next = past;
        past = current;

        if (next == nullptr) return current;

        current = next;
        next = next->next;
        return inverte_recursiva_helper(past, current, next);
    }

    void print() {
        if (head == nullptr) return;
        Node* current = head;
        while (current != nullptr) {
            cout << current->id << " " << current->memoria << endl;
            current = current->next;
        }
    }

};

int main() {
    LinkedList ll = LinkedList();

    ll.push(1, "mãe");
    ll.push(2, "pai");
    ll.push(3, "infância");
    ll.push(4, "praia");
    ll.push(5, "natal");
    ll.push(6, "ensino fundamental");
    ll.push(7, "adolescência");
    ll.push(8, "amizade");
    ll.push(9, "ensino médio");
    ll.push(10, "faculdade");
    ll.push(11, "trabalho");
    ll.push(12, "vida aulta");
    
    cout << "Lista normal:" << endl;
    ll.print();
    cout << endl;
    
    cout << "lista invertida:" << endl;
    ll.inverte();
    ll.print();
    cout << endl;
    
    ll.inverte();
    
    cout << "lista invertida com stack:" << endl;
    ll.inverte();
    ll.print();
    cout << endl;
    
    ll.inverte();
    
    cout << "lista invertida com recursão:" << endl;
    ll.inverte();
    ll.print();
    cout << endl;

}

