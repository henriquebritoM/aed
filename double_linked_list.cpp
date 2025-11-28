#include <string>
#include <iostream>
#include <vector>
#include <algorithm>
#include <map>

using namespace std;

class Node1 {    
    public:

    int value;
    Node1* next;
    Node1* previous; 

    Node1(int value) {
        this->value = value;
        this->next = nullptr;
    }

    void add_next(Node1* node) {
        node->previous = this;
        this->next = node;
    }

};

class LinkedList1 {
    public:
    Node1* head;
    Node1* tail;
    
    LinkedList1() {
        head = nullptr;
        tail->next;
    }

    //  Adiciona um elemento ao final da lista
    void push(int value) {
        Node1* next = new Node1(value);
        
        if (head == nullptr) {
            head = next;
            tail = next;
            return;
        }

        tail->next = next;
        tail = next;
    }

    void print() {
        if (head == nullptr) return;
        Node1* current = head;
        while (current != nullptr) {
            cout << current->value << endl;
            current = current->next;
        }
    }

};

class Node2 {    
    public:

    int value;
    int quantity;
    Node2* next;
    Node2* previous; 

    Node2(int value, int quantity) {
        this->value = value;
        this->quantity = quantity;
        this->next = nullptr;
    }

    void add_next(Node2* node) {
        node->previous = this;
        this->next = node;
    }

};

class LinkedList2 {
    Node2* head;
    Node2* tail;
 
    public:
    LinkedList2() {
        head = nullptr;
        tail = nullptr;
    }

    //  Adiciona um elemento ao final da lista
    void push(int value, int quantity) {
        Node2* next = new Node2(value, quantity);
        
        if (head == nullptr) {
            head = next;
            tail = next;
            return;
        }

        tail->next = next;
        tail = next;
    }

    void print() {
        if (head == nullptr) return;
        Node2* current = head;
        while (current != nullptr) {
            cout << "Valor: " << current->value << " Quantidade: " << current->quantity << endl;
            current = current->next;
        }
    }

};

LinkedList2 convert(LinkedList1 l1) {
    map<int, int> map;  //(value, quantity)
    LinkedList2 l2;

    Node1* current = l1.head;
    
    //  automaticamente organiza as keys em ordem crescente 
    //  e conta quantas vezes aparecem
    while (current != nullptr) {
        map[current->value]++;
        current = current->next;
    }

    for (auto pair: map) {
        l2.push(pair.first, pair.second);   //  (value, quantity)
    }

    return l2;
}


int main() {
    LinkedList1 ll = LinkedList1();
    int arr[] = {4, 2, 1, 4, 2, 3, 1, 3, 5, 5, 2, 2, 3, 2, 4, 1, 3, 5};
    for (int i = 0; i < 18; i++) ll.push(arr[i]);

    LinkedList2 l2 = convert(ll);

    l2.print();
}