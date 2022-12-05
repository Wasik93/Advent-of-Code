#include <iostream>
using namespace std;

struct segment{
    int x;
    int y;
};
int main(){
    int counter = 0;
    for(int iz = 0; iz < 1000; iz++){
        segment A,B;
        char syf0,syf1,syf2;
        cin >> A.x >> syf0 >> A.y >> syf1 >> B.x >> syf2 >> B.y;
        /*if ((A.x >= B.x && A.y <= B.y) || (A.x <=B.x && A.y >= B.y)){
            counter++;
        } */
        if ((A.x >= B.x && A.x <= B.y) || (A.y >=B.x && A.x <= B.x)){
            counter++;
        }
    }
    cout<<counter<<'\n';
    return 0;
}