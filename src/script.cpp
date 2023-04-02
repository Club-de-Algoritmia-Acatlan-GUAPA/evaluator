#include<iostream>
#include<vector>

using namespace std;

int main() {
    int n;
    cin>>n;
    while(n--) {
        int k;
        cin>>k;
        vector<int>arr(k);
        for(auto &x: arr)cin>>x;
        for(auto &x: arr)cout<<x<<" ";
        cout<<endl;
    }
}
