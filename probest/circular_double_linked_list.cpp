#include <string>
#include <iostream>
#include <vector>
#include <algorithm>
#include <stack>
#include <limits>

using namespace std;

class Node {    
    public:

    int value;
    Node* next;
    Node* previous; 

    Node(int value) {
        this->value = value;
        this->next = nullptr;
    }

    void add_next(Node* node) {
        node->previous = this;
        this->next = node;
    }

};

//  Uma lista duplamente encadeada circular
class LinkedList {
    public:
    Node *head;
    Node *tail;
    
    LinkedList() {
        head = nullptr;
        tail = nullptr;
    }

    void push(int value) {
        Node* next = new Node(value);
        
        if (head == nullptr) {
            next->next = next;
            next->previous = next;
            head = next;
            tail = next;
            return;
        }

        tail->next = next;
        next->previous = tail;
        next->next = head;
        head->previous = next;
        tail = next;
    }

    void push_front(int value) {
        Node* next = new Node(value);
        
        if (head == nullptr) {
            next->next = next;
            next->previous = next;
            head = next;
            tail = next;
            return;
        }

        next->next = head;
        head->previous = next;
        next->previous = tail;
        tail->next = next;
        head = next;
    }

    stack<int> remove_impares() {
        int i = 1;
        Node* node = head;
        stack<int> s;
        stack<int> to_be_deleted;

        if (node == nullptr) return s;

        while (node->next != head) {
            if (node->value % 2 == 0) {
                s.push(get_node(i)->value);
                to_be_deleted.push(i);
            }
            node = node->next;
            i++;
        }
        
        while(!to_be_deleted.empty()) {
            int temp = to_be_deleted.top();
            to_be_deleted.pop();
            delete_node(temp);
        }
        
        return s;
    }

    void delete_node(int i) {
        Node* node = get_node(i);
        Node* n_next = node->next;
        Node* n_previous = node->previous;

        if (node == head) head = n_next;
        if (node == tail) tail = n_previous;
        n_next->previous = n_previous;
        n_previous->next = n_next;
        free(node);
    }

    Node* get_node(int num) {
        Node* node = head;
        for (int i = 0; i < num; i++) {
            node = node->next;
        }
        return node;
    }

    void print() {
        if (head == nullptr) return;
        Node* current = head;
        while (current->next != head) {
            cout << current->value << endl;
            current = current->next;
        }
        cout << current->value << endl;
    }

};

void arruma_cavaleiros(LinkedList &ll) {
    stack<int> s = ll.remove_impares();
    while(!s.empty()) {
        int temp = s.top();
        s.pop();
        ll.push_front(temp);
    }
}

int main() {
    LinkedList ll = LinkedList();
    int arr[] = {4, 3, 2, 7, 8, 5, 6, 9};
    for (int i = 0; i < 8; i++) ll.push(arr[i]);
    ll.print();
    cout << endl << endl;
    arruma_cavaleiros(ll);
    ll.print();
    
}