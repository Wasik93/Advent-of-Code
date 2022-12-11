#include <iostream>

using namespace std;

void Part1(){
    string task;
    int cycle_count = 1;
    int value = 1;
    int addx;
    int score = 0;
    for(int i = 0; i < 146; i++){
        cin >> task;
        if(task == "noop"){
            cycle_count++;
            if(cycle_count%40 == 20){
                cout<<value<<'\n';
                score += cycle_count*value;
            }
        }
        else{
            cin >> addx;
            cycle_count++;
            if(cycle_count%40 == 20){
                cout<<value<<'\n';
                score += cycle_count*value;
            }
            cycle_count++;
            value += addx;
            if(cycle_count%40 == 20){
                cout<<value<<'\n';
                score += cycle_count*value;
            }
        }
    }
    cout << score<<'\n';
}

void Part2(){
    string task;
    string CRT = "";
    int cycle_count = 0;
    int value = 1;
    int addx;
    for(int i = 0; i < 142; i++){
        cin >> task;
        if(task == "noop"){
          
            if(abs(cycle_count - value) <= 1){
                CRT = CRT + "#";
            }
            else {
                CRT = CRT + ".";
            }
            cycle_count++;
            cycle_count %= 40;
        }
        else{
            cin >> addx;
            
            if(abs(cycle_count - value) <= 1){
                CRT = CRT + "#";
            }
            else {
                CRT = CRT + ".";
            }
            cycle_count++;
            cycle_count %= 40;
            if(abs(cycle_count - value) <= 1){
                CRT = CRT + "#";
            }
            else {
                CRT = CRT + ".";
            }
            cycle_count++;
            cycle_count %= 40;
            value = value + addx;
        }
    }
    for(int i = 0; i < 6; i++){
        for(int j = 0; j < 40; j++){
            cout<<CRT[i*40 + j];
        }
        cout <<'\n';
    }
}

int main(){
    //Part1();
    Part2();
    return 0;
}