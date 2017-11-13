#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

int main(){
  long a,c,d;
  vector<long> b;
  bool esPal;
  for (long i = 999; i > 99; i--) {
    for (long j = 999; j > 99; j--) {
      esPal = true;
      a = i*j;
      c = 10;
      d = 100000;
      while(esPal){
        if(a/d>0 && a/d != a%c){
          esPal=false;
          break;
        }else if(a/d>0){
          c *= 10;
        }
        d /= 10;
      }
      if (esPal) {
        b.push_back(a);
      }
    }
  }
  sort(b.begin(),b.end());
  for (size_t i = 0; i < b.size(); i++) {
    cout << b[i] << endl;
  }
  return 0;
}
