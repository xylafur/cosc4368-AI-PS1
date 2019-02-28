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
    return equation(point[0], point[1], point[2]);
}

double get_valid_value(double p, double r){
    double z = (((double)rand() / (double)RAND_MAX) * 2 * r) - r;

    while (p + z > 1 || p + z < 0){
        z = (((double)rand() / (double)RAND_MAX) * 2 * r) - r;
    }

    return p + z;
}

void generate_neighbor(double sp[3], double new[3] , double r){
    new[0] = get_valid_value(sp[0], r);
    new[1] = get_valid_value(sp[1], r);
    new[2] = get_valid_value(sp[2], r);
}

void generate_neighbors(double sp[3], double ** neighbors, int p, double r){
    assert(neighbors != 0);
    for(int ii = 0; ii < p; ii++){
        neighbors[ii] = malloc(3 * sizeof(double));
        generate_neighbor(sp, neighbors[ii], r);
    }
}

void print_neighbors(double ** neighbors, int num_neighbors){
    for(int ii = 0; ii < num_neighbors; ii++){
        printf("%d: f(%lf, %lf, %lf) = %lf\n", ii, neighbors[ii][0],
                                               neighbors[ii][1],
                                               neighbors[ii][2],
                                               eval(neighbors[ii]));
    }
}

void free_neighbors(double ** neighbors, int num_neighbors){
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
    for(int ii = 0; ii < num_neighbors; ii++){
        assert(neighbors[ii] != 0);
        if(eval(neighbors[ii]) > sp_val){
            return 1;
        }
    }
    return 0;
}

#define STATUS_UPDATE_COUTNER 10000000

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

        // this is just for those really long runs.. just to sanity check I
        // guess
        if(ii != 0 && ii % STATUS_UPDATE_COUTNER == 0){
            printf("Status update (iteration %lu):\n", ii);
            printf("    f(%lf, %lf, %lf) = %lf\n", current[0], current[1],
                                               current[2], eval(current));
        }

        ii++;

        generate_neighbors(current, neighbors, p, r);
        //print_neighbors(neighbors, p);

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
    printf("f(%f, %f, %f) = %f\n\n", current[0], current[1], current[2],
                                     eval(current));
}


int main(int argc, char * argv[]){
    if(argc == 6){
        double sp [3] = {atof(argv[1]), atof(argv[2]), atof(argv[3])};
        int p = atoi(argv[4]);
        double r = atof(argv[5]);

        for(int ii = 0; ii < 3 ; ii ++){
            printf("Iteration: %d\n", ii+1);
            int seed = (int)rand();

            time_t start = time(NULL);
            RHC(sp, p, r, seed);
            time_t end = time(NULL);
            printf("Took %d seconds to complete\n",
                   (unsigned int)(end - start));
        }

        return 0;

    }else if(argc != 1){
        printf("Invalid usage!\n");
        printf("Usage: %s <sp0 sp1 sp2> p r\n", argv[0]);

        return 1;
    }

    double sps [3][3] = {{0.5, 0.5, 0.5},
                        {0.0, 0.5, 1.0},
                        {0.9, 0.6, 0.3}};
    int ps [2] = {20, 100};
    double rs [2] = {0.02, 0.05};


    srand(time(NULL));
    for(int ii = 0; ii < 3; ii++){
        for(int jj = 0; jj < 2; jj++){
            for(int kk = 0; kk < 2; kk++){
                double sp [3];
                sp [0] = sps[ii][0];
                sp [1] = sps[ii][1];
                sp [2] = sps[ii][2];

                int p = ps[jj];
                double r = rs[kk];
                for(int ll = 0 ; ll < 3; ll++){
                    printf("Iteration: %d\n", ll + 1);
                    int seed = (int)rand();

                    time_t start = time(NULL);
                    RHC(sp, p, r, seed);
                    time_t end = time(NULL);
                    printf("Took %d seconds to complete\n",
                           (unsigned int)(end - start));
                }
            }
        }
    }
}
