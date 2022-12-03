#include <iostream>

using namespace std;

int main(){
    string elf,my;
    int score = 0;
    for(int i = 0; i <2500 ; i++){
        cin >> elf >> my;
        if(elf == "A"){
            if(my == "X"){
                score+= 3;
            }
            if(my == "Y"){
                score+= 1;
            }
            if(my == "Z"){
                score+=2;
            }
        }
        if(elf == "B"){
            if(my == "X"){
                score+= 1;
            }
            if(my == "Y"){
                score+= 2;
            }
            if(my == "Z"){
                score+= 3;
            }
        }

        if(elf == "C"){
            if(my == "X"){
                score+=2;
            }
            if(my == "Y"){
                score+=3;
            }
            if(my == "Z"){
                score+=1;
            }
        }
        if(my == "X"){
            score+=0;
        }
        if(my == "Y"){
            score+=3;
        }
        if(my == "Z"){
            score+=6;
        }
    }
    cout<<score<<'\n';
    return 0;
}