#include <iostream>
#include <cmath>
#define PI 3.14159265

using namespace std;

int main(){
  int a,b,c,d,e;
  double f,g;
  cin >> a;
  e=a;
  while(a--){
    cin >> b >> c >> d;
    c -= 50;
    d -= 50;
    if(sqrt(pow(c,2)+pow(d,2))>50){
      cout << "Case #" << e-a << ": white" << endl;
    }else if(b >0 && c ==0 && d == 0){
    	cout << "Case #" << e-a << ": black" << endl;
    } else{
    	f = (b*0.02*PI);
    	g = acos(d/sqrt(pow(c,2)+pow(d,2)));
    	f -= g;
    	if(f>0){
    		cout << "Case #" << e-a << ": black" << endl;
    	}else{
    		cout << "Case #" << e-a << ": white" << endl;
    	}

    }
  }
  return 0;
}
