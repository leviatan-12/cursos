#include <iostream>
#include <algorithm>
#include <vector>

using namespace std;

int main(){
  int a,b,c,d;
  cin >> a;
  b = a;
  while(a--){
    cin >> c;
    vector<int> v;
    while(c--){
      cin >> d;
      v.push_back(d);
    }
    sort(v.begin(),v.end());
    int e=0,f=1;
    while(!v.empty()){
    	if(f>(v.size()-1) && v[v.size()-1]*f<50){
    		break;
    	}else if(v[v.size()-1]*f>=50){
    		v.pop_back();
    		e++;
    		f--;
    		if(f==1){
   				v.erase(v.begin());
   			}else if(f>1){
   				v.erase(v.begin(),v.begin()+f);
   			}
   			f=1;
   		}else{
    		f++;
    	}
    }
   cout << "Case #" << b-a << ": " << e << endl;
  }
  return 0;
}
