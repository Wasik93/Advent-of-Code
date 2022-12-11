#include <iostream>
#include <set>
#include <vector> 

using namespace std;

set<pair<int,int>> tab;

vector<pair<int,int>> wonsz(10); 

int print_situation(){
    for(int i = 20; i >= -20; i--){
        for(int j = -20; j <= 20; j++){
            char znak = '_';
            for(int k = 9; k >= 0; k--){
                if(wonsz[k].first == j && wonsz[k].second == i){
                    znak = (char)(k + 48);
                }
            }
            cout<<znak<<" ";
        }
        cout<<'\n';
    }
    cout <<'\n';
    cout <<'\n';
    return 0;
}

void move(int i, int j){
    int distance_x = wonsz[i].first - wonsz[j].first;
    int distance_y = wonsz[i].second - wonsz[j].second;
    if(abs(distance_x) + abs(distance_y) > 2){
        if(distance_x > 0){
            wonsz[j].first++;
        }
        else{
            wonsz[j].first--;
        }
        if(distance_y > 0){
            wonsz[j].second++;
        }
        else{
            wonsz[j].second--;
        }
    }
    else if(abs(distance_x) > 1){
        if(distance_x > 0){
            wonsz[j].first++;
        }
        else{
            wonsz[j].first--;
        }
    }
    else if(abs(distance_y) > 1){
        if(distance_y > 0){
            wonsz[j].second++;
        }
        else{
            wonsz[j].second--;
        }
    }
    else{
        return;
    }
    if(j == 9){
        return;
    }
    else{
        move(j,j+1);
    }
}


int main(){
    int n = 2000;
    char znak;
    int numer;
    for(int i = 0;i <10;i++){
        wonsz[i].first =0;
        wonsz[i].second=0;
    }
    for(int i = 0 ; i < n; i++){
        cin >> znak >> numer;
        for(int i = 0; i< numer; i++){
                switch (znak)
                {
                case 'D':
                    wonsz[0].second++;
                break;
                case 'U':
                    wonsz[0].second--;
                break;
                case 'L':
                    wonsz[0].first--;
                break;
                case 'R':
                    wonsz[0].first++;
                break;
                }
                move(0,1);

            pair<int,int> tail;
            tail.first = wonsz[9].first;
            tail.second = wonsz[9].second;
            tab.insert(tail);
        }
    }
    cout<<tab.size()<<'\n';

    return 0;
}