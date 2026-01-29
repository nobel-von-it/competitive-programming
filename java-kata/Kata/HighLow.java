package Kata;

import java.util.Arrays;
public class HighLow {
    public static String highAndLow(String n) {
        int[] arr = Arrays.stream(n.split(" ")).mapToInt(Integer::parseInt).sorted().toArray();
        return String.format("%d %d", arr[arr.length-1], arr[0]);
    }
}
