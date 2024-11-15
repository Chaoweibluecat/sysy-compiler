

// int arr[2][3][4] = {1, 2, 3, 4, {5}, {6}, {7, 8}};
int arr1[1] = {1};

int get_first(int array[]) {
    return array[0];
}

int main() {
    int arr2[1] = {0};
    return get_first(arr1);
}


// int main() {
//     int a = 1;
//     return a;
// }