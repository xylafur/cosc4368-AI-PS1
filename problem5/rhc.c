#include <stdio.h>
#include <time.h>
#include <stdlib.h>
#include <math.h>
#include <assert.h>

double equation(double x, double y, double z){
    return fabs(x - y - 0.2) * fabs(x * z - 0.8) *
           fabs(0.3 - z * z * y) + (x * y * (1-z) * fabs(z - 0.5));
}

double eval(double point [3]){
    //printf("Evaling (%f, %f, %f)\n", point[0], point[1], point[2]);
    return equation(point[0], point[1], point[2]);
}

void generate_neighbor(double sp[3], double new[3] , double r){
    //printf("Generating Neighbor\n");
    double z1 = (((double)rand() / (double)RAND_MAX) * 2 * r) - r;
    double z2 = (((double)rand() / (double)RAND_MAX) * 2 * r) - r;
    double z3 = (((double)rand() / (double)RAND_MAX) * 2 * r) - r;

    //printf("sp = (%f, %f, %f)\n", sp[0], sp[1], sp[2]);
    new[0] = sp[0] + z1;
    new[1] = sp[2] + z2;
    new[2] = sp[2] + z3;
    //printf("new = (%f, %f, %f)\n", new[0], new[1], new[2]);
}

void generate_neighbors(double sp[3], double ** neighbors, int p, double r){
    assert(neighbors != 0);
    for(int ii = 0; ii < p; ii++){
        neighbors[ii] = malloc(3 * sizeof(double));
        generate_neighbor(sp, neighbors[ii], r);
    }
}

void free_neighbors(double ** neighbors, int num_neighbors){
    //printf("Freeing neighbors\n");
    for(int ii = 0; ii < num_neighbors; ii++){
        free(neighbors[ii]);
    }
}

void copy_best_neighbor(double new [3], double ** neighbors, int num_neighbors){
    int best_index = 0;
    double best_val = eval(neighbors[best_index]);

    for(int ii = 0; ii < num_neighbors; ii++){
        double this_val = eval(neighbors[ii]);
        if(this_val > best_val){
            best_val = this_val;
            best_index = ii;
        }
    }

    new[0] = neighbors[best_index][0];
    new[1] = neighbors[best_index][1];
    new[2] = neighbors[best_index][2];
}

int is_better_neighbor(double sp [3], double ** neighbors, int num_neighbors){
    assert(neighbors != 0);
    double sp_val = eval(sp);
    //printf("Evaled sp: %f\n", sp_val);
    for(int ii = 0; ii < num_neighbors; ii++){
        assert(neighbors[ii] != 0);
        if(eval(neighbors[ii]) > sp_val){
            return 1;
        }
    }
    return 0;
}

void RHC(double sp [3], int p, double r, int seed){
    unsigned long ii = 0;
    double current [3] = {sp[0], sp[1], sp[2]};

    printf("Evaluating with starting poition (%f, %f, %f)\n", sp[0], sp[1], sp[2]);
    printf("Using p=%d, r=%f\n", p, r);
    printf("Using seed %d\n", seed);

    srand(seed);

    while(1) {
        int better = 0;
        double ** neighbors = malloc(p * sizeof(double*));

        ii++;

        //printf("Current: (%f, %f, %f) = %f\n", current[0], current[1],
        //                                       current[2], eval(current));

        generate_neighbors(current, neighbors, p, r);

        if(is_better_neighbor(current, neighbors, p)){
            better = 1;
            copy_best_neighbor(current, neighbors, p);
        }
        free_neighbors(neighbors, p);

        free(neighbors);

        if(!better){
            break;
        }
    }
    printf("Took %lu iterations\n", ii);
    printf("f(%f, %f, %f) = %f\n", current[0], current[1], current[2], eval(current));
}


int main(){
    double sps [3][3] = {{0.5, 0.5, 0.5},
                        {0.0, 0.5, 1.0},
                        {0.9, 0.6, 0.3}};
    int ps [2] = {20, 100};
    double rs [2] = {0.02, 0.05};


    for(int ii = 0; ii < 3; ii++){
        for(int jj = 0; jj < 2; jj++){
            for(int kk = 0; kk < 2; kk++){
                double sp [3];
                sp [0] = sps[ii][0];
                sp [1] = sps[ii][1];
                sp [2] = sps[ii][2];

                int p = ps[jj];
                double r = rs[kk];

                srand(time(NULL));
                int seed = (int)rand();

                RHC(sp, p, r, seed);
            }
        }
    }


}
