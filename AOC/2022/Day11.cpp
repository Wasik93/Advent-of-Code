#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

long long wl;

struct MONKE{
    long long counter;
    long long MONKE_number;
    char operation;
    long long operation_number;
    long long test;
    long long test_true;
    long long test_false;
    vector<long long> items;
    friend ostream& operator<<(ostream &stream, const MONKE &S){
        cout <<"MONKE "<<S.MONKE_number<<": ";
        for(long long i = 0; i < S.items.size(); i++){
            cout << S.items[i]<<", ";
        }
        cout << endl;
        return stream;
    }
};

void phase(vector<MONKE> &monkeys, long long v){
    long long k = monkeys[v].items.size();
    while(k != 0){
        long long worry = monkeys[v].items[0];
        if(monkeys[v].operation_number == 2137){
            worry = worry * worry;
        }
        else{
            if(monkeys[v].operation == '+'){
                worry += monkeys[v].operation_number;
            }
            else{
                worry *= monkeys[v].operation_number;
            }
        }
        worry = worry%wl;//part 2
        //worry = worry/3 //part 1
        if(worry%monkeys[v].test == 0){
            monkeys[monkeys[v].test_true].items.push_back(worry);
        }
        else{
            monkeys[monkeys[v].test_false].items.push_back(worry);
        }
        monkeys[v].items.erase(monkeys[v].items.begin());
        k--;
        monkeys[v].counter++;
    }
}


int main(){
    long long MONK = 8;
    //input file was changed
    vector<MONKE> monkeys(MONK);
    long long monke_number;
    long long n;
    for(long long i = 0; i < MONK; i++){
        cin >> monke_number;
        monkeys[i].MONKE_number = i;
        cin >> n;
        long long a;
        for(long long j = 0; j < n; j++){
            cin >> a;
            monkeys[i].items.push_back(a);
        }
        cin >> monkeys[i].operation;
        string op_num;
        cin >> op_num;
        if(op_num == "old"){
            monkeys[i].operation_number = 2137;
        }
        else{
            monkeys[i].operation_number = stoi(op_num);
        }
        cin >> monkeys[i].test >> monkeys[i].test_true >> monkeys[i].test_false;
    }
    wl = 1;
    for(long long i = 0; i < MONK; i++){
        wl *= monkeys[i].test;
    }
    cout << wl <<'\n';
    for(long long j = 0; j < 10000; j++){
        for(long long i = 0; i < MONK; i++){
            phase(monkeys, i);
        }
    }
    vector<long long> tmp;
    for(long long i = 0; i < MONK; i++){
        cout << monkeys[i].counter <<'\n';
        tmp.push_back(monkeys[i].counter);
    }
    sort(tmp.begin(),tmp.end());
    cout <<tmp[MONK - 2]*tmp[MONK - 1]<<'\n';
    return 0;
}