#include <iostream>
#include <vector>

using namespace std;

struct Directory;
struct File;
int sum_of_sums = 0;
int operating_space = 70000000;
int to_delete = INT32_MAX;

struct File{
    Directory * parent;
    int size;    
    string name;
};

struct Directory{
    Directory * parent;
    vector<Directory*> children;
    vector<File*> files;
    string name;
    int size;
};

void DFS_sum(Directory* dir){
    int sum = 0;
    for(int i = 0;i < dir->files.size(); i++){
        sum += dir->files[i]->size;
    }
   
    for(int i = 0; i < dir->children.size(); i++){
        DFS_sum(dir->children[i]);
        sum += dir->children[i]->size;
    }
    dir->size = sum;
}

void DFS(Directory* dir, int wc){
    for(int i = 0; i<wc;i++){
        cout<<" ";
    }
    cout<<dir->name<<" (dir size:"<<dir->size<<")\n";
    if(dir->size <= 100000){
        sum_of_sums += dir->size;
    }
    if(dir->size >= operating_space){
        if(dir->size < to_delete){
            to_delete = dir->size;
        }
    }
    for(int i = 0;i < dir->files.size(); i++){
        for(int i = 0; i<wc+4;i++){
            cout<<" ";
        }
        cout<<"-"<< dir->files[i]->name<<" (file :" << dir->files[i]->size<<")\n";
    }
   
    for(int i = 0; i < dir->children.size(); i++){
        DFS(dir->children[i],wc + 4);
    }

}


int main(){
    Directory* root = new Directory;
    root->name = "/";
    root->parent = root;
    root->size = 0;
    Directory *cwd;
    cwd = root;
    string a,command,what;

    bool flag_ls = false;
    for(int i = 1; i<= 1083; i++){
        cin>>a;
        if(a == "$"){
            cin >> command;
            if(command == "cd" ){
                cin >> what;
                if(what == ".."){
                    cwd = cwd->parent;
                    continue;
                }
                if(what == "/"){
                    cwd = root;
                    continue;
                }
                for(int j = 0; j<cwd->children.size();j++){
                    if(cwd->children[j]->name == what){
                        cwd = cwd->children[j];
                        break;
                    }
                }
            }

        }
        else{
            cin >>what;
            if(a == "dir"){
                bool xdd = false;
                for(int j = 0; j < cwd->children.size(); j++){
                    if(cwd->children[j]->name == what){
                        xdd = true;
                        break;
                    }
                    
                }
                if(xdd == false){
                    Directory* tmp = new Directory;
                    tmp->name = what;
                    tmp->parent = cwd;
                    cwd->children.push_back(tmp);  
                }
            }
            else{
                bool xdd = false;
                for(int j = 0; j < cwd->files.size(); j++){
                    if(cwd->files[j]->name == what){
                        xdd = true;
                        continue;
                    }
                }
                if(xdd == false){
                    File * tmp = new File;
                    tmp->name = what;
                    tmp->parent = cwd;
                    tmp->size = stoi(a);
                    cwd->files.push_back(tmp);
                }
            }
        }
    }
    DFS_sum(root);
    operating_space = operating_space - root->size;
    operating_space = 30000000 - operating_space;
    DFS(root,0);
    cout << sum_of_sums <<'\n';
    cout << to_delete <<'\n';
    delete root;
    return 0;
}