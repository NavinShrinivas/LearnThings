#include<stdio.h>

int main(){
/*
 *    int lower,step,higher;
 *    int faren,cel;
 *    //first implmentation
 */
    float lower,step,higher,faren,cel;
    lower=0.0;
    higher=300.0;
    step=20.0;

    faren=lower;
    while(faren<=higher){
        /*cel=(5*(faren-32))/9;//first implmentation*/
        cel = (5.0/9.0)*(faren-32.0);
        /*printf("%3d %5d \n",faren,cel); //first implmentation*/
        printf("%3.3f %5.3f \n",faren,cel);
        faren+=step;
    }
}
