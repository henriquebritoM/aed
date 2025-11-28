#include <string>
#include <iostream>
#include <vector>
#include <stack>

using namespace std;

class Node {
    public:
    int value;
    Node* next;

    Node(int value) {
        this->value = value;
        this->next = nullptr;
    }

    void add_next(Node* node) {
        this->next = node;
    }

};

class LinkedList {
    public:

    Node* head;
    Node* tail;

    LinkedList() {
        head = nullptr;
    }

    //  Adiciona um elemento ao final da lista
    void push(int value) {
        Node* next = new Node(value);
        
        if (head == nullptr) {
            head = next;
            tail = next;
            return;
        }

        tail->next = next;
        tail = next;
    }

    //  Adiciona um elemento ao final da lista
    void push_node(Node* next) {
        if (head == nullptr) {
            head = next;
            tail = next;
            return;
        }

        tail->next = next;
        tail = next;
    }

    bool is_empty() {
        return head == nullptr;
    }

    void print() {
        if (head == nullptr) return;
        Node* current = head;
        while (current != nullptr) {
            cout << current->value << endl;
            current = current->next;
        }
    }

};

//  essa função destrói as duas linked lists originais 
//  mudando completamente a ordem dos ponteiros
LinkedList merge(LinkedList & l1, LinkedList & l2) {
    LinkedList new_ll = LinkedList();

    Node* n1 = l1.head;
    Node* n1_next;
    Node* n2 = l2.head;
    Node* n2_next;

    while (n1 != nullptr || n2 != nullptr) {

        if (n1 != nullptr && n2 != nullptr) {
            n1_next = n1->next;
            n2_next = n2->next;

            if (n1->value > n2->value) {
                new_ll.push_node(n2);
                new_ll.push_node(n1);
            }
            else if (n1->value < n2->value) {
                new_ll.push_node(n1);
                new_ll.push_node(n2);   
            }
            else {
                new_ll.push_node(n1);
            }

            n1 = n1_next;
            n2 = n2_next;
        }
        else if (n1 == nullptr) {
            n2_next = n2->next;

            new_ll.push_node(n2);
            n2 = n2_next;
        }
        else if (n2 == nullptr) {
            n1_next = n1->next;

            new_ll.push_node(n1);
            n1 = n1_next;
        }
    }
    
    
    return new_ll;

}

int main() {
    LinkedList ll1 = LinkedList();
    LinkedList ll2 = LinkedList();

    ll1.push(1);
    ll1.push(3);
    ll1.push(5);
    ll1.push(7);
    ll1.push(9);
    ll1.push(10);
    ll1.push(11);
    
    ll2.push(2);
    ll2.push(3);
    ll2.push(4);
    ll2.push(6);
    ll2.push(8);
    ll2.push(10);
    ll2.push(12);
    
    LinkedList ll3 = merge(ll1, ll2);

    ll3.print();
}