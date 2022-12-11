#include <iostream>
#include <vector>

using namespace std;

class Tree{
public:
    int height;
    bool left;
    bool right;
    bool top;
    bool bottom;
    int scenic_score;
    Tree(){
        left = true;
        right = true;
        bottom = true;
        top = true;
        scenic_score = 0;
    }
    
};
void see_left(vector<vector<Tree>>&Grid,int n){
    for(int i = 0; i < n; i++){
        int p_tree = -1;
        for (int j = 0; j < n; j++){
            if(Grid[i][j].height <= p_tree){
                Grid[i][j].left = false;
            }
            else{
                p_tree = Grid[i][j].height;
            }
        }
    }
}
void see_right(vector<vector<Tree>>&Grid,int n){
    for(int i = 0; i < n; i++){
        int p_tree = -1;
        for (int j = n - 1 ; j >= 0; j--){
            if(Grid[i][j].height <= p_tree){
                Grid[i][j].right = false;
            }
            else{
                p_tree = Grid[i][j].height;
            }
        }
    }
}
void see_top(vector<vector<Tree>>&Grid,int n){
    for(int j = 0; j < n; j++){
        int p_tree = -1;
        for (int i = 0; i < n; i++){
            if(Grid[i][j].height <= p_tree){
                Grid[i][j].top = false;
            }
            else{
                p_tree = Grid[i][j].height;
            }
        }
    }
}
void see_bottom(vector<vector<Tree>>&Grid,int n){
    for(int j = 0; j < n; j++){
        int p_tree = -1;
        for (int i = n - 1; i >= 0; i--){
            if(Grid[i][j].height <= p_tree){
                Grid[i][j].bottom = false;
            }
            else{
                p_tree = Grid[i][j].height;
            }
        }
    }
}

void count_see(vector<vector<Tree>>& Grid,int vi,int vj, int n){
    int left_see = 0;
    int right_see = 0;
    int top_see = 0;
    int bottom_see = 0;
    int p_tree = Grid[vi][vj].height;
    for(int i = vi - 1; i >=0 ; i--){
        if(Grid[i][vj].height < p_tree){
            top_see++;
        }
        else{
            top_see++;
            break;
        }
    }
    for(int i = vi+1; i < n ; i++){
        if(Grid[i][vj].height < p_tree){
            bottom_see++;
        }
        else{
            bottom_see++;
            break;
        }
    }
    for(int j = vj - 1; j >=0 ; j--){
        if(Grid[vi][j].height < p_tree){
            left_see++;
        }
        else{
            left_see++;
            break;
        }
    }
    for(int j = vj + 1; j < n ; j++){
        if(Grid[vi][j].height < p_tree){
            right_see++;
        }
        else{
            right_see++;
            break;
        }
    }
    Grid[vi][vj].scenic_score = left_see * right_see * top_see * bottom_see;
}

int main(){
    int n = 99;
    vector<vector<Tree>> Grid(n);
    string treeline;
    for(int i = 0; i < n; i++){
        cin >> treeline;
        for(int j = 0; j < treeline.length(); j++){
            Tree tmp;
            tmp.height = treeline[j] - '0';
            Grid[i].push_back(tmp);
        }
    }
    see_left(Grid,n);
    see_right(Grid,n);
    see_top(Grid,n);
    see_bottom(Grid,n);

    for(int i = 0; i < n; i++){
        for(int j = 0; j < n; j++){
            count_see(Grid, i, j, n);
        }
    }

    int counter = 0;
    for(int i = 0;i <n;i++){
        for(int j = 0; j < n; j++){
            if(Grid[i][j].left || Grid[i][j].right || Grid[i][j].top || Grid[i][j].bottom){
                counter++;
            }
        }
    }
    cout << counter<<'\n';
    int maxx = 0;
    for(int i = 0;i <n;i++){
        for(int j = 0; j < n; j++){
            maxx = max(Grid[i][j].scenic_score,maxx);
            //cout << Grid[i][j].scenic_score<<" ";
        }
        //cout << '\n';
    }
    cout << maxx<<'\n';

    return 0;
}