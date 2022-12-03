#include <iostream>
#include <string.h>
#include <map>
#include <vector>

using namespace std;

int main(){
    map<char,int> slownik;
    for(int i = 1; i <= 26; i++){
        slownik[(char)(64 + i)] = i + 26;
        slownik[(char)(96 + i)] = i;
    }
    int suma = 0;
    /*for(int iz = 0; iz < 300; iz++){
        string zdanie;
        cin >> zdanie;
        int k = zdanie.length();
        map<char,int> litery;
        for(int i = 1; i <= 26; i++){
            litery[(char)(64 + i)] = 0;
            litery[(char)(96 + i)] = 0;
        }
        string first = zdanie.substr(0,k/2);
        string second = zdanie.substr(k/2,k/2);
        for(int i = 0; i < first.length(); i++){
            for(int j = 0; j < second.length(); j++){
                if(first[i] == second[j]){
                    litery[first[i]] = 1;
                }
            }
        }
        for(int i = 1; i <= 26; i++){
            if(litery[(char)(64 + i)]==1){
                suma += slownik[(char)(64 + i)];
            }
            if(litery[(char)(96 + i)] == 1){
                suma += slownik[(char)(96 + i)];
            }
        }
    }
    cout << suma<<'\n';
    */
    for(int iz = 0; iz < 100; iz++){
        vector<char> literki;
        string first,second,third;
        map<char,int> litery;
        for(int i = 1; i <= 26; i++){
            litery[(char)(64 + i)] = 0;
            litery[(char)(96 + i)] = 0;
        }
        cin >> first >> second>> third;
        for(int i = 0; i < first.length(); i++){
            for(int j = 0; j < second.length(); j++){
                if(first[i] == second[j]){
                    literki.push_back(first[i]);
                }
            }
        }
        for(int i = 0; i < literki.size(); i++){
            for(int j = 0; j < third.size(); j++){
                if(literki[i] == third[j]){
                    litery[third[j]] = 1;
                }
            }
        }
        for(int i = 1; i <= 26; i++){
            if(litery[(char)(64 + i)]==1){
                suma += slownik[(char)(64 + i)];
            }
            if(litery[(char)(96 + i)] == 1){
                suma += slownik[(char)(96 + i)];
            }
        }
    }
    cout <<suma<<'\n';


    return 0;
}