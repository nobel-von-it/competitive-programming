package Kata;

import java.util.Arrays;

public class StarSort {
    public static String twoSort(String[] s) {
        Arrays.sort(s);
        return String.join("***", s[0].split(""));
    }
}
