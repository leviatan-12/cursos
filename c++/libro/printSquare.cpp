#include <iostream>

using namespace std;

double square(double x){
  return x*x;
}

void print_square(double x){
  cout<<"El cuadrado de "<<x<<" es "<<square(x)<<"\n";
}

int main(){
  print_square(2);
}
