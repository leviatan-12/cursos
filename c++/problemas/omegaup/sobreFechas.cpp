#include <iostream>
using namespace std;

int main() {
	int a,b,c,d=0,e=0,f=0;
	cin >> a >> b >> c;
	b -= 1;
	d = (b>0)?(57*b)+(c%5==0)?1:2:0;
	e = ((c - (c/5) - 1600)*686)+(((c/5) - 401)*685);
	f = a + d + e;
	f = (f%7==0)?7:f%7;
	cout << f << endl;
	return 0;
}
