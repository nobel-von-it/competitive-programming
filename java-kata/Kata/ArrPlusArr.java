package Kata;

import java.util.Arrays;
import java.util.stream.IntStream;

public class ArrPlusArr {
    public static int arrayPlusArray(int[] arr1, int[] arr2) {
        return Arrays.stream(arr1).reduce(Integer::sum).getAsInt() +
                Arrays.stream(arr2).reduce(Integer::sum).getAsInt();
    }
//    or (best practices)
    public static int arrPlusArr(int[] arr1, int[] arr2) {
        return IntStream.of(arr1).sum() + IntStream.of(arr2).sum();
    }
    // hello
}
