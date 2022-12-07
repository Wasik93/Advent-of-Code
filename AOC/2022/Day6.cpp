#include <iostream>
#include <set>

using namespace std;
int MARKER_LENGTH = 14;

int main(){
    string sentence;
    cin >> sentence;
    set<char> tmp;
    for(int i = MARKER_LENGTH - 1 ; i < sentence.length(); i++){
        for(int j = 0; j < MARKER_LENGTH; j++){
            tmp.insert(sentence[i - j]);
        }
        if(tmp.size() == MARKER_LENGTH){
            cout << i+1 << '\n';
            break;
        }
        tmp.clear();
    }
    return 0;
}