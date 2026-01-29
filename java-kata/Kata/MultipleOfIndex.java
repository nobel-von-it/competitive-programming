package Kata;

import java.util.Arrays;

public class MultipleOfIndex {
    public static void main(String[] args) {
        multipleOfIndex(new int[]{22, -6, 32, 82, 9, 25});
    }
    public static int[] multipleOfIndex(int[] arr) {

        for (int i = 0; i < arr.length; i++){
            if (arr[i] == 0) {
                continue;
            }
            if (arr[i] % i != 0) {
                arr[i] = 1;
            }
        }
        arr = Arrays.stream(arr).filter(el -> el != 0).toArray();
        return arr;
    }
}
