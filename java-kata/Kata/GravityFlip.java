package Kata;

import java.util.Arrays;

public class GravityFlip {
    public static int[] flip(char dir, int[] arr) {
        Arrays.sort(arr);

        if (dir == 'L')
            for (int i = 0; i < arr.length / 2; i++) {
                int tmp = arr[i];
                arr[i] = arr[arr.length-1-i];
                arr[arr.length-1-i] = tmp;
            }
        System.out.println(Arrays.toString(arr));
        return arr;
    }
}
