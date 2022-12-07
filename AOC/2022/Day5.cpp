#include <iostream>
#include <stack>
#include <vector>
#include <queue>

using namespace std;

int STACK_NUMBER = 10;
int DATA_SIZE = 512;

int main()
{
    vector<stack<char>> stacks(STACK_NUMBER);
    vector<stack<char>> tmp(STACK_NUMBER);
    string syf1,syf2,syf3;
    stack<char> tab;
    int a,b,c;
    char ct;
    string znak;
    for(int i=0;i<8;i++){
        for(int j = 1;j<=STACK_NUMBER-1;j++){
            cin>>znak;
            if(znak=="0"){
                continue;
            }
            tmp[j].push(znak[1]);
        }
    }
    int bajo;
    for(int i = 1 ;i<=STACK_NUMBER-1;i++){
        cin >> bajo;
        while(!tmp[i].empty()){
            stacks[i].push(tmp[i].top());
            tmp[i].pop();
        }
    }
    for(int i = 11; i <=DATA_SIZE;i++){
        cin >>syf1>>a>>syf2>>b>>syf3>>c;
        while(a--){
            ct = stacks[b].top();
            stacks[b].pop();
            tab.push(ct);
        }
        while (!tab.empty()){
            ct = tab.top();
            tab.pop();
            stacks[c].push(ct);
        }
        
    }
    for(int i = 1; i <= STACK_NUMBER-1; i++){
        cout<<stacks[i].top();
    }
    return 0;
}