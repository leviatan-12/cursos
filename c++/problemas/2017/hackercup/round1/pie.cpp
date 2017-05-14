#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

int main(){
  int a,a1,b,c,d,e,f;
  vector<long> v;
  cin >> a;
  a1 = a;
  while (a--) {
  	v.clear();
    cin >> b >> c;
    d = b;
    while(d--) {
    	e = c;
      	while(e--){
      		cin >> f;
      		v.push_back(f);
      	}
    }
    sort(v.begin(),v.end());
    d = 0;
    e = 0;
    for(int i=0;i<b;i++) {
    	d += v[i]; //+ ((i+1)%c)*((i+1)%c);
    	//cout << v[i] + ((i+1)%c)*((i+1)%c) << endl;
    }

    cout << "Case #" << a1-a << ": " << d << endl;
  }
  return 0;
}
